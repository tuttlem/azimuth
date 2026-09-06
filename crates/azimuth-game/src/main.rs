mod aiming;
mod battlefield;
mod combat;
mod projectile;
mod tank;
mod turn;
mod world;

use std::f32::consts::FRAC_PI_2;
use std::time::{SystemTime, UNIX_EPOCH};

use aiming::{AimAdjustment, AimingState};
use battlefield::{BattlefieldTerrain, Crater, HALF_EXTENT, terrain_mesh_indices};
use bevy::{
    asset::RenderAssetUsages,
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use combat::resolve_explosion;
use projectile::{
    Gravity, Projectile, ProjectileAdvance, SimulationLimits, TerrainImpact, Wind,
    azimuth_from_horizontal_direction,
};
use tank::{
    HorizontalDirection, MAX_HEALTH, MovementDirection, MovementRejection, PlayerId, Tank,
    TankFiringRepresentation, initial_tanks,
};
use turn::{MatchState, TurnPhase, TurnState};
use world::{WorldPosition, WorldVector};

const CAMERA_MIN_DISTANCE: f32 = 8.0;
const CAMERA_MAX_DISTANCE: f32 = 60.0;
const CAMERA_ORBIT_SENSITIVITY: f32 = 0.005;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_PITCH_LIMIT: f32 = FRAC_PI_2 - 0.1;
const CAMERA_TRANSITION_SPEED: f32 = 5.0;
const ACTIVE_PLAYER_CAMERA_DISTANCE: f32 = 16.0;
const ACTIVE_PLAYER_CAMERA_HEIGHT: f32 = 1.8;
const ACTIVE_PLAYER_CAMERA_PITCH: f32 = -0.35;
const SHOT_CAMERA_DISTANCE: f32 = 38.0;
const SHOT_CAMERA_HEIGHT: f32 = 3.0;
const SHOT_CAMERA_PITCH: f32 = -0.6;
const AIM_REPEAT_DELAY_SECONDS: f32 = 0.3;
const AIM_REPEAT_INTERVAL_SECONDS: f32 = 0.1;
const PROJECTILE_FIXED_HZ: f64 = 120.0;
const DEVELOPMENT_GRAVITY: f32 = 8.0;
const MINIMUM_WIND_STRENGTH: f32 = 0.75;
const MAXIMUM_WIND_STRENGTH: f32 = 1.75;
const WIND_ENABLED: bool = true;
const EXPLOSION_VISUAL_DURATION_SECONDS: f32 = 0.6;
const EXPLOSION_INITIAL_SCALE: f32 = 0.35;
const EXPLOSION_MAXIMUM_SCALE: f32 = 5.0;

#[derive(Component)]
struct BattlefieldCamera {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
    presentation_intent: Option<CameraPresentationIntent>,
    desired_pose: CameraPose,
    tracked_aim_yaw: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CameraPose {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CameraPresentationIntent {
    ActivePlayer(PlayerId),
    WatchingShot,
}

#[derive(Clone, Copy, Debug)]
struct AimKeyRepeat {
    is_held: bool,
    seconds_until_repeat: f32,
}

impl Default for AimKeyRepeat {
    fn default() -> Self {
        Self {
            is_held: false,
            seconds_until_repeat: 0.0,
        }
    }
}

#[derive(Resource, Default)]
struct AimRepeatState {
    keys: [AimKeyRepeat; 6],
}

#[derive(Resource)]
struct Tanks([Tank; 2]);

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

/// Match-constant, projectile-only wind. Tank movement and settling intentionally consume no
/// wind because this is an artillery aiming variable, not vehicle physics.
#[derive(Resource)]
struct BattlefieldWind(Wind);

#[derive(Resource, Default)]
struct MovementFeedback(Option<MovementRejection>);

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
struct TankVisual(PlayerId);

#[derive(Component)]
struct TankBody(PlayerId);

#[derive(Component)]
struct TankBarrel(PlayerId);

#[derive(Component)]
struct TankMuzzle(PlayerId);

#[derive(Component)]
struct TacticalHud;

#[derive(Component)]
struct HudText(HudTextField);

#[derive(Component)]
struct HudHealthFill(PlayerId);

#[derive(Component)]
struct HudWindMarker;

#[derive(Component)]
struct HudActivePlayerPanel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HudTextField {
    Match,
    PlayerOneHealth,
    PlayerTwoHealth,
    Aim,
    Wind,
    Movement,
    Controls,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HudAction {
    Choose,
    Moving,
    Resolving,
    Finished,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TacticalHudView {
    active_player: Option<PlayerId>,
    action: HudAction,
    player_one_health: u8,
    player_two_health: u8,
    player_one_eliminated: bool,
    player_two_eliminated: bool,
    aim: Option<AimingState>,
    wind: Wind,
    movement: Option<(u8, Option<MovementRejection>)>,
    result: MatchState,
}

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
type HudDecorations<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Node,
        Option<&'static HudWindMarker>,
        Option<&'static mut BorderColor>,
        Option<&'static HudActivePlayerPanel>,
    ),
    Without<HudHealthFill>,
>;
type TankMuzzleTransforms<'w, 's> = Query<
    'w,
    's,
    (&'static TankMuzzle, &'static mut Transform),
    (Without<TankTurret>, Without<TankBarrel>),
>;

impl Default for BattlefieldCamera {
    fn default() -> Self {
        let pose = CameraPose {
            target: Vec3::ZERO,
            yaw: 0.0,
            pitch: -0.5,
            distance: 30.0,
        };
        Self {
            target: pose.target,
            yaw: pose.yaw,
            pitch: pose.pitch,
            distance: pose.distance,
            presentation_intent: None,
            desired_pose: pose,
            tracked_aim_yaw: None,
        }
    }
}

fn main() {
    let terrain = BattlefieldTerrain::initial();
    let tanks = initial_tanks(&terrain);
    let turn = initial_turn_state(tanks);
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Tanks(tanks))
        .insert_resource(CurrentTurn(turn))
        .insert_resource(BattlefieldState(terrain))
        .insert_resource(ProjectileFlight::default())
        .insert_resource(LatestTerrainImpact::default())
        .insert_resource(CurrentImpactExplosionConsumed::default())
        .insert_resource(MovementFeedback::default())
        .insert_resource(AimRepeatState::default())
        .insert_resource(BattlefieldGravity(
            Gravity::new(DEVELOPMENT_GRAVITY).expect("development gravity must be valid"),
        ))
        .insert_resource(BattlefieldWind(select_match_wind()))
        .insert_resource(Time::<Fixed>::from_hz(PROJECTILE_FIXED_HZ))
        .add_systems(Startup, spawn_battlefield_scene)
        .add_systems(
            Update,
            (
                update_battlefield_camera,
                select_movement_action,
                update_aiming_input,
                update_movement_input,
                sync_tank_pose,
                sync_tank_elimination,
                sync_tank_aim,
                launch_aimed_projectile,
                sync_tactical_hud,
                sync_projectile_visual,
                sync_impact_marker,
                sync_terrain_impact_explosion,
                update_explosion_visuals,
                sync_battlefield_mesh,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (advance_projectile, advance_tank_settling).chain(),
        )
        .run();
}

fn spawn_battlefield_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tanks: Res<Tanks>,
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

    spawn_tactical_hud(&mut commands);

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
    tanks: Res<Tanks>,
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

fn select_movement_action(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut feedback: ResMut<MovementFeedback>,
    mut turn: ResMut<CurrentTurn>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) && turn.0.begin_movement() {
        feedback.0 = None;
    }
}

fn update_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    terrain: Res<BattlefieldState>,
    camera: Single<&BattlefieldCamera>,
    mut tanks: ResMut<Tanks>,
    mut feedback: ResMut<MovementFeedback>,
    mut turn: ResMut<CurrentTurn>,
) {
    if turn.0.remaining_movement().is_none() {
        return;
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        if turn.0.finish_movement() {
            feedback.0 = None;
        }
        return;
    }
    let Some(direction) = movement_direction(&keyboard, camera.yaw) else {
        return;
    };
    let player = turn.0.current_player;
    let tank = tank_for_player(tanks.0, player);
    match tank.step_on_terrain(&terrain.0, direction) {
        Ok(moved) => {
            *tank_for_player_mut(&mut tanks.0, player) = moved;
            assert!(
                turn.0.accept_movement_step(),
                "moving turn must consume accepted step"
            );
            feedback.0 = None;
        }
        Err(rejection) => feedback.0 = Some(rejection),
    }
}

/// Maps screen-space arrows to the nearest existing cardinal world step. This retains deliberate
/// one-unit movement while making directions follow the current tactical camera orientation.
fn movement_direction(
    keyboard: &ButtonInput<KeyCode>,
    camera_yaw: f32,
) -> Option<MovementDirection> {
    let camera_forward = (-camera_yaw.sin(), -camera_yaw.cos());
    let camera_right = (camera_yaw.cos(), -camera_yaw.sin());
    let directions = [
        (KeyCode::ArrowUp, camera_forward),
        (KeyCode::ArrowLeft, (-camera_right.0, -camera_right.1)),
        (KeyCode::ArrowDown, (-camera_forward.0, -camera_forward.1)),
        (KeyCode::ArrowRight, camera_right),
    ];
    let mut requested = directions
        .into_iter()
        .filter_map(|(key, direction)| keyboard.just_pressed(key).then_some(direction));
    let horizontal_direction = requested.next()?;
    requested
        .next()
        .is_none()
        .then_some(nearest_cardinal_movement_direction(
            horizontal_direction.0,
            horizontal_direction.1,
        ))
}

fn nearest_cardinal_movement_direction(x: f32, z: f32) -> MovementDirection {
    if x.abs() >= z.abs() {
        if x >= 0.0 {
            MovementDirection::PositiveX
        } else {
            MovementDirection::NegativeX
        }
    } else if z >= 0.0 {
        MovementDirection::PositiveZ
    } else {
        MovementDirection::NegativeZ
    }
}

fn update_aiming_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    flight: Res<ProjectileFlight>,
    time: Res<Time>,
    mut repeat_state: ResMut<AimRepeatState>,
    mut turn: ResMut<CurrentTurn>,
) {
    if flight.0.is_some() || turn.0.phase != TurnPhase::Choosing {
        repeat_state.reset();
        return;
    }

    let coarse = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    for (adjustment, count) in aiming_adjustments(&keyboard, time.delta_secs(), &mut repeat_state) {
        for _ in 0..count {
            turn.0.apply_current_aim(adjustment, coarse);
        }
    }
}

impl AimRepeatState {
    fn reset(&mut self) {
        self.keys = [AimKeyRepeat::default(); 6];
    }
}

fn aiming_adjustments(
    keyboard: &ButtonInput<KeyCode>,
    delta_seconds: f32,
    repeat_state: &mut AimRepeatState,
) -> Vec<(AimAdjustment, u32)> {
    let pressed = [
        keyboard.pressed(KeyCode::ArrowLeft),
        keyboard.pressed(KeyCode::ArrowRight),
        keyboard.pressed(KeyCode::ArrowUp),
        keyboard.pressed(KeyCode::ArrowDown),
        keyboard.pressed(KeyCode::Minus),
        keyboard.pressed(KeyCode::Equal),
    ];
    let adjustments = [
        AimAdjustment::AzimuthDecrease,
        AimAdjustment::AzimuthIncrease,
        AimAdjustment::ElevationDecrease,
        AimAdjustment::ElevationIncrease,
        AimAdjustment::PowerDecrease,
        AimAdjustment::PowerIncrease,
    ];
    let mut counts = [0; 6];

    for (first, second) in [(0, 1), (2, 3), (4, 5)] {
        match (pressed[first], pressed[second]) {
            (true, false) => {
                repeat_state.keys[second] = AimKeyRepeat::default();
                counts[first] = repeat_count(&mut repeat_state.keys[first], delta_seconds);
            }
            (false, true) => {
                repeat_state.keys[first] = AimKeyRepeat::default();
                counts[second] = repeat_count(&mut repeat_state.keys[second], delta_seconds);
            }
            (false, false) | (true, true) => {
                repeat_state.keys[first] = AimKeyRepeat::default();
                repeat_state.keys[second] = AimKeyRepeat::default();
            }
        }
    }

    adjustments
        .into_iter()
        .zip(counts)
        .filter_map(|(adjustment, count)| (count > 0).then_some((adjustment, count)))
        .collect()
}

fn repeat_count(repeat: &mut AimKeyRepeat, delta_seconds: f32) -> u32 {
    if !repeat.is_held {
        repeat.is_held = true;
        repeat.seconds_until_repeat = AIM_REPEAT_DELAY_SECONDS;
        return 1;
    }

    repeat.seconds_until_repeat -= delta_seconds.max(0.0);
    if repeat.seconds_until_repeat > 0.0 {
        return 0;
    }

    let repeats = 1 + (-repeat.seconds_until_repeat / AIM_REPEAT_INTERVAL_SECONDS).floor() as u32;
    repeat.seconds_until_repeat += repeats as f32 * AIM_REPEAT_INTERVAL_SECONDS;
    repeats
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

fn tank_for_player_mut(tanks: &mut [Tank; 2], player: PlayerId) -> &mut Tank {
    tanks
        .iter_mut()
        .find(|tank| tank.owner == player)
        .expect("each current player must own one tank")
}

fn sync_tank_aim(
    tanks: Res<Tanks>,
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

fn sync_tank_pose(
    tanks: Res<Tanks>,
    mut roots: Query<(&TankVisual, &mut Transform), Without<TankBody>>,
    mut bodies: Query<(&TankBody, &mut Transform), Without<TankVisual>>,
) {
    for (owner, mut transform) in &mut roots {
        let position = tank_for_player(tanks.0, owner.0).pose.position;
        transform.translation = to_bevy_position(position);
    }
    for (owner, mut transform) in &mut bodies {
        let tank = tank_for_player(tanks.0, owner.0);
        *transform =
            direction_transform(tank.pose.body_forward).with_translation(Vec3::new(0.0, 0.4, 0.0));
    }
}

fn sync_tank_elimination(tanks: Res<Tanks>, mut visuals: Query<(&TankVisual, &mut Visibility)>) {
    for (owner, mut visibility) in &mut visuals {
        *visibility = if tank_for_player(tanks.0, owner.0).is_eliminated() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

fn spawn_tactical_hud(commands: &mut Commands) {
    commands
        .spawn((
            TacticalHud,
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            root.spawn((
                BackgroundColor(Color::srgba(0.03, 0.05, 0.08, 0.82)),
                BorderColor::all(Color::srgb(0.85, 0.25, 0.18)),
                HudActivePlayerPanel,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(16),
                    left: px(16),
                    width: px(245),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(2)),
                    row_gap: px(4),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ))
            .with_children(|p| {
                hud_text(p, HudTextField::Match, 22.0);
                hud_text(p, HudTextField::PlayerOneHealth, 16.0);
                hud_text(p, HudTextField::PlayerTwoHealth, 16.0);
                health_bar(p, PlayerId::One);
                health_bar(p, PlayerId::Two);
            });
            root.spawn((
                BackgroundColor(Color::srgba(0.03, 0.05, 0.08, 0.82)),
                BorderColor::all(Color::srgb(0.9, 0.75, 0.2)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(16),
                    right: px(16),
                    width: px(230),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(2)),
                    row_gap: px(4),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ))
            .with_children(|p| {
                hud_text(p, HudTextField::Aim, 17.0);
                hud_text(p, HudTextField::Wind, 16.0);
                p.spawn((
                    BackgroundColor(Color::srgba(0.08, 0.12, 0.16, 0.9)),
                    Node {
                        width: px(96),
                        height: px(76),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                ))
                .with_children(|plot| {
                    plot.spawn((
                        BackgroundColor(Color::srgba(0.7, 0.75, 0.8, 0.6)),
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(8),
                            right: px(8),
                            top: px(37),
                            height: px(1),
                            ..default()
                        },
                    ));
                    plot.spawn((
                        BackgroundColor(Color::srgba(0.7, 0.75, 0.8, 0.6)),
                        Node {
                            position_type: PositionType::Absolute,
                            top: px(8),
                            bottom: px(8),
                            left: px(47),
                            width: px(1),
                            ..default()
                        },
                    ));
                    plot.spawn((
                        Text::new("+Z      +X"),
                        TextFont {
                            font_size: 11.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            position_type: PositionType::Absolute,
                            top: px(1),
                            left: px(48),
                            ..default()
                        },
                    ));
                    plot.spawn((
                        HudWindMarker,
                        BackgroundColor(Color::srgb(0.95, 0.8, 0.2)),
                        Node {
                            position_type: PositionType::Absolute,
                            width: px(8),
                            height: px(8),
                            left: px(44),
                            top: px(33),
                            ..default()
                        },
                    ));
                });
            });
            root.spawn((
                BackgroundColor(Color::srgba(0.03, 0.05, 0.08, 0.82)),
                BorderColor::all(Color::srgb(0.4, 0.6, 0.8)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: px(16),
                    left: px(16),
                    width: px(300),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(2)),
                    row_gap: px(3),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ))
            .with_children(|p| {
                hud_text(p, HudTextField::Movement, 17.0);
                hud_text(p, HudTextField::Controls, 14.0);
            });
        });
}

fn hud_text(parent: &mut ChildSpawnerCommands, field: HudTextField, size: f32) {
    parent.spawn((
        HudText(field),
        Text::default(),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}
fn health_bar(parent: &mut ChildSpawnerCommands, player: PlayerId) {
    parent
        .spawn((
            BackgroundColor(Color::srgba(0.15, 0.18, 0.22, 0.95)),
            Node {
                width: percent(100),
                height: px(8),
                ..default()
            },
        ))
        .with_children(|bar| {
            bar.spawn((
                HudHealthFill(player),
                BackgroundColor(player_color(player)),
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
            ));
        });
}

fn tactical_hud_view(
    turn: TurnState,
    tanks: [Tank; 2],
    wind: Wind,
    feedback: Option<MovementRejection>,
) -> TacticalHudView {
    let action = if turn.match_state != MatchState::InProgress {
        HudAction::Finished
    } else {
        match turn.phase {
            TurnPhase::Choosing => HudAction::Choose,
            TurnPhase::Moving { .. } => HudAction::Moving,
            TurnPhase::ResolvingFire => HudAction::Resolving,
            TurnPhase::Finished => HudAction::Finished,
        }
    };
    TacticalHudView {
        active_player: (turn.match_state == MatchState::InProgress).then_some(turn.current_player),
        action,
        player_one_health: tanks[0].health,
        player_two_health: tanks[1].health,
        player_one_eliminated: tanks[0].is_eliminated(),
        player_two_eliminated: tanks[1].is_eliminated(),
        aim: (turn.match_state == MatchState::InProgress).then_some(turn.current_aim()),
        wind,
        movement: turn.remaining_movement().map(|steps| (steps, feedback)),
        result: turn.match_state,
    }
}

/// Presentation observes state only; no HUD path mutates gameplay or gates fixed simulation.
fn sync_tactical_hud(
    turn: Res<CurrentTurn>,
    tanks: Res<Tanks>,
    wind: Res<BattlefieldWind>,
    feedback: Res<MovementFeedback>,
    mut text: Query<(&HudText, &mut Text)>,
    mut fills: Query<(&HudHealthFill, &mut Node)>,
    mut decorations: HudDecorations,
) {
    let view = tactical_hud_view(turn.0, tanks.0, wind.0, feedback.0);
    for (field, mut value) in &mut text {
        value.0 = hud_field_text(field.0, view);
    }
    for (fill, mut node) in &mut fills {
        let health = if fill.0 == PlayerId::One {
            view.player_one_health
        } else {
            view.player_two_health
        };
        node.width = percent(health as f32 / MAX_HEALTH as f32 * 100.0);
    }
    let active_colour = view
        .active_player
        .map_or(Color::srgb(0.45, 0.45, 0.45), player_color);
    let (x, z) = wind_plot_offset(view.wind);
    for (mut node, wind_marker, border, active_panel) in &mut decorations {
        if wind_marker.is_some() {
            node.left = px(44.0 + x);
            node.top = px(33.0 - z);
        }
        if active_panel.is_some()
            && let Some(mut border) = border
        {
            border.top = active_colour;
            border.right = active_colour;
            border.bottom = active_colour;
            border.left = active_colour;
        }
    }
}

/// Maps world wind to the small HUD plot: +X is right and +Z is up.  The
/// normalisation deliberately represents direction while the adjacent number
/// represents strength; calm wind stays at the axis origin.
fn wind_plot_offset(wind: Wind) -> (f32, f32) {
    let vector = wind.horizontal_acceleration();
    let strength = wind.strength();
    if strength == 0.0 {
        (0.0, 0.0)
    } else {
        (vector.x / strength * 24.0, vector.z / strength * 24.0)
    }
}

fn hud_field_text(field: HudTextField, view: TacticalHudView) -> String {
    match field {
        HudTextField::Match => match view.result {
            MatchState::Winner(player) => format!("{} WINS", player_name(player)),
            MatchState::Draw => "DRAW".into(),
            MatchState::InProgress => format!(
                "{} - {}",
                player_name(view.active_player.unwrap()),
                match view.action {
                    HudAction::Choose => "CHOOSE ACTION",
                    HudAction::Moving => "MOVING",
                    HudAction::Resolving => "RESOLVING SHOT",
                    HudAction::Finished => "MATCH OVER",
                }
            ),
        },
        HudTextField::PlayerOneHealth => player_health_text(
            "PLAYER ONE",
            view.player_one_health,
            view.player_one_eliminated,
        ),
        HudTextField::PlayerTwoHealth => player_health_text(
            "PLAYER TWO",
            view.player_two_health,
            view.player_two_eliminated,
        ),
        HudTextField::Aim => view.aim.map_or_else(
            || "AIM LOCKED".into(),
            |a| {
                format!(
                    "AZ {:.0}\nEL {:.0}\nPOWER {:.1}",
                    a.azimuth_degrees, a.elevation_degrees, a.launch_speed
                )
            },
        ),
        HudTextField::Wind => {
            let a = view.wind.horizontal_acceleration();
            format!(
                "WIND {:.1}\nX {:+.1}  Z {:+.1}",
                view.wind.strength(),
                a.x,
                a.z
            )
        }
        HudTextField::Movement => view.movement.map_or_else(String::new, |(s, r)| {
            format!(
                "MOVE: {s} LEFT{}",
                match r {
                    Some(MovementRejection::Bounds) => " - EDGE",
                    Some(MovementRejection::Slope) => " - STEEP",
                    None => "",
                }
            )
        }),
        HudTextField::Controls => match view.action {
            HudAction::Choose => "M MOVE | SPACE FIRE\nARROWS AIM | -/= POWER".into(),
            HudAction::Moving => "ARROWS MOVE (CAMERA) | ENTER END".into(),
            HudAction::Resolving | HudAction::Finished => String::new(),
        },
    }
}
fn player_health_text(name: &str, health: u8, out: bool) -> String {
    if out {
        format!("{name}: OUT")
    } else {
        format!("{name}: {health}/{MAX_HEALTH}")
    }
}
fn player_color(player: PlayerId) -> Color {
    match player {
        PlayerId::One => Color::srgb(0.85, 0.25, 0.18),
        PlayerId::Two => Color::srgb(0.18, 0.4, 0.85),
    }
}

/// Selects one gentle, constant wind for the match. The sampled value becomes authoritative
/// state immediately; it never changes during flight or a player's turn.
fn select_match_wind() -> Wind {
    if !WIND_ENABLED {
        return Wind::new(WorldVector::ZERO).expect("calm wind must be valid");
    }

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64);
    wind_from_seed(seed)
}

/// Kept pure so a recorded seed recreates the same match condition in tests or diagnostics.
fn wind_from_seed(mut seed: u64) -> Wind {
    let direction_fraction = next_random_fraction(&mut seed);
    let strength_fraction = next_random_fraction(&mut seed);
    let angle = direction_fraction * std::f32::consts::TAU;
    let strength =
        MINIMUM_WIND_STRENGTH + (MAXIMUM_WIND_STRENGTH - MINIMUM_WIND_STRENGTH) * strength_fraction;
    Wind::new(WorldVector {
        x: angle.cos() * strength,
        y: 0.0,
        z: angle.sin() * strength,
    })
    .expect("sampled match wind must be horizontal and finite")
}

fn next_random_fraction(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    ((*seed >> 40) as f32) / ((1_u32 << 24) as f32)
}

fn player_name(player: PlayerId) -> &'static str {
    match player {
        PlayerId::One => "Player One",
        PlayerId::Two => "Player Two",
    }
}

fn advance_projectile(
    gravity: Res<BattlefieldGravity>,
    wind: Res<BattlefieldWind>,
    mut terrain: ResMut<BattlefieldState>,
    mut tanks: ResMut<Tanks>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
    mut turn: ResMut<CurrentTurn>,
) {
    let Some(mut projectile) = flight.0 else {
        return;
    };

    let advance = projectile.advance_with_terrain(
        gravity.0,
        wind.0,
        SimulationLimits::DEVELOPMENT,
        |x, z| terrain.0.height_if_within_bounds(x, z),
    );
    flight.0 = resolve_projectile_advance(
        projectile,
        advance,
        &mut terrain.0,
        &mut tanks.0,
        &mut latest_impact.0,
        &mut turn.0,
    );
}

fn resolve_projectile_advance(
    projectile: Projectile,
    advance: ProjectileAdvance,
    terrain: &mut BattlefieldTerrain,
    tanks: &mut [Tank; 2],
    latest_impact: &mut Option<TerrainImpact>,
    turn: &mut TurnState,
) -> Option<Projectile> {
    match advance {
        ProjectileAdvance::Active => Some(projectile),
        ProjectileAdvance::TerrainImpact(impact) => {
            resolve_explosion(tanks, impact.position);
            terrain.apply_crater(impact.position, Crater::default_development());
            *latest_impact = Some(impact);
            reconcile_living_tank_support(tanks, terrain);
            complete_resolution_if_settled(tanks, turn);
            None
        }
        ProjectileAdvance::OutOfBounds => {
            assert!(
                turn.complete_fire_resolution(survivors(tanks)),
                "only a resolving shot may terminate"
            );
            None
        }
    }
}

/// Authoritative settling is intentionally fixed-step and follows projectile advancement. A
/// camera transition or boom lifetime can observe this state but cannot make it progress.
fn advance_tank_settling(
    gravity: Res<BattlefieldGravity>,
    terrain: Res<BattlefieldState>,
    mut tanks: ResMut<Tanks>,
    mut turn: ResMut<CurrentTurn>,
) {
    if turn.0.phase != TurnPhase::ResolvingFire || !any_living_tank_is_settling(&tanks.0) {
        return;
    }

    for tank in &mut tanks.0 {
        if !tank.is_eliminated() {
            tank.advance_settling(&terrain.0, gravity.0);
        }
    }
    complete_resolution_if_settled(&tanks.0, &mut turn.0);
}

fn reconcile_living_tank_support(tanks: &mut [Tank; 2], terrain: &BattlefieldTerrain) {
    for tank in tanks {
        if !tank.is_eliminated() {
            tank.reconcile_support(terrain);
        }
    }
}

fn any_living_tank_is_settling(tanks: &[Tank; 2]) -> bool {
    tanks
        .iter()
        .any(|tank| !tank.is_eliminated() && tank.is_settling())
}

fn complete_resolution_if_settled(tanks: &[Tank; 2], turn: &mut TurnState) {
    if !any_living_tank_is_settling(tanks) {
        assert!(
            turn.complete_fire_resolution(survivors(tanks)),
            "only a resolving shot may terminate"
        );
    }
}

fn survivors(tanks: &[Tank; 2]) -> [bool; 2] {
    [!tanks[0].is_eliminated(), !tanks[1].is_eliminated()]
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
            TankVisual(tank.owner),
            Transform::from_xyz(position.x, position.y, position.z),
            Visibility::default(),
        ))
        .with_children(|tank_parent| {
            tank_parent.spawn((
                Mesh3d(meshes.body.clone()),
                MeshMaterial3d(material.clone()),
                direction_transform(tank.pose.body_forward)
                    .with_translation(Vec3::new(0.0, 0.4, 0.0)),
                TankBody(tank.owner),
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
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    time: Res<Time>,
    gameplay: (Res<Tanks>, Res<CurrentTurn>, Res<ProjectileFlight>),
    camera: Single<(&mut Transform, &mut BattlefieldCamera)>,
) {
    let (tanks, turn, flight) = gameplay;
    let (mut transform, mut controller) = camera.into_inner();
    let intent = camera_presentation_intent(turn.0, flight.0);
    if controller.presentation_intent != Some(intent) {
        controller.presentation_intent = Some(intent);
        controller.desired_pose = camera_pose_for_intent(intent, tanks.0, turn.0);
        controller.tracked_aim_yaw = active_aim_yaw(intent, turn.0);
    } else if let Some(aim_yaw) = active_aim_yaw(intent, turn.0) {
        if let Some(previous_aim_yaw) = controller.tracked_aim_yaw {
            controller.desired_pose.yaw += shortest_angle_delta(previous_aim_yaw, aim_yaw);
        }
        controller.tracked_aim_yaw = Some(aim_yaw);
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        // Mouse motion already represents the full movement since the prior frame.
        controller.yaw -= mouse_motion.delta.x * CAMERA_ORBIT_SENSITIVITY;
        controller.pitch = (controller.pitch - mouse_motion.delta.y * CAMERA_ORBIT_SENSITIVITY)
            .clamp(-CAMERA_PITCH_LIMIT, CAMERA_PITCH_LIMIT);
        controller.desired_pose.yaw = controller.yaw;
        controller.desired_pose.pitch = controller.pitch;
    }

    for wheel in mouse_wheel.read() {
        controller.distance = (controller.distance - wheel.y * CAMERA_ZOOM_SPEED)
            .clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
        controller.desired_pose.distance = controller.distance;
    }

    // Camera interpolation is presentation-only. No gameplay or fixed-update system reads this
    // controller, so a slow transition can never delay input, projectile resolution, or handoff.
    interpolate_camera_pose(&mut controller, time.delta_secs());

    *transform = camera_transform(&controller);
}

fn camera_presentation_intent(
    turn: TurnState,
    flight: Option<Projectile>,
) -> CameraPresentationIntent {
    if flight.is_some() {
        CameraPresentationIntent::WatchingShot
    } else {
        CameraPresentationIntent::ActivePlayer(turn.current_player)
    }
}

fn active_aim_yaw(intent: CameraPresentationIntent, turn: TurnState) -> Option<f32> {
    match intent {
        CameraPresentationIntent::ActivePlayer(player) => {
            Some(-turn.aim_for(player).azimuth_degrees.to_radians())
        }
        CameraPresentationIntent::WatchingShot => None,
    }
}

fn camera_pose_for_intent(
    intent: CameraPresentationIntent,
    tanks: [Tank; 2],
    turn: TurnState,
) -> CameraPose {
    match intent {
        CameraPresentationIntent::ActivePlayer(player) => {
            let tank = tank_for_player(tanks, player);
            CameraPose {
                target: clamp_camera_target(
                    to_bevy_position(tank.pose.position) + Vec3::Y * ACTIVE_PLAYER_CAMERA_HEIGHT,
                ),
                yaw: active_aim_yaw(intent, turn)
                    .expect("an active-player camera intent must have aiming yaw"),
                pitch: ACTIVE_PLAYER_CAMERA_PITCH,
                distance: ACTIVE_PLAYER_CAMERA_DISTANCE,
            }
        }
        CameraPresentationIntent::WatchingShot => CameraPose {
            target: Vec3::Y * SHOT_CAMERA_HEIGHT,
            yaw: 0.0,
            pitch: SHOT_CAMERA_PITCH,
            distance: SHOT_CAMERA_DISTANCE,
        },
    }
}

fn interpolate_camera_pose(camera: &mut BattlefieldCamera, delta_seconds: f32) {
    let factor = camera_transition_factor(delta_seconds);
    camera.target = camera.target.lerp(camera.desired_pose.target, factor);
    camera.yaw += shortest_angle_delta(camera.yaw, camera.desired_pose.yaw) * factor;
    camera.pitch = camera.pitch.lerp(camera.desired_pose.pitch, factor);
    camera.distance = camera.distance.lerp(camera.desired_pose.distance, factor);
}

fn camera_transition_factor(delta_seconds: f32) -> f32 {
    1.0 - (-CAMERA_TRANSITION_SPEED * delta_seconds.max(0.0)).exp()
}

fn shortest_angle_delta(from: f32, to: f32) -> f32 {
    (to - from + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
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
    fn directional_aiming_keys_select_expected_adjustments_and_opposites_cancel() {
        let mut keyboard = ButtonInput::default();
        let mut repeats = AimRepeatState::default();
        keyboard.press(KeyCode::ArrowLeft);
        keyboard.press(KeyCode::ArrowUp);
        keyboard.press(KeyCode::Equal);
        assert_eq!(
            aiming_adjustments(&keyboard, 0.0, &mut repeats),
            vec![
                (AimAdjustment::AzimuthDecrease, 1),
                (AimAdjustment::ElevationDecrease, 1),
                (AimAdjustment::PowerIncrease, 1),
            ]
        );

        keyboard = ButtonInput::default();
        repeats.reset();
        keyboard.press(KeyCode::ArrowLeft);
        keyboard.press(KeyCode::ArrowRight);
        assert!(aiming_adjustments(&keyboard, 0.0, &mut repeats).is_empty());
    }

    #[test]
    fn movement_mode_maps_arrows_to_camera_relative_cardinal_steps() {
        let cases = [
            (KeyCode::ArrowUp, MovementDirection::NegativeZ),
            (KeyCode::ArrowLeft, MovementDirection::NegativeX),
            (KeyCode::ArrowDown, MovementDirection::PositiveZ),
            (KeyCode::ArrowRight, MovementDirection::PositiveX),
        ];
        for (key, expected) in cases {
            let mut keyboard = ButtonInput::default();
            keyboard.press(key);
            assert_eq!(movement_direction(&keyboard, 0.0), Some(expected));
        }

        let mut rotated = ButtonInput::default();
        rotated.press(KeyCode::ArrowUp);
        assert_eq!(
            movement_direction(&rotated, std::f32::consts::FRAC_PI_2),
            Some(MovementDirection::NegativeX)
        );

        let mut retired = ButtonInput::default();
        retired.press(KeyCode::KeyI);
        assert_eq!(movement_direction(&retired, 0.0), None);
    }

    #[test]
    fn held_aiming_key_repeats_after_the_configured_delay_and_stops_on_release() {
        let mut keyboard = ButtonInput::default();
        let mut repeats = AimRepeatState::default();
        keyboard.press(KeyCode::ArrowRight);

        assert_eq!(
            aiming_adjustments(&keyboard, 0.0, &mut repeats),
            vec![(AimAdjustment::AzimuthIncrease, 1)]
        );
        assert!(
            aiming_adjustments(&keyboard, AIM_REPEAT_DELAY_SECONDS - 0.01, &mut repeats).is_empty()
        );
        assert_eq!(
            aiming_adjustments(&keyboard, 0.01, &mut repeats),
            vec![(AimAdjustment::AzimuthIncrease, 1)]
        );
        assert_eq!(
            aiming_adjustments(&keyboard, AIM_REPEAT_INTERVAL_SECONDS * 2.0, &mut repeats),
            vec![(AimAdjustment::AzimuthIncrease, 2)]
        );

        keyboard.release(KeyCode::ArrowRight);
        assert!(aiming_adjustments(&keyboard, 1.0, &mut repeats).is_empty());
        keyboard.press(KeyCode::ArrowRight);
        assert_eq!(
            aiming_adjustments(&keyboard, 0.0, &mut repeats),
            vec![(AimAdjustment::AzimuthIncrease, 1)]
        );
    }

    #[test]
    fn directional_repeat_remains_bounded_and_changes_only_the_current_player() {
        let terrain = BattlefieldTerrain::initial();
        let mut turn = initial_turn_state(initial_tanks(&terrain));
        let player_two_before = turn.aim_for(PlayerId::Two);
        let mut keyboard = ButtonInput::default();
        let mut repeats = AimRepeatState::default();
        keyboard.press(KeyCode::ArrowDown);

        for _ in 0..100 {
            for (adjustment, count) in aiming_adjustments(&keyboard, 1.0, &mut repeats) {
                for _ in 0..count {
                    turn.apply_current_aim(adjustment, true);
                }
            }
        }

        assert_eq!(
            turn.current_aim().elevation_degrees,
            aiming::MAX_ELEVATION_DEGREES
        );
        assert_eq!(turn.aim_for(PlayerId::Two), player_two_before);
    }

    #[test]
    fn tactical_hud_view_is_read_only_and_tracks_turn_specific_state() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let wind = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let choosing = initial_turn_state(tanks);
        let choosing_view = tactical_hud_view(choosing, tanks, wind, None);
        assert_eq!(choosing_view.active_player, Some(PlayerId::One));
        assert_eq!(choosing_view.action, HudAction::Choose);
        assert_eq!(choosing_view.player_one_health, MAX_HEALTH);
        assert_eq!(choosing_view.player_two_health, MAX_HEALTH);
        assert_eq!(choosing_view.aim, Some(choosing.current_aim()));
        assert_eq!(choosing_view.movement, None);

        let mut moving = choosing;
        assert!(moving.begin_movement());
        let moving_view = tactical_hud_view(moving, tanks, wind, Some(MovementRejection::Slope));
        assert_eq!(moving_view.action, HudAction::Moving);
        assert_eq!(
            moving_view.movement,
            Some((
                moving.remaining_movement().unwrap(),
                Some(MovementRejection::Slope)
            ))
        );

        let mut resolving = choosing;
        assert!(resolving.begin_fire().is_some());
        let resolving_view = tactical_hud_view(resolving, tanks, wind, None);
        assert_eq!(resolving_view.action, HudAction::Resolving);
        assert_eq!(resolving_view.aim, Some(resolving.current_aim()));
    }

    #[test]
    fn hud_wind_plot_uses_ascii_world_axes_and_keeps_calm_at_origin() {
        let calm = Wind::new(WorldVector::ZERO).unwrap();
        let positive_x = Wind::new(WorldVector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let negative_x = Wind::new(WorldVector {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let positive_z = Wind::new(WorldVector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        })
        .unwrap();
        let negative_z = Wind::new(WorldVector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        })
        .unwrap();

        assert_eq!(wind_plot_offset(calm), (0.0, 0.0));
        assert_eq!(wind_plot_offset(positive_x), (24.0, 0.0));
        assert_eq!(wind_plot_offset(negative_x), (-24.0, 0.0));
        assert_eq!(wind_plot_offset(positive_z), (0.0, 24.0));
        assert_eq!(wind_plot_offset(negative_z), (0.0, -24.0));
    }

    #[test]
    fn hud_text_hides_contextual_controls_after_actions_resolve() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let wind = Wind::new(WorldVector::ZERO).unwrap();
        let choosing = tactical_hud_view(initial_turn_state(tanks), tanks, wind, None);
        assert!(hud_field_text(HudTextField::Controls, choosing).contains("M MOVE"));

        let mut resolving_turn = initial_turn_state(tanks);
        assert!(resolving_turn.begin_fire().is_some());
        let resolving = tactical_hud_view(resolving_turn, tanks, wind, None);
        assert!(hud_field_text(HudTextField::Controls, resolving).is_empty());
    }

    #[test]
    fn match_wind_is_reproducible_from_a_seed_and_stays_gentle() {
        let first = wind_from_seed(42);
        let second = wind_from_seed(42);
        let different = wind_from_seed(43);

        assert_eq!(first, second);
        assert_ne!(first, different);
        assert!((MINIMUM_WIND_STRENGTH..=MAXIMUM_WIND_STRENGTH).contains(&first.strength()));
        assert_eq!(first.horizontal_acceleration().y, 0.0);
    }

    #[test]
    fn camera_intent_observes_turn_and_flight_without_owning_either() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let mut turn = initial_turn_state(tanks);
        assert_eq!(
            camera_presentation_intent(turn, None),
            CameraPresentationIntent::ActivePlayer(PlayerId::One)
        );

        let projectile = Projectile::launch(turn.current_aim().shot_parameters(WorldPosition {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        }));
        assert_eq!(
            camera_presentation_intent(turn, Some(projectile)),
            CameraPresentationIntent::WatchingShot
        );

        assert!(turn.begin_movement());
        assert!(turn.finish_movement());
        assert_eq!(
            camera_presentation_intent(turn, None),
            CameraPresentationIntent::ActivePlayer(PlayerId::Two)
        );
    }

    #[test]
    fn camera_pose_is_bounded_and_transition_does_not_change_turn_state() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let turn = initial_turn_state(tanks);
        let pose = camera_pose_for_intent(
            CameraPresentationIntent::ActivePlayer(PlayerId::One),
            tanks,
            turn,
        );
        assert!(pose.target.x.abs() <= HALF_EXTENT);
        assert!(pose.target.z.abs() <= HALF_EXTENT);
        assert!((CAMERA_MIN_DISTANCE..=CAMERA_MAX_DISTANCE).contains(&pose.distance));

        let mut camera = BattlefieldCamera {
            desired_pose: pose,
            ..default()
        };
        interpolate_camera_pose(&mut camera, 1.0);
        assert_eq!(turn.current_player, PlayerId::One);
        assert_eq!(turn.phase, TurnPhase::Choosing);
    }

    #[test]
    fn active_player_camera_yaw_follows_retained_barrel_aim_not_body_facing() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let mut turn = initial_turn_state(tanks);
        let intent = CameraPresentationIntent::ActivePlayer(PlayerId::One);
        let before = camera_pose_for_intent(intent, tanks, turn);
        turn.apply_current_aim(AimAdjustment::AzimuthIncrease, false);
        let after = camera_pose_for_intent(intent, tanks, turn);

        assert_ne!(before.yaw, after.yaw);
        assert!(
            (shortest_angle_delta(before.yaw, after.yaw) + aiming::FINE_ANGLE_DEGREES.to_radians())
                .abs()
                < 0.000_01
        );
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
    fn movement_keeps_aim_but_changes_the_later_launch_position() {
        let terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let mut turn = initial_turn_state(tanks);
        turn.apply_current_aim(AimAdjustment::AzimuthIncrease, false);
        let retained_aim = turn.current_aim();
        let before = launch_parameters_for_aim(tanks[0], retained_aim);

        let moved = tanks[0]
            .step_on_terrain(&terrain, MovementDirection::PositiveX)
            .unwrap();
        tanks[0] = moved;
        assert!(turn.begin_movement());
        assert!(turn.accept_movement_step());
        assert!(turn.finish_movement());
        assert!(turn.begin_fire().is_some());
        assert!(turn.complete_fire_resolution([true, true]));

        assert_eq!(turn.current_aim(), retained_aim);
        let after = launch_parameters_for_aim(tanks[0], turn.current_aim());
        assert_eq!(after.azimuth_degrees, before.azimuth_degrees);
        assert_eq!(after.elevation_degrees, before.elevation_degrees);
        assert_eq!(after.launch_speed, before.launch_speed);
        assert_ne!(after.launch_position, before.launch_position);
    }

    #[test]
    fn moving_one_players_tank_leaves_the_other_tank_unchanged() {
        let terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let player_two_before = tank_for_player(tanks, PlayerId::Two);
        let moved = tank_for_player(tanks, PlayerId::One)
            .step_on_terrain(&terrain, MovementDirection::PositiveX)
            .unwrap();

        *tank_for_player_mut(&mut tanks, PlayerId::One) = moved;

        assert_eq!(tank_for_player(tanks, PlayerId::One), moved);
        assert_eq!(tank_for_player(tanks, PlayerId::Two), player_two_before);
    }

    #[test]
    fn projectile_resolution_keeps_active_turn_then_applies_crater_before_handoff() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
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
            &mut tanks,
            &mut impact,
            &mut turn,
        );
        assert!(active.is_some());
        assert_eq!(turn.current_player, PlayerId::One);
        assert_eq!(turn.phase, TurnPhase::ResolvingFire);

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
            &mut tanks,
            &mut impact,
            &mut turn,
        );
        assert!(resolved.is_none());
        assert_eq!(impact.unwrap().position.y, before);
        assert!(terrain.height(0.0, 0.0) < before);
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Choosing);
    }

    #[test]
    fn wind_derived_terrain_impact_uses_the_existing_damage_crater_and_handoff_pipeline() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let mut latest_impact = None;
        let mut turn = initial_turn_state(tanks);
        let wind = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let mut projectile = Projectile::launch(AimingState::new(0.0, 60.0, 10.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 15.0,
                z: 0.0,
            },
        ));
        let impact = (0..2_400)
            .find_map(|_| {
                match projectile.advance_with_terrain(
                    Gravity::new(DEVELOPMENT_GRAVITY).unwrap(),
                    wind,
                    SimulationLimits::DEVELOPMENT,
                    |x, z| terrain.height_if_within_bounds(x, z),
                ) {
                    ProjectileAdvance::Active => None,
                    ProjectileAdvance::TerrainImpact(impact) => Some(impact),
                    ProjectileAdvance::OutOfBounds => {
                        panic!("development shot should impact terrain")
                    }
                }
            })
            .unwrap();
        let before_height = terrain.height(impact.position.x, impact.position.z);
        let health_before = tanks.map(|tank| tank.health);

        assert!(turn.begin_fire().is_some());
        assert!(
            resolve_projectile_advance(
                projectile,
                ProjectileAdvance::TerrainImpact(impact),
                &mut terrain,
                &mut tanks,
                &mut latest_impact,
                &mut turn,
            )
            .is_none()
        );

        assert_eq!(latest_impact, Some(impact));
        assert!(terrain.height(impact.position.x, impact.position.z) < before_height);
        assert!(
            tanks
                .iter()
                .zip(health_before)
                .all(|(tank, health)| tank.health <= health)
        );
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Choosing);
    }

    #[test]
    fn crater_beneath_a_living_tank_defers_handoff_until_it_settles() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let mut impact = None;
        let mut turn = initial_turn_state(tanks);
        let projectile = Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
        ));
        let centre = tanks[0].pose.position;

        assert!(turn.begin_fire().is_some());
        assert!(
            resolve_projectile_advance(
                projectile,
                ProjectileAdvance::TerrainImpact(TerrainImpact { position: centre }),
                &mut terrain,
                &mut tanks,
                &mut impact,
                &mut turn,
            )
            .is_none()
        );
        assert_eq!(tanks[0].health, 60);
        assert!(tanks[0].is_settling());
        assert_eq!(turn.phase, TurnPhase::ResolvingFire);
        assert_eq!(turn.current_player, PlayerId::One);
        assert!(!turn.begin_movement());
        assert!(turn.begin_fire().is_none());

        for _ in 0..200 {
            if !any_living_tank_is_settling(&tanks) {
                break;
            }
            for tank in &mut tanks {
                if !tank.is_eliminated() {
                    tank.advance_settling(&terrain, Gravity::new(DEVELOPMENT_GRAVITY).unwrap());
                }
            }
            complete_resolution_if_settled(&tanks, &mut turn);
        }
        assert!(!tanks[0].is_settling());
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Choosing);
        assert!(!turn.complete_fire_resolution(survivors(&tanks)));
    }

    #[test]
    fn zero_gravity_settling_keeps_the_resolving_turn_locked() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let mut impact = None;
        let mut turn = initial_turn_state(tanks);
        let projectile = Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
        ));
        let centre = tanks[0].pose.position;

        assert!(turn.begin_fire().is_some());
        resolve_projectile_advance(
            projectile,
            ProjectileAdvance::TerrainImpact(TerrainImpact { position: centre }),
            &mut terrain,
            &mut tanks,
            &mut impact,
            &mut turn,
        );
        let before = tanks[0];
        for tank in &mut tanks {
            if !tank.is_eliminated() {
                tank.advance_settling(&terrain, Gravity::new(0.0).unwrap());
            }
        }
        complete_resolution_if_settled(&tanks, &mut turn);

        assert_eq!(tanks[0], before);
        assert!(tanks[0].is_settling());
        assert_eq!(turn.phase, TurnPhase::ResolvingFire);
        assert_eq!(turn.current_player, PlayerId::One);
    }

    #[test]
    fn settled_pose_preserves_aim_and_changes_the_later_firing_origin() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let turn = initial_turn_state(tanks);
        let retained_aim = turn.current_aim();
        let before = launch_parameters_for_aim(tanks[0], retained_aim);

        terrain.apply_crater(tanks[0].pose.position, Crater::default_development());
        reconcile_living_tank_support(&mut tanks, &terrain);
        for _ in 0..200 {
            tanks[0].advance_settling(&terrain, Gravity::new(DEVELOPMENT_GRAVITY).unwrap());
        }
        let after = launch_parameters_for_aim(tanks[0], retained_aim);

        assert!(!tanks[0].is_settling());
        assert_eq!(turn.current_aim(), retained_aim);
        assert_eq!(after.azimuth_degrees, before.azimuth_degrees);
        assert_eq!(after.elevation_degrees, before.elevation_degrees);
        assert_eq!(after.launch_speed, before.launch_speed);
        assert_ne!(after.launch_position, before.launch_position);
        assert_eq!(
            tanks[0].pose.position.y,
            terrain.height(tanks[0].pose.position.x, tanks[0].pose.position.z)
        );
    }

    #[test]
    fn out_of_bounds_resolution_advances_once_without_impact_or_terrain_change() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
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
                &mut tanks,
                &mut impact,
                &mut turn,
            )
            .is_none()
        );
        assert!(impact.is_none());
        assert_eq!(terrain.height(0.0, 0.0), before);
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Choosing);
        assert!(!turn.complete_fire_resolution([true, true]));
    }
}
