mod aiming;
mod battlefield;
mod combat;
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
use combat::resolve_explosion;
use projectile::{
    Gravity, Projectile, ProjectileAdvance, SimulationLimits, TerrainImpact,
    azimuth_from_horizontal_direction,
};
use tank::{
    HorizontalDirection, MovementDirection, MovementRejection, PlayerId, Tank,
    TankFiringRepresentation, initial_tanks,
};
use turn::{MatchState, TurnPhase, TurnState};
use world::WorldPosition;

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

    commands.spawn((
        AimingHud,
        Text::new(format_aiming_hud(turn.0, tanks.0, None)),
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
    let Some(direction) = movement_direction(&keyboard) else {
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

fn movement_direction(keyboard: &ButtonInput<KeyCode>) -> Option<MovementDirection> {
    let directions = [
        (KeyCode::KeyI, MovementDirection::NegativeZ),
        (KeyCode::KeyJ, MovementDirection::NegativeX),
        (KeyCode::KeyK, MovementDirection::PositiveZ),
        (KeyCode::KeyL, MovementDirection::PositiveX),
    ];
    let mut requested = directions
        .into_iter()
        .filter_map(|(key, direction)| keyboard.just_pressed(key).then_some(direction));
    let direction = requested.next()?;
    requested.next().is_none().then_some(direction)
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
        AimAdjustment::ElevationIncrease,
        AimAdjustment::ElevationDecrease,
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

fn sync_aiming_hud(
    turn: Res<CurrentTurn>,
    tanks: Res<Tanks>,
    feedback: Res<MovementFeedback>,
    mut hud: Single<&mut Text, With<AimingHud>>,
) {
    hud.0 = format_aiming_hud(turn.0, tanks.0, feedback.0);
}

fn format_aiming_hud(
    turn: TurnState,
    tanks: [Tank; 2],
    feedback: Option<MovementRejection>,
) -> String {
    let health = |player| {
        let tank = tank_for_player(tanks, player);
        let state = if tank.is_eliminated() {
            " — ELIMINATED"
        } else {
            ""
        };
        format!("{}/100{state}", tank.health)
    };
    if let MatchState::Winner(player) = turn.match_state {
        return format!(
            "{} WINS\nPlayer One: {}\nPlayer Two: {}",
            player_name(player),
            health(PlayerId::One),
            health(PlayerId::Two)
        );
    }
    if turn.match_state == MatchState::Draw {
        return format!(
            "DRAW\nPlayer One: {}\nPlayer Two: {}",
            health(PlayerId::One),
            health(PlayerId::Two)
        );
    }
    let status = match turn.phase {
        TurnPhase::Choosing => {
            "CHOOSE ACTION — arrows aim, -/= power, hold to repeat, Shift coarse, Space fire, M move".to_owned()
        }
        TurnPhase::Moving { remaining_steps } => {
            let rejection = match feedback {
                Some(MovementRejection::Bounds) => " — blocked: battlefield edge",
                Some(MovementRejection::Slope) => " — blocked: terrain too steep",
                None => "",
            };
            format!("MOVING — {remaining_steps} steps left — I/J/K/L move, Enter finish{rejection}")
        }
        TurnPhase::ResolvingFire => "RESOLVING SHOT — action locked".to_owned(),
        TurnPhase::Finished => "MATCH FINISHED — action locked".to_owned(),
    };
    let player_name = player_name(turn.current_player);
    let aiming = turn.current_aim();
    format!(
        "{player_name}\nPlayer One health: {}\nPlayer Two health: {}\nAzimuth: {:.0}°\nElevation: {:.0}°\nPower: {:.1} units/s\n{status}",
        health(PlayerId::One),
        health(PlayerId::Two),
        aiming.azimuth_degrees,
        aiming.elevation_degrees,
        aiming.launch_speed,
    )
}

fn player_name(player: PlayerId) -> &'static str {
    match player {
        PlayerId::One => "Player One",
        PlayerId::Two => "Player Two",
    }
}

fn advance_projectile(
    gravity: Res<BattlefieldGravity>,
    mut terrain: ResMut<BattlefieldState>,
    mut tanks: ResMut<Tanks>,
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
                (AimAdjustment::ElevationIncrease, 1),
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
        keyboard.press(KeyCode::ArrowUp);

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
