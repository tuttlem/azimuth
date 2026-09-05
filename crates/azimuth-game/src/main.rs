mod aiming;
mod battlefield;
mod projectile;
mod tank;
mod turn;
mod world;

use std::f32::consts::FRAC_PI_2;

use aiming::{AimAdjustment, AimingState};
use battlefield::{BattlefieldTerrain, Crater, HALF_EXTENT, terrain_mesh_indices};
use bevy::{
    asset::RenderAssetUsages,
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use projectile::{
    Gravity, Projectile, ProjectileAdvance, SimulationLimits, TerrainImpact,
    azimuth_from_horizontal_direction,
};
use tank::{HorizontalDirection, PlayerId, Tank, TankFiringRepresentation, initial_tanks};
use turn::{TurnPhase, TurnState};
use world::WorldPosition;

const CAMERA_MIN_DISTANCE: f32 = 8.0;
const CAMERA_MAX_DISTANCE: f32 = 60.0;
const CAMERA_PAN_SPEED: f32 = 12.0;
const CAMERA_ORBIT_SENSITIVITY: f32 = 0.005;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_PITCH_LIMIT: f32 = FRAC_PI_2 - 0.1;
const PROJECTILE_FIXED_HZ: f64 = 120.0;
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

#[derive(Resource)]
struct CurrentTurn(TurnState);

#[derive(Resource)]
struct BattlefieldState(BattlefieldTerrain);

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
struct BattlefieldVisual;

#[derive(Component)]
struct ExplosionVisual {
    elapsed_seconds: f32,
}

#[derive(Component)]
struct TankTurret(PlayerId);

#[derive(Component)]
struct TankBarrel(PlayerId);

#[derive(Component)]
struct TankMuzzle(PlayerId);

#[derive(Component)]
struct AimingHud;

type TankTurretTransforms<'w, 's> = Query<
    'w,
    's,
    (&'static TankTurret, &'static mut Transform),
    (Without<TankBarrel>, Without<TankMuzzle>),
>;
type TankBarrelTransforms<'w, 's> = Query<
    'w,
    's,
    (&'static TankBarrel, &'static mut Transform),
    (Without<TankTurret>, Without<TankMuzzle>),
>;
type TankMuzzleTransforms<'w, 's> = Query<
    'w,
    's,
    (&'static TankMuzzle, &'static mut Transform),
    (Without<TankTurret>, Without<TankBarrel>),
>;

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
    let terrain = BattlefieldTerrain::initial();
    let tanks = initial_tanks(&terrain);
    let turn = initial_turn_state(tanks);
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(InitialTanks(tanks))
        .insert_resource(CurrentTurn(turn))
        .insert_resource(BattlefieldState(terrain))
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
                update_aiming_input,
                sync_tank_aim,
                launch_aimed_projectile,
                sync_aiming_hud,
                sync_projectile_visual,
                sync_impact_marker,
                sync_terrain_impact_explosion,
                update_explosion_visuals,
                sync_battlefield_mesh,
                draw_world_axes,
            )
                .chain(),
        )
        .add_systems(FixedUpdate, advance_projectile)
        .run();
}

fn spawn_battlefield_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tanks: Res<InitialTanks>,
    turn: Res<CurrentTurn>,
    terrain: Res<BattlefieldState>,
) {
    let camera = BattlefieldCamera::default();
    let transform = camera_transform(&camera);
    commands.spawn((Camera3d::default(), camera, transform));

    commands.spawn((
        BattlefieldVisual,
        Mesh3d(meshes.add(create_battlefield_mesh(&terrain.0))),
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
            active_firing_representation(tank, turn.0.aim_for(tank.owner)),
        );
    }

    commands.spawn((
        AimingHud,
        Text::new(format_aiming_hud(
            turn.0.current_player,
            turn.0.current_aim(),
            turn.0.phase,
        )),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
    ));

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

fn launch_aimed_projectile(
    keyboard: Res<ButtonInput<KeyCode>>,
    tanks: Res<InitialTanks>,
    assets: Res<ProjectileVisualAssets>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
    mut turn: ResMut<CurrentTurn>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::Space) || flight.0.is_some() {
        return;
    }

    let Some((player, aiming)) = turn.0.begin_fire() else {
        return;
    };
    let tank = tank_for_player(tanks.0, player);
    let parameters = launch_parameters_for_aim(tank, aiming);
    let projectile = Projectile::launch(parameters);

    commands.spawn((
        Name::new("Aimed projectile"),
        ProjectileVisual,
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform::from_translation(to_bevy_position(projectile.position)),
    ));
    flight.0 = Some(projectile);
    latest_impact.0 = None;
}

fn update_aiming_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    flight: Res<ProjectileFlight>,
    mut turn: ResMut<CurrentTurn>,
) {
    if flight.0.is_some() || turn.0.phase != TurnPhase::Ready {
        return;
    }

    let coarse = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    for adjustment in aiming_adjustments(&keyboard) {
        turn.0.apply_current_aim(adjustment, coarse);
    }
}

fn aiming_adjustments(keyboard: &ButtonInput<KeyCode>) -> Vec<AimAdjustment> {
    [
        key_pair_adjustment(
            keyboard,
            KeyCode::KeyQ,
            KeyCode::KeyE,
            AimAdjustment::AzimuthDecrease,
            AimAdjustment::AzimuthIncrease,
        ),
        key_pair_adjustment(
            keyboard,
            KeyCode::KeyF,
            KeyCode::KeyR,
            AimAdjustment::ElevationDecrease,
            AimAdjustment::ElevationIncrease,
        ),
        key_pair_adjustment(
            keyboard,
            KeyCode::KeyG,
            KeyCode::KeyT,
            AimAdjustment::PowerDecrease,
            AimAdjustment::PowerIncrease,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn key_pair_adjustment(
    keyboard: &ButtonInput<KeyCode>,
    decrease: KeyCode,
    increase: KeyCode,
    decrease_adjustment: AimAdjustment,
    increase_adjustment: AimAdjustment,
) -> Option<AimAdjustment> {
    match (
        keyboard.just_pressed(decrease),
        keyboard.just_pressed(increase),
    ) {
        (true, false) => Some(decrease_adjustment),
        (false, true) => Some(increase_adjustment),
        _ => None,
    }
}

fn active_firing_representation(tank: Tank, aiming: AimingState) -> TankFiringRepresentation {
    let direction = projectile::ShotParameters::direction_for_angles(
        aiming.azimuth_degrees,
        aiming.elevation_degrees,
    )
    .expect("validated aiming state must have a valid canonical direction");
    tank.firing_representation(direction)
}

fn launch_parameters_for_aim(tank: Tank, aiming: AimingState) -> projectile::ShotParameters {
    let firing = active_firing_representation(tank, aiming);
    aiming.shot_parameters(firing.muzzle_position)
}

fn initial_turn_state(tanks: [Tank; 2]) -> TurnState {
    TurnState::new(
        initial_aim_for_tank(tank_for_player(tanks, PlayerId::One)),
        initial_aim_for_tank(tank_for_player(tanks, PlayerId::Two)),
    )
}

fn initial_aim_for_tank(tank: Tank) -> AimingState {
    let azimuth =
        azimuth_from_horizontal_direction(tank.pose.turret_forward.x, tank.pose.turret_forward.z)
            .expect("initial tank turret direction must be valid");
    AimingState::new(azimuth, 45.0, 18.0)
}

fn tank_for_player(tanks: [Tank; 2], player: PlayerId) -> Tank {
    tanks
        .into_iter()
        .find(|tank| tank.owner == player)
        .expect("each current player must own one initial tank")
}

fn sync_tank_aim(
    tanks: Res<InitialTanks>,
    turn: Res<CurrentTurn>,
    mut turrets: TankTurretTransforms,
    mut barrels: TankBarrelTransforms,
    mut muzzles: TankMuzzleTransforms,
) {
    for (owner, mut transform) in &mut turrets {
        let tank = tank_for_player(tanks.0, owner.0);
        let firing = active_firing_representation(tank, turn.0.aim_for(owner.0));
        *transform =
            direction_transform(firing.turret_forward).with_translation(Vec3::new(0.0, 1.0, 0.0));
    }
    for (owner, mut transform) in &mut barrels {
        let aim = turn.0.aim_for(owner.0);
        let pitch = Quat::from_rotation_x(aim.elevation_degrees.to_radians());
        *transform = Transform {
            translation: pitch * Vec3::NEG_Z * 1.35,
            rotation: pitch,
            ..default()
        };
    }
    for (owner, mut transform) in &mut muzzles {
        let tank = tank_for_player(tanks.0, owner.0);
        let firing = active_firing_representation(tank, turn.0.aim_for(owner.0));
        *transform = Transform::from_translation(Vec3::new(
            firing.muzzle_position.x - tank.pose.position.x,
            firing.muzzle_position.y - tank.pose.position.y,
            firing.muzzle_position.z - tank.pose.position.z,
        ));
    }
}

fn sync_aiming_hud(turn: Res<CurrentTurn>, mut hud: Single<&mut Text, With<AimingHud>>) {
    hud.0 = format_aiming_hud(turn.0.current_player, turn.0.current_aim(), turn.0.phase);
}

fn format_aiming_hud(player: PlayerId, aiming: AimingState, phase: TurnPhase) -> String {
    let status = match phase {
        TurnPhase::Ready => {
            "READY — Q/E azimuth, R/F elevation, T/G power, Shift coarse, Space fire"
        }
        TurnPhase::Resolving => "RESOLVING SHOT — aiming locked",
    };
    let player_name = match player {
        PlayerId::One => "Player One",
        PlayerId::Two => "Player Two",
    };
    format!(
        "{player_name}\nAzimuth: {:.0}°\nElevation: {:.0}°\nPower: {:.1} units/s\n{status}",
        aiming.azimuth_degrees, aiming.elevation_degrees, aiming.launch_speed,
    )
}

fn advance_projectile(
    gravity: Res<BattlefieldGravity>,
    mut terrain: ResMut<BattlefieldState>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
    mut turn: ResMut<CurrentTurn>,
) {
    let Some(mut projectile) = flight.0 else {
        return;
    };

    let advance =
        projectile.advance_with_terrain(gravity.0, SimulationLimits::DEVELOPMENT, |x, z| {
            terrain.0.height_if_within_bounds(x, z)
        });
    flight.0 = resolve_projectile_advance(
        projectile,
        advance,
        &mut terrain.0,
        &mut latest_impact.0,
        &mut turn.0,
    );
}

fn resolve_projectile_advance(
    projectile: Projectile,
    advance: ProjectileAdvance,
    terrain: &mut BattlefieldTerrain,
    latest_impact: &mut Option<TerrainImpact>,
    turn: &mut TurnState,
) -> Option<Projectile> {
    match advance {
        ProjectileAdvance::Active => Some(projectile),
        ProjectileAdvance::TerrainImpact(impact) => {
            terrain.apply_crater(impact.position, Crater::default_development());
            *latest_impact = Some(impact);
            assert!(
                turn.complete_resolution(),
                "only a resolving shot may terminate"
            );
            None
        }
        ProjectileAdvance::OutOfBounds => {
            assert!(
                turn.complete_resolution(),
                "only a resolving shot may terminate"
            );
            None
        }
    }
}

fn sync_battlefield_mesh(
    terrain: Res<BattlefieldState>,
    mut meshes: ResMut<Assets<Mesh>>,
    visuals: Query<&Mesh3d, With<BattlefieldVisual>>,
) {
    if !terrain.is_changed() {
        return;
    }
    for mesh_handle in &visuals {
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = create_battlefield_mesh(&terrain.0);
        }
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
    firing: TankFiringRepresentation,
) {
    let position = tank.pose.position;
    let firing_origin = firing.muzzle_position;
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

            let mut turret_entity = tank_parent.spawn((
                Mesh3d(meshes.turret.clone()),
                MeshMaterial3d(material.clone()),
                direction_transform(tank.pose.turret_forward)
                    .with_translation(Vec3::new(0.0, 1.0, 0.0)),
                TankTurret(tank.owner),
            ));
            turret_entity.with_children(|turret| {
                turret.spawn((
                    Mesh3d(meshes.barrel.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(0.0, 0.0, -1.35),
                    TankBarrel(tank.owner),
                ));
            });

            tank_parent.spawn((
                Name::new("Firing origin"),
                Mesh3d(meshes.firing_origin_marker.clone()),
                MeshMaterial3d(firing_origin_material.clone()),
                Transform::from_xyz(
                    firing_origin.x - position.x,
                    firing_origin.y - position.y,
                    firing_origin.z - position.z,
                ),
                TankMuzzle(tank.owner),
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

fn create_battlefield_mesh(terrain: &BattlefieldTerrain) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, terrain.mesh_positions())
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

    #[test]
    fn aiming_keys_select_expected_adjustments_and_opposites_cancel() {
        let mut keyboard = ButtonInput::default();
        keyboard.press(KeyCode::KeyQ);
        keyboard.press(KeyCode::KeyR);
        keyboard.press(KeyCode::KeyT);
        assert_eq!(
            aiming_adjustments(&keyboard),
            vec![
                AimAdjustment::AzimuthDecrease,
                AimAdjustment::ElevationIncrease,
                AimAdjustment::PowerIncrease,
            ]
        );

        keyboard = ButtonInput::default();
        keyboard.press(KeyCode::KeyQ);
        keyboard.press(KeyCode::KeyE);
        assert!(aiming_adjustments(&keyboard).is_empty());
    }

    #[test]
    fn current_aim_controls_initial_projectile_conditions() {
        let terrain = BattlefieldTerrain::initial();
        let tank = initial_tanks(&terrain)[0];
        let baseline = AimingState::new(90.0, 30.0, 12.0);
        let same = launch_parameters_for_aim(tank, baseline);
        let changed_azimuth = launch_parameters_for_aim(tank, AimingState::new(100.0, 30.0, 12.0));
        let raised = launch_parameters_for_aim(tank, AimingState::new(90.0, 40.0, 12.0));
        let stronger = launch_parameters_for_aim(tank, AimingState::new(90.0, 30.0, 18.0));

        assert_eq!(same, launch_parameters_for_aim(tank, baseline));
        assert_ne!(
            same.launch_direction().x,
            changed_azimuth.launch_direction().x
        );
        assert!(raised.launch_velocity().y > same.launch_velocity().y);
        assert!(stronger.launch_velocity().x.abs() > same.launch_velocity().x.abs());
        assert_eq!(same.launch_speed, 12.0);
    }

    #[test]
    fn each_player_launches_from_their_own_tank_and_retained_aim() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let turn = initial_turn_state(tanks);
        let player_one = launch_parameters_for_aim(
            tank_for_player(tanks, PlayerId::One),
            turn.aim_for(PlayerId::One),
        );
        let player_two = launch_parameters_for_aim(
            tank_for_player(tanks, PlayerId::Two),
            turn.aim_for(PlayerId::Two),
        );

        assert_ne!(player_one.launch_position, player_two.launch_position);
        assert_ne!(player_one.azimuth_degrees, player_two.azimuth_degrees);
    }

    #[test]
    fn projectile_resolution_keeps_active_turn_then_applies_crater_before_handoff() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut impact = None;
        let mut turn = TurnState::new(
            AimingState::new(0.0, 45.0, 18.0),
            AimingState::new(180.0, 45.0, 18.0),
        );
        let projectile = Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
        ));

        turn.begin_fire();
        let active = resolve_projectile_advance(
            projectile,
            ProjectileAdvance::Active,
            &mut terrain,
            &mut impact,
            &mut turn,
        );
        assert!(active.is_some());
        assert_eq!(turn.current_player, PlayerId::One);
        assert_eq!(turn.phase, TurnPhase::Resolving);

        let before = terrain.height(0.0, 0.0);
        let resolved = resolve_projectile_advance(
            projectile,
            ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: WorldPosition {
                    x: 0.0,
                    y: before,
                    z: 0.0,
                },
            }),
            &mut terrain,
            &mut impact,
            &mut turn,
        );
        assert!(resolved.is_none());
        assert_eq!(impact.unwrap().position.y, before);
        assert!(terrain.height(0.0, 0.0) < before);
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Ready);
    }

    #[test]
    fn out_of_bounds_resolution_advances_once_without_impact_or_terrain_change() {
        let mut terrain = BattlefieldTerrain::initial();
        let before = terrain.height(0.0, 0.0);
        let mut impact = None;
        let mut turn = TurnState::new(
            AimingState::new(0.0, 45.0, 18.0),
            AimingState::new(180.0, 45.0, 18.0),
        );
        let projectile = Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
        ));

        turn.begin_fire();
        assert!(
            resolve_projectile_advance(
                projectile,
                ProjectileAdvance::OutOfBounds,
                &mut terrain,
                &mut impact,
                &mut turn,
            )
            .is_none()
        );
        assert!(impact.is_none());
        assert_eq!(terrain.height(0.0, 0.0), before);
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Ready);
        assert!(!turn.complete_resolution());
    }
}
