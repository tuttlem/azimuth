mod battlefield;
mod projectile;
mod tank;
mod world;

use std::f32::consts::FRAC_PI_2;

use battlefield::{
    HALF_EXTENT, terrain_height_if_within_bounds, terrain_mesh_indices, terrain_mesh_positions,
};
use bevy::{
    asset::RenderAssetUsages,
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use projectile::{
    Gravity, Projectile, ProjectileAdvance, ShotParameters, SimulationLimits, TerrainImpact,
    azimuth_from_horizontal_direction,
};
use tank::{HorizontalDirection, PlayerId, Tank, initial_tanks};
use world::WorldPosition;

const CAMERA_MIN_DISTANCE: f32 = 8.0;
const CAMERA_MAX_DISTANCE: f32 = 60.0;
const CAMERA_PAN_SPEED: f32 = 12.0;
const CAMERA_ORBIT_SENSITIVITY: f32 = 0.005;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_PITCH_LIMIT: f32 = FRAC_PI_2 - 0.1;
const PROJECTILE_FIXED_HZ: f64 = 120.0;
const DEVELOPMENT_ELEVATION_DEGREES: f32 = 45.0;
const DEVELOPMENT_LAUNCH_SPEED: f32 = 18.0;
const DEVELOPMENT_IMPACT_LAUNCH_SPEED: f32 = 14.0;
const DEVELOPMENT_GRAVITY: f32 = 8.0;
const EXPLOSION_VISUAL_DURATION_SECONDS: f32 = 0.6;
const EXPLOSION_INITIAL_SCALE: f32 = 0.35;
const EXPLOSION_MAXIMUM_SCALE: f32 = 5.0;

#[derive(Component)]
struct BattlefieldCamera {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

#[derive(Resource)]
struct InitialTanks([Tank; 2]);

#[derive(Resource, Default)]
struct ProjectileFlight(Option<Projectile>);

#[derive(Resource, Default)]
struct LatestTerrainImpact(Option<TerrainImpact>);

/// Records whether the persistent latest impact has already created its one presentation effect.
/// The impact remains available for the diagnostic marker until the next shot clears it.
#[derive(Resource, Default)]
struct CurrentImpactExplosionConsumed(bool);

#[derive(Resource)]
struct BattlefieldGravity(Gravity);

#[derive(Resource)]
struct ProjectileVisualAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct ImpactMarkerAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct ExplosionVisualAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct ProjectileVisual;

#[derive(Component)]
struct ImpactMarker;

#[derive(Component)]
struct ExplosionVisual {
    elapsed_seconds: f32,
}

impl Default for BattlefieldCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            yaw: 0.0,
            pitch: -0.5,
            distance: 30.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(InitialTanks(initial_tanks()))
        .insert_resource(ProjectileFlight::default())
        .insert_resource(LatestTerrainImpact::default())
        .insert_resource(CurrentImpactExplosionConsumed::default())
        .insert_resource(BattlefieldGravity(
            Gravity::new(DEVELOPMENT_GRAVITY).expect("development gravity must be valid"),
        ))
        .insert_resource(Time::<Fixed>::from_hz(PROJECTILE_FIXED_HZ))
        .add_systems(Startup, spawn_battlefield_scene)
        .add_systems(
            Update,
            (
                update_battlefield_camera,
                launch_development_projectile,
                sync_projectile_visual,
                sync_impact_marker,
                sync_terrain_impact_explosion,
                update_explosion_visuals,
                draw_world_axes,
            ),
        )
        .add_systems(FixedUpdate, advance_projectile)
        .run();
}

fn spawn_battlefield_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tanks: Res<InitialTanks>,
) {
    let camera = BattlefieldCamera::default();
    let transform = camera_transform(&camera);
    commands.spawn((Camera3d::default(), camera, transform));

    commands.spawn((
        Mesh3d(meshes.add(create_battlefield_mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.42, 0.2))),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.6, 0.0)),
    ));

    let tank_meshes = TankMeshes {
        body: meshes.add(Cuboid::new(1.8, 0.8, 2.4)),
        turret: meshes.add(Cuboid::new(1.2, 0.45, 1.2)),
        barrel: meshes.add(Cuboid::new(0.18, 0.18, 1.5)),
        firing_origin_marker: meshes.add(Sphere::new(0.12)),
    };
    let player_one_material = materials.add(Color::srgb(0.85, 0.25, 0.18));
    let player_two_material = materials.add(Color::srgb(0.18, 0.4, 0.85));
    let firing_origin_material = materials.add(Color::srgb(0.95, 0.85, 0.2));

    for tank in tanks.0.iter().copied() {
        let material = match tank.owner {
            PlayerId::One => player_one_material.clone(),
            PlayerId::Two => player_two_material.clone(),
        };
        spawn_tank(
            &mut commands,
            tank,
            &tank_meshes,
            material,
            firing_origin_material.clone(),
        );
    }

    commands.insert_resource(ProjectileVisualAssets {
        mesh: meshes.add(Sphere::new(0.28)),
        material: materials.add(Color::srgb(1.0, 0.92, 0.35)),
    });
    commands.insert_resource(ImpactMarkerAssets {
        mesh: meshes.add(Sphere::new(0.18)),
        material: materials.add(Color::srgb(1.0, 0.25, 0.1)),
    });
    commands.insert_resource(ExplosionVisualAssets {
        mesh: meshes.add(Sphere::new(0.5)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.55, 0.05),
            emissive: Color::srgb(4.0, 1.2, 0.05).into(),
            ..default()
        }),
    });
}

fn launch_development_projectile(
    keyboard: Res<ButtonInput<KeyCode>>,
    tanks: Res<InitialTanks>,
    assets: Res<ProjectileVisualAssets>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
    mut commands: Commands,
) {
    let launch_speed = if keyboard.just_pressed(KeyCode::Space) {
        DEVELOPMENT_LAUNCH_SPEED
    } else if keyboard.just_pressed(KeyCode::KeyI) {
        DEVELOPMENT_IMPACT_LAUNCH_SPEED
    } else {
        return;
    };

    if flight.0.is_some() {
        return;
    }

    let tank = tanks.0[0];
    let azimuth =
        azimuth_from_horizontal_direction(tank.pose.turret_forward.x, tank.pose.turret_forward.z)
            .expect("initial tank turret direction must be valid");
    let parameters = ShotParameters::new(
        tank.firing_origin(),
        azimuth,
        DEVELOPMENT_ELEVATION_DEGREES,
        launch_speed,
    )
    .expect("development shot parameters must be valid");
    let projectile = Projectile::launch(parameters);

    commands.spawn((
        Name::new("Development projectile"),
        ProjectileVisual,
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform::from_translation(to_bevy_position(projectile.position)),
    ));
    flight.0 = Some(projectile);
    latest_impact.0 = None;
}

fn advance_projectile(
    gravity: Res<BattlefieldGravity>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
) {
    let Some(mut projectile) = flight.0 else {
        return;
    };

    match projectile.advance_with_terrain(
        gravity.0,
        SimulationLimits::DEVELOPMENT,
        terrain_height_if_within_bounds,
    ) {
        ProjectileAdvance::Active => flight.0 = Some(projectile),
        ProjectileAdvance::TerrainImpact(impact) => {
            latest_impact.0 = Some(impact);
            flight.0 = None;
        }
        ProjectileAdvance::OutOfBounds => flight.0 = None,
    }
}

fn sync_projectile_visual(
    flight: Res<ProjectileFlight>,
    mut commands: Commands,
    mut visuals: Query<(Entity, &mut Transform), With<ProjectileVisual>>,
) {
    if let Some(projectile) = flight.0 {
        for (_, mut transform) in &mut visuals {
            transform.translation = to_bevy_position(projectile.position);
        }
    } else {
        for (entity, _) in &mut visuals {
            commands.entity(entity).despawn();
        }
    }
}

fn sync_impact_marker(
    latest_impact: Res<LatestTerrainImpact>,
    assets: Res<ImpactMarkerAssets>,
    mut commands: Commands,
    mut markers: Query<(Entity, &mut Transform), With<ImpactMarker>>,
) {
    if let Some(impact) = latest_impact.0 {
        if let Some((_, mut transform)) = markers.iter_mut().next() {
            transform.translation = to_bevy_position(impact.position);
        } else {
            commands.spawn((
                Name::new("Terrain impact marker"),
                ImpactMarker,
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.material.clone()),
                Transform::from_translation(to_bevy_position(impact.position)),
            ));
        }
    } else {
        for (entity, _) in &mut markers {
            commands.entity(entity).despawn();
        }
    }
}

fn sync_terrain_impact_explosion(
    latest_impact: Res<LatestTerrainImpact>,
    assets: Res<ExplosionVisualAssets>,
    mut consumed: ResMut<CurrentImpactExplosionConsumed>,
    mut commands: Commands,
) {
    let Some(impact) = latest_impact.0 else {
        consumed.0 = false;
        return;
    };

    if !should_spawn_explosion(true, consumed.0) {
        return;
    }

    commands.spawn((
        Name::new("Terrain impact explosion"),
        ExplosionVisual {
            elapsed_seconds: 0.0,
        },
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        explosion_transform(impact.position),
    ));
    consumed.0 = true;
}

fn update_explosion_visuals(
    time: Res<Time>,
    mut commands: Commands,
    mut explosions: Query<(Entity, &mut ExplosionVisual, &mut Transform)>,
) {
    for (entity, mut explosion, mut transform) in &mut explosions {
        explosion.elapsed_seconds += time.delta_secs();
        if explosion_has_expired(explosion.elapsed_seconds) {
            commands.entity(entity).despawn();
        } else {
            transform.scale = Vec3::splat(explosion_scale(explosion_progress(
                explosion.elapsed_seconds,
            )));
        }
    }
}

fn should_spawn_explosion(has_terrain_impact: bool, impact_already_consumed: bool) -> bool {
    has_terrain_impact && !impact_already_consumed
}

fn explosion_progress(elapsed_seconds: f32) -> f32 {
    (elapsed_seconds / EXPLOSION_VISUAL_DURATION_SECONDS).clamp(0.0, 1.0)
}

fn explosion_scale(progress: f32) -> f32 {
    EXPLOSION_INITIAL_SCALE + (EXPLOSION_MAXIMUM_SCALE - EXPLOSION_INITIAL_SCALE) * progress
}

fn explosion_has_expired(elapsed_seconds: f32) -> bool {
    elapsed_seconds >= EXPLOSION_VISUAL_DURATION_SECONDS
}

fn explosion_transform(impact_position: WorldPosition) -> Transform {
    Transform::from_translation(to_bevy_position(impact_position))
        .with_scale(Vec3::splat(EXPLOSION_INITIAL_SCALE))
}

fn to_bevy_position(position: WorldPosition) -> Vec3 {
    Vec3::new(position.x, position.y, position.z)
}

struct TankMeshes {
    body: Handle<Mesh>,
    turret: Handle<Mesh>,
    barrel: Handle<Mesh>,
    firing_origin_marker: Handle<Mesh>,
}

fn spawn_tank(
    commands: &mut Commands,
    tank: Tank,
    meshes: &TankMeshes,
    material: Handle<StandardMaterial>,
    firing_origin_material: Handle<StandardMaterial>,
) {
    let position = tank.pose.position;
    let firing_origin = tank.firing_origin();
    let player_name = match tank.owner {
        PlayerId::One => "Player one tank",
        PlayerId::Two => "Player two tank",
    };

    commands
        .spawn((
            Name::new(player_name),
            Transform::from_xyz(position.x, position.y, position.z),
            Visibility::default(),
        ))
        .with_children(|tank_parent| {
            tank_parent.spawn((
                Mesh3d(meshes.body.clone()),
                MeshMaterial3d(material.clone()),
                direction_transform(tank.pose.body_forward)
                    .with_translation(Vec3::new(0.0, 0.4, 0.0)),
            ));

            tank_parent
                .spawn((
                    Mesh3d(meshes.turret.clone()),
                    MeshMaterial3d(material.clone()),
                    direction_transform(tank.pose.turret_forward)
                        .with_translation(Vec3::new(0.0, 1.0, 0.0)),
                ))
                .with_children(|turret| {
                    turret.spawn((
                        Mesh3d(meshes.barrel.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(0.0, 0.0, -1.35),
                    ));
                });

            // This marks the derived handoff point for the next projectile feature. It is not a
            // projectile and does not add firing behaviour to this scene.
            tank_parent.spawn((
                Name::new("Firing origin"),
                Mesh3d(meshes.firing_origin_marker.clone()),
                MeshMaterial3d(firing_origin_material.clone()),
                Transform::from_xyz(
                    firing_origin.x - position.x,
                    firing_origin.y - position.y,
                    firing_origin.z - position.z,
                ),
            ));
        });
}

fn direction_transform(direction: HorizontalDirection) -> Transform {
    Transform::IDENTITY.looking_to(Vec3::new(direction.x, 0.0, direction.z), Vec3::Y)
}

fn update_battlefield_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    time: Res<Time>,
    camera: Single<(&mut Transform, &mut BattlefieldCamera)>,
) {
    let (mut transform, mut controller) = camera.into_inner();

    if mouse_buttons.pressed(MouseButton::Right) {
        // Mouse motion already represents the full movement since the prior frame.
        controller.yaw -= mouse_motion.delta.x * CAMERA_ORBIT_SENSITIVITY;
        controller.pitch = (controller.pitch - mouse_motion.delta.y * CAMERA_ORBIT_SENSITIVITY)
            .clamp(-CAMERA_PITCH_LIMIT, CAMERA_PITCH_LIMIT);
    }

    for wheel in mouse_wheel.read() {
        controller.distance = (controller.distance - wheel.y * CAMERA_ZOOM_SPEED)
            .clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
    }

    let mut pan = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        pan.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        pan.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        pan.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        pan.y += 1.0;
    }

    if pan != Vec2::ZERO {
        let pan = pan.normalize() * CAMERA_PAN_SPEED * time.delta_secs();
        controller.target = clamp_camera_target(controller.target + Vec3::new(pan.x, 0.0, pan.y));
    }

    *transform = camera_transform(&controller);
}

fn camera_transform(camera: &BattlefieldCamera) -> Transform {
    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let position = camera.target - rotation * Vec3::NEG_Z * camera.distance;

    Transform {
        translation: position,
        rotation,
        ..default()
    }
}

fn clamp_camera_target(target: Vec3) -> Vec3 {
    Vec3::new(
        target.x.clamp(-HALF_EXTENT, HALF_EXTENT),
        target.y,
        target.z.clamp(-HALF_EXTENT, HALF_EXTENT),
    )
}

fn create_battlefield_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, terrain_mesh_positions())
    .with_inserted_indices(Indices::U32(terrain_mesh_indices()))
    .with_computed_smooth_normals()
}

fn draw_world_axes(mut gizmos: Gizmos) {
    gizmos.axes(Transform::IDENTITY, 3.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explosion_lifetime_progress_and_scale_are_explicit_and_bounded() {
        assert_eq!(explosion_progress(0.0), 0.0);
        assert_eq!(
            explosion_scale(explosion_progress(0.0)),
            EXPLOSION_INITIAL_SCALE
        );

        let halfway = explosion_progress(EXPLOSION_VISUAL_DURATION_SECONDS / 2.0);
        assert_eq!(halfway, 0.5);
        assert_eq!(
            explosion_scale(halfway),
            (EXPLOSION_INITIAL_SCALE + EXPLOSION_MAXIMUM_SCALE) / 2.0
        );

        assert_eq!(explosion_progress(EXPLOSION_VISUAL_DURATION_SECONDS), 1.0);
        assert_eq!(
            explosion_scale(explosion_progress(EXPLOSION_VISUAL_DURATION_SECONDS)),
            EXPLOSION_MAXIMUM_SCALE
        );
        assert_eq!(
            explosion_progress(EXPLOSION_VISUAL_DURATION_SECONDS * 2.0),
            1.0
        );
    }

    #[test]
    fn explosion_starts_at_the_authoritative_impact_position() {
        let impact_position = WorldPosition {
            x: 3.25,
            y: 1.5,
            z: -4.75,
        };

        let transform = explosion_transform(impact_position);
        assert_eq!(transform.translation, to_bevy_position(impact_position));
        assert_eq!(transform.scale, Vec3::splat(EXPLOSION_INITIAL_SCALE));
    }

    #[test]
    fn explosion_expiry_begins_at_its_configured_duration() {
        assert!(!explosion_has_expired(
            EXPLOSION_VISUAL_DURATION_SECONDS - f32::EPSILON
        ));
        assert!(explosion_has_expired(EXPLOSION_VISUAL_DURATION_SECONDS));
    }

    #[test]
    fn persistent_impact_is_consumed_once_without_marker_state() {
        assert!(should_spawn_explosion(true, false));
        assert!(!should_spawn_explosion(true, true));
        assert!(!should_spawn_explosion(false, false));
    }

    #[test]
    fn clearing_an_impact_rearms_an_identical_later_impact() {
        let consumed_first_impact = should_spawn_explosion(true, false);
        assert!(consumed_first_impact);

        let consumed_after_launch_clears_impact = false;
        assert!(should_spawn_explosion(
            true,
            consumed_after_launch_clears_impact
        ));
    }

    #[test]
    fn battlefield_camera_target_stays_within_visible_bounds() {
        assert_eq!(
            clamp_camera_target(Vec3::new(30.0, 0.0, -30.0)),
            Vec3::new(HALF_EXTENT, 0.0, -HALF_EXTENT)
        );
    }
}
