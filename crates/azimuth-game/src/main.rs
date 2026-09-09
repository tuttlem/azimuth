mod ai;
mod aiming;
mod battlefield;
mod combat;
mod match_setup;
mod projectile;
mod tank;
mod turn;
mod weapon;
mod world;

use std::time::{SystemTime, UNIX_EPOCH};

use ai::decide_firing;
use aiming::{AimAdjustment, AimingState};
use battlefield::{
    BattlefieldSeed, BattlefieldTerrain, BuildingPlacement, HALF_EXTENT, VisualHorizon,
    WATER_TABLE, generate_buildings, terrain_mesh_indices,
};
use bevy::{
    audio::{AudioPlayer, AudioSource, PlaybackSettings, SpatialListener, Volume},
    asset::RenderAssetUsages,
    camera::{Viewport, visibility::RenderLayers},
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use combat::resolve_explosion;
use match_setup::{ControllerType, MatchConfiguration, PlayerConfiguration};
use projectile::{
    Gravity, Projectile, ProjectileAdvance, SimulationLimits, TerrainImpact, Wind,
    azimuth_from_horizontal_direction,
};
use tank::{
    HorizontalDirection, MAX_HEALTH, MovementDirection, MovementRejection, PlayerId, Tank,
    TankFiringRepresentation, initial_tanks_for_players_seeded,
};
use turn::{MatchState, TurnPhase, TurnState};
use weapon::{FiredShot, PlayerWeaponLoadouts, WeaponAvailability, WeaponId, weapon_definition};
use world::{WorldPosition, WorldVector};

const CAMERA_MIN_DISTANCE: f32 = 8.0;
const CAMERA_MAX_DISTANCE: f32 = 150.0;
const CAMERA_ORBIT_SENSITIVITY: f32 = 0.005;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_MIN_PITCH: f32 = -std::f32::consts::FRAC_PI_2 + 0.1;
// Normal tactical poses already look down. Keeping the upper limit below horizontal avoids views
// from beneath the single-sided battlefield mesh without changing camera controls or intent.
const CAMERA_MAX_PITCH: f32 = -0.08;
const CAMERA_TRANSITION_SPEED: f32 = 5.0;
const ACTIVE_PLAYER_CAMERA_DISTANCE: f32 = 16.0;
const ACTIVE_PLAYER_CAMERA_HEIGHT: f32 = 1.8;
const ACTIVE_PLAYER_CAMERA_PITCH: f32 = -0.35;
const SHOT_CAMERA_PITCH: f32 = -0.6;
const HUMAN_SHOT_DISTANCE: f32 = 22.0;
const HUMAN_APEX_DISTANCE: f32 = 46.0;
const HUMAN_SHOT_HEIGHT: f32 = 5.0;
const HUMAN_LOOK_AHEAD: f32 = 5.0;
const AI_TACTICAL_DISTANCE: f32 = 82.0;
const AI_TACTICAL_HEIGHT: f32 = 12.0;
const IMPACT_CAMERA_DISTANCE: f32 = 27.0;
const IMPACT_CAMERA_HEIGHT: f32 = 3.0;
const IMPACT_VIEW_HOLD_SECONDS: f32 = 0.9;
const AIM_REPEAT_DELAY_SECONDS: f32 = 0.3;
const AIM_REPEAT_INTERVAL_SECONDS: f32 = 0.1;
const TURRET_DINK_INTERVAL_SECONDS: f32 = 0.07;
const PROJECTILE_FIXED_HZ: f64 = 120.0;
const DEVELOPMENT_GRAVITY: f32 = 8.0;
const MINIMUM_WIND_STRENGTH: f32 = 0.75;
const MAXIMUM_WIND_STRENGTH: f32 = 1.75;
/// The simulation's wind value is an acceleration.  The HUD presents the same relative
/// intensity on a familiar, player-facing kilometres-per-hour scale.
const WIND_KPH_PER_ACCELERATION: f32 = 10.0;
const WIND_INDICATOR_RENDER_LAYER: usize = 1;
const WIND_INDICATOR_WIDTH: u32 = 150;
const WIND_INDICATOR_HEIGHT: u32 = 52;
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum CameraPresentationIntent {
    ActivePlayer(PlayerId),
    HumanShotFollow(Projectile),
    AiTacticalShot {
        shooter: PlayerId,
        projectile: Projectile,
    },
    Impact(WorldPosition),
    Result(WorldPosition),
}

/// Presentation records who fired once at launch, rather than guessing from input or a display
/// name after authoritative turn resolution has already advanced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShotPresentationMode {
    HumanFollow,
    AiTactical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ShotPresentationPhase {
    PlayerView,
    Flight {
        mode: ShotPresentationMode,
        shooter: PlayerId,
        apex_seen: bool,
    },
    Impact {
        position: WorldPosition,
        elapsed_seconds: f32,
    },
    Result {
        position: WorldPosition,
    },
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
struct ShotPresentation {
    phase: ShotPresentationPhase,
    seen_impact: Option<TerrainImpact>,
}

impl Default for ShotPresentation {
    fn default() -> Self {
        Self {
            phase: ShotPresentationPhase::PlayerView,
            seen_impact: None,
        }
    }
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
struct Tanks(Vec<Tank>);

#[derive(Resource)]
struct CurrentTurn(TurnState);

#[derive(Resource)]
struct BattlefieldState(BattlefieldTerrain);

#[derive(Resource, Clone, Copy)]
struct MatchSeed(BattlefieldSeed);

#[derive(Resource)]
struct AiDecisionSeed(u64);

#[derive(Resource, Default)]
struct WorldDressing(Vec<BuildingPlacement>);

#[derive(Resource, Default)]
struct ProjectileFlight(Option<FiredShot>);

#[derive(Resource, Default)]
struct WeaponState(PlayerWeaponLoadouts);

#[derive(Resource)]
struct LatestTerrainImpact {
    impact: Option<TerrainImpact>,
    explosion_visual_scale: f32,
}

impl Default for LatestTerrainImpact {
    fn default() -> Self {
        Self {
            impact: None,
            explosion_visual_scale: 1.0,
        }
    }
}

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

/// The existing tactical scene is prepared behind setup so the start boundary is explicit:
/// configuration is chosen before any human action can mutate authoritative match state.
#[derive(Resource, Default)]
struct MatchSetupGate {
    started: bool,
    selected_slot: usize,
}

#[derive(Resource)]
struct PendingMatchConfiguration(MatchConfiguration);

#[derive(Component)]
struct MatchSetupOverlay;
#[derive(Component)]
struct MatchSetupDetails;

#[derive(Resource)]
struct ProjectileVisualAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct AudioAssets {
    fire: Handle<AudioSource>,
    flight: Handle<AudioSource>,
    impact: Handle<AudioSource>,
    wind: Handle<AudioSource>,
    turret_dink: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct TurretDinkCooldown(f32);

#[derive(Component)]
struct FlightAudio;

#[derive(Component)]
struct WindAudio;

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

#[derive(Resource)]
struct WorldDressingAssets {
    material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct ProjectileVisual;

#[derive(Component)]
struct ImpactMarker;

#[derive(Component)]
struct BattlefieldVisual;

#[derive(Component)]
struct HorizonVisual;

#[derive(Component)]
struct WaterVisual;

#[derive(Component)]
struct BuildingVisual;

#[derive(Component)]
struct ExplosionVisual {
    elapsed_seconds: f32,
    scale_multiplier: f32,
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
struct HudScoreboardText(PlayerId);

#[derive(Component)]
struct HudScoreboardRow(PlayerId);

#[derive(Component)]
struct WindIndicatorCamera;

#[derive(Component)]
struct WindIndicatorArrow;

#[derive(Component)]
struct HudActivePlayerPanel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HudTextField {
    Match,
    Aim,
    Weapon,
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

#[derive(Clone, Debug, PartialEq)]
struct TacticalHudView {
    active_player: Option<PlayerId>,
    action: HudAction,
    scoreboard: Vec<ScoreboardEntry>,
    aim: Option<AimingState>,
    weapon: Option<(WeaponId, WeaponAvailability)>,
    wind: Wind,
    movement: Option<(u8, Option<MovementRejection>)>,
    result: MatchState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScoreboardEntry {
    player: PlayerId,
    display_name: String,
    /// A setup screen can contain newly configured slots before their tanks are generated.
    /// Keeping this optional makes the scoreboard presentation follow configuration without
    /// pretending those slots are eliminated gameplay entities.
    health: Option<u8>,
    eliminated: bool,
    active: bool,
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
        Option<&'static mut BorderColor>,
        Option<&'static HudActivePlayerPanel>,
    ),
    (Without<HudHealthFill>, Without<HudScoreboardRow>),
>;
type TankMuzzleTransforms<'w, 's> = Query<
    'w,
    's,
    (&'static TankMuzzle, &'static mut Transform),
    (Without<TankTurret>, Without<TankBarrel>),
>;
type BattlefieldSceneResources<'w> = (
    Res<'w, Tanks>,
    Res<'w, CurrentTurn>,
    Res<'w, BattlefieldState>,
    Res<'w, WorldDressing>,
    Res<'w, MatchSeed>,
    Res<'w, PendingMatchConfiguration>,
);
type MatchSetupWorldResources<'w> = (
    ResMut<'w, BattlefieldState>,
    Res<'w, MatchSeed>,
    ResMut<'w, WorldDressing>,
    ResMut<'w, BattlefieldWind>,
    ResMut<'w, AiDecisionSeed>,
);

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
    let configuration = MatchConfiguration::default();
    let player_ids = configuration
        .players
        .iter()
        .map(|player| player.id)
        .collect::<Vec<_>>();
    let match_seed = MatchSeed(BattlefieldSeed(select_match_seed()));
    let (terrain, tanks, dressing, wind) = generate_match_world(match_seed.0, &player_ids);
    let turn = initial_turn_state(&tanks);
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(sky_colour()))
        .insert_resource(Tanks(tanks))
        .insert_resource(CurrentTurn(turn))
        .insert_resource(BattlefieldState(terrain))
        .insert_resource(match_seed)
        .insert_resource(AiDecisionSeed(derived_seed(match_seed.0, "ai")))
        .insert_resource(WorldDressing(dressing))
        .insert_resource(ProjectileFlight::default())
        .insert_resource(ShotPresentation::default())
        .insert_resource(WeaponState::default())
        .insert_resource(LatestTerrainImpact::default())
        .insert_resource(CurrentImpactExplosionConsumed::default())
        .insert_resource(MovementFeedback::default())
        .insert_resource(MatchSetupGate::default())
        .insert_resource(PendingMatchConfiguration(configuration))
        .insert_resource(AimRepeatState::default())
        .insert_resource(TurretDinkCooldown::default())
        .insert_resource(BattlefieldGravity(
            Gravity::new(DEVELOPMENT_GRAVITY).expect("development gravity must be valid"),
        ))
        .insert_resource(BattlefieldWind(wind))
        .insert_resource(Time::<Fixed>::from_hz(PROJECTILE_FIXED_HZ))
        .add_systems(Startup, (spawn_battlefield_scene, spawn_match_setup))
        .add_systems(
            Update,
            (
                (
                    update_shot_presentation,
                    update_battlefield_camera,
                    update_match_setup,
                    select_weapon_input,
                    select_movement_action,
                    update_aiming_input,
                    update_movement_input,
                )
                    .chain(),
                (
                    run_ai_controller,
                    sync_tank_pose,
                    sync_tank_elimination,
                    sync_tank_aim,
                    update_wind_indicator_overlay,
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
    asset_server: Res<AssetServer>,
    scene: BattlefieldSceneResources,
) {
    let (tanks, turn, terrain, dressing, match_seed, configuration) = scene;
    let camera = BattlefieldCamera::default();
    let transform = camera_transform(&camera);
    commands.spawn((
        Camera3d::default(),
        SpatialListener::default(),
        IsDefaultUiCamera,
        Projection::Perspective(PerspectiveProjection {
            far: 300.0,
            ..default()
        }),
        camera,
        transform,
    ));
    commands.insert_resource(AudioAssets {
        fire: asset_server.load("audio/fire.ogg"),
        flight: asset_server.load("audio/flight.ogg"),
        impact: asset_server.load("audio/impact.ogg"),
        wind: asset_server.load("audio/wind.ogg"),
        turret_dink: asset_server.load("audio/turret-dink.ogg"),
    });
    spawn_wind_indicator_overlay(&mut commands, &mut meshes, &mut materials);

    commands.spawn((
        BattlefieldVisual,
        Mesh3d(meshes.add(create_battlefield_mesh(&terrain.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    commands.spawn((
        HorizonVisual,
        Mesh3d(meshes.add(create_horizon_mesh(&VisualHorizon::from_terrain(
            &terrain.0,
            BattlefieldSeed(derived_seed(match_seed.0, "terrain")),
        )))),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            ..default()
        })),
    ));
    spawn_clouds(&mut commands, &mut meshes, &mut materials);

    commands.spawn((
        WaterVisual,
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(HALF_EXTENT * 2.0, HALF_EXTENT * 2.0),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgb(0.04, 0.22, 0.70))),
        Transform::from_xyz(0.0, WATER_TABLE, 0.0),
    ));
    let building_material = materials.add(Color::srgb(0.36, 0.38, 0.40));
    spawn_buildings(
        &mut commands,
        &mut meshes,
        &dressing.0,
        building_material.clone(),
    );
    commands.insert_resource(WorldDressingAssets {
        material: building_material,
    });

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
    let player_materials = tank_materials(&mut materials);
    let firing_origin_material = materials.add(Color::srgb(0.95, 0.85, 0.2));
    commands.insert_resource(TankPresentationAssets {
        materials: player_materials.clone(),
        firing_origin_material: firing_origin_material.clone(),
    });
    commands.insert_resource(tank_meshes.clone());

    for tank in tanks.0.iter().copied() {
        let material = player_materials
            [(tank.owner.0.saturating_sub(1) as usize) % player_materials.len()]
        .clone();
        spawn_tank(
            &mut commands,
            tank,
            &tank_meshes,
            material,
            firing_origin_material.clone(),
            active_firing_representation(tank, turn.0.aim_for(tank.owner)),
        );
    }

    spawn_tactical_hud(&mut commands, &configuration.0.players);

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

/// This is a separate render-layer viewport, not a battlefield entity.  It gives the wind cue
/// genuine perspective and lighting while keeping it permanently outside the playable scene.
fn spawn_wind_indicator_overlay(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let layer = RenderLayers::layer(WIND_INDICATOR_RENDER_LAYER);
    commands.spawn((
        WindIndicatorCamera,
        Camera3d::default(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 28.0_f32.to_radians(),
            near: 0.1,
            far: 30.0,
            ..default()
        }),
        Transform::from_xyz(0.0, 2.2, 6.5).looking_at(Vec3::ZERO, Vec3::Y),
        layer.clone(),
        Visibility::Hidden,
    ));
    commands.spawn((
        PointLight {
            intensity: 90_000.0,
            range: 20.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, 4.0),
        layer.clone(),
    ));
    commands
        .spawn((
            WindIndicatorArrow,
            Transform::default(),
            Visibility::Hidden,
            layer.clone(),
        ))
        .with_children(|arrow| {
            arrow.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.13, 2.6))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.16, 0.43, 0.82),
                    metallic: 0.45,
                    perceptual_roughness: 0.25,
                    ..default()
                })),
                Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                layer.clone(),
            ));
            arrow.spawn((
                Mesh3d(meshes.add(Cone::new(0.48, 1.15))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.98, 0.73, 0.12),
                    metallic: 0.3,
                    perceptual_roughness: 0.2,
                    ..default()
                })),
                Transform::from_translation(Vec3::X * 1.75)
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                layer,
            ));
        });
}

fn spawn_match_setup(mut commands: Commands) {
    commands
        .spawn((
            MatchSetupOverlay,
            Node {
                width: percent(100.0),
                height: percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.03, 0.06, 0.88)),
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: px(500.0),
                    padding: UiRect::all(px(28.0)),
                    row_gap: px(14.0),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.09, 0.14, 0.98)),
                BorderColor::all(Color::srgb(0.9, 0.75, 0.2)),
            ))
            .with_children(|panel| {
                setup_text(
                    panel,
                    "AZIMUTH — MATCH SETUP",
                    30.0,
                    Color::srgb(0.95, 0.8, 0.25),
                );
                panel.spawn((
                    MatchSetupDetails,
                    Text::new(""),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                setup_text(
                    panel,
                    "2–8: COUNT | UP/DOWN: SLOT | TYPE: NAME | CTRL+C: HUMAN/AI | CTRL+R: REROLL AI",
                    16.0,
                    Color::srgb(0.6, 0.75, 0.9),
                );
                setup_text(
                    panel,
                    "PRESS ENTER TO START MATCH",
                    18.0,
                    Color::srgb(0.95, 0.8, 0.25),
                );
            });
        });
}

fn setup_text(parent: &mut ChildSpawnerCommands, value: &str, font_size: f32, color: Color) {
    parent.spawn((
        Text::new(value),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
    ));
}

#[allow(clippy::too_many_arguments)]
fn update_match_setup(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gate: ResMut<MatchSetupGate>,
    mut configuration: ResMut<PendingMatchConfiguration>,
    mut tanks: ResMut<Tanks>,
    mut turn: ResMut<CurrentTurn>,
    mut weapons: ResMut<WeaponState>,
    world: MatchSetupWorldResources,
    dressing_render: (ResMut<Assets<Mesh>>, Res<WorldDressingAssets>),
    tank_meshes: Res<TankMeshes>,
    presentation: Res<TankPresentationAssets>,
    overlays: Query<Entity, With<MatchSetupOverlay>>,
    huds: Query<Entity, With<TacticalHud>>,
    tank_visuals: Query<Entity, With<TankVisual>>,
    building_visuals: Query<Entity, With<BuildingVisual>>,
    mut commands: Commands,
    mut details: Query<&mut Text, With<MatchSetupDetails>>,
) {
    let (mut terrain, match_seed, mut dressing, mut wind, mut ai_seed) = world;
    let (mut meshes, dressing_assets) = dressing_render;
    if gate.started {
        return;
    }
    let requested_count = [
        (KeyCode::Digit2, 2),
        (KeyCode::Digit3, 3),
        (KeyCode::Digit4, 4),
        (KeyCode::Digit5, 5),
        (KeyCode::Digit6, 6),
        (KeyCode::Digit7, 7),
        (KeyCode::Digit8, 8),
    ]
    .into_iter()
    .find_map(|(key, count)| keyboard.just_pressed(key).then_some(count));
    if let Some(count) = requested_count {
        configuration
            .0
            .set_player_count(count)
            .expect("setup controls use valid counts");
        gate.selected_slot = gate.selected_slot.min(count - 1);
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        gate.selected_slot = gate
            .selected_slot
            .checked_sub(1)
            .unwrap_or(configuration.0.players.len() - 1);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        gate.selected_slot = (gate.selected_slot + 1) % configuration.0.players.len();
    }
    let control_held =
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    if control_held && keyboard.just_pressed(KeyCode::KeyC) {
        let (id, previous) = {
            let slot = &configuration.0.players[gate.selected_slot];
            (slot.id, slot.controller)
        };
        let controller = if previous == ControllerType::Human {
            ControllerType::Ai
        } else {
            ControllerType::Human
        };
        configuration.0.set_controller(id, controller);
    }
    if control_held && keyboard.just_pressed(KeyCode::KeyR) {
        let id = configuration.0.players[gate.selected_slot].id;
        configuration.0.reroll_ai_name(id);
    }
    if !control_held {
        let id = configuration.0.players[gate.selected_slot].id;
        if keyboard.just_pressed(KeyCode::Backspace) {
            configuration.0.backspace_human_name(id);
        } else if let Some(character) = setup_name_character(&keyboard) {
            configuration.0.append_human_name_character(id, character);
        }
    }
    let lines = configuration
        .0
        .players
        .iter()
        .enumerate()
        .map(|(index, player)| {
            let marker = if index == gate.selected_slot {
                ">"
            } else {
                " "
            };
            format!(
                "{marker} {}. [{}] {}    {:?}",
                index + 1,
                player.visual.label(),
                player.display_name,
                player.controller
            )
            .to_uppercase()
        })
        .collect::<Vec<_>>()
        .join("\n");
    let validation_message = match configuration.0.validate() {
        Ok(()) => "ENTER: START MATCH".to_owned(),
        Err(error) => format!("Cannot start: {error:?}"),
    };
    for mut text in &mut details {
        text.0 = format!(
            "PLAYERS: {}\n{lines}\n\n{validation_message}",
            configuration.0.players.len()
        );
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        configuration.0.trim_human_names();
    }
    if keyboard.just_pressed(KeyCode::Enter) && configuration.0.validate().is_ok() {
        let ids = configuration
            .0
            .players
            .iter()
            .map(|player| player.id)
            .collect::<Vec<_>>();
        let (generated, generated_tanks, generated_dressing, generated_wind) =
            generate_match_world(match_seed.0, &ids);
        terrain.0 = generated;
        tanks.0 = generated_tanks;
        dressing.0 = generated_dressing;
        wind.0 = generated_wind;
        turn.0 = initial_turn_state(&tanks.0);
        weapons.0 = PlayerWeaponLoadouts::new(&ids);
        // Gameplay AI gets its own labeled stream; setup names and scenery cannot perturb it.
        ai_seed.0 = derived_seed(match_seed.0, "ai");
        for entity in &tank_visuals {
            commands.entity(entity).despawn();
        }
        for tank in tanks.0.iter().copied() {
            let material = presentation.materials
                [(tank.owner.0.saturating_sub(1) as usize) % presentation.materials.len()]
            .clone();
            spawn_tank(
                &mut commands,
                tank,
                &tank_meshes,
                material,
                presentation.firing_origin_material.clone(),
                active_firing_representation(tank, turn.0.aim_for(tank.owner)),
            );
        }
        for entity in &building_visuals {
            commands.entity(entity).despawn();
        }
        spawn_buildings(
            &mut commands,
            &mut meshes,
            &dressing.0,
            dressing_assets.material.clone(),
        );
        for entity in &huds {
            commands.entity(entity).despawn();
        }
        spawn_tactical_hud(&mut commands, &configuration.0.players);
        gate.started = true;
        for overlay in &overlays {
            commands.entity(overlay).despawn();
        }
    }
}

/// The setup uses a deliberately small keyboard editor instead of a general UI text-input
/// framework.  It accepts the printable letters and a space needed by the short display-name
/// contract; Shift changes case and Backspace is handled by the caller.
fn setup_name_character(keyboard: &ButtonInput<KeyCode>) -> Option<char> {
    let shifted = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let keys = [
        (KeyCode::KeyA, 'a'),
        (KeyCode::KeyB, 'b'),
        (KeyCode::KeyC, 'c'),
        (KeyCode::KeyD, 'd'),
        (KeyCode::KeyE, 'e'),
        (KeyCode::KeyF, 'f'),
        (KeyCode::KeyG, 'g'),
        (KeyCode::KeyH, 'h'),
        (KeyCode::KeyI, 'i'),
        (KeyCode::KeyJ, 'j'),
        (KeyCode::KeyK, 'k'),
        (KeyCode::KeyL, 'l'),
        (KeyCode::KeyM, 'm'),
        (KeyCode::KeyN, 'n'),
        (KeyCode::KeyO, 'o'),
        (KeyCode::KeyP, 'p'),
        (KeyCode::KeyQ, 'q'),
        (KeyCode::KeyR, 'r'),
        (KeyCode::KeyS, 's'),
        (KeyCode::KeyT, 't'),
        (KeyCode::KeyU, 'u'),
        (KeyCode::KeyV, 'v'),
        (KeyCode::KeyW, 'w'),
        (KeyCode::KeyX, 'x'),
        (KeyCode::KeyY, 'y'),
        (KeyCode::KeyZ, 'z'),
    ];
    if keyboard.just_pressed(KeyCode::Space) {
        Some(' ')
    } else {
        keys.into_iter().find_map(|(key, character)| {
            keyboard.just_pressed(key).then_some(if shifted {
                character.to_ascii_uppercase()
            } else {
                character
            })
        })
    }
}

// This direct Bevy system deliberately keeps the one atomic launch boundary visible; a custom
// SystemParam would add indirection without reducing its gameplay responsibilities.
#[allow(clippy::too_many_arguments)]
fn launch_aimed_projectile(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    tanks: Res<Tanks>,
    assets: Res<ProjectileVisualAssets>,
    audio: Res<AudioAssets>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
    mut weapons: ResMut<WeaponState>,
    mut turn: ResMut<CurrentTurn>,
    mut commands: Commands,
) {
    if !setup.started
        || !keyboard.just_pressed(KeyCode::Space)
        || !current_player_is_human(&configuration.0, &turn.0)
        || !presentation_allows_new_action(&presentation)
        || flight.0.is_some()
        || turn.0.match_state != MatchState::InProgress
        || turn.0.phase != TurnPhase::Choosing
    {
        return;
    }

    fire_current_player(
        &tanks.0,
        &assets,
        &audio,
        &mut flight.0,
        &mut latest_impact,
        &mut weapons.0,
        &mut turn.0,
        &mut commands,
    );
}

#[allow(clippy::too_many_arguments)]
fn fire_current_player(
    tanks: &[Tank],
    assets: &ProjectileVisualAssets,
    audio: &AudioAssets,
    flight: &mut Option<FiredShot>,
    latest_impact: &mut LatestTerrainImpact,
    weapons: &mut PlayerWeaponLoadouts,
    turn: &mut TurnState,
    commands: &mut Commands,
) -> bool {
    if flight.is_some()
        || turn.match_state != MatchState::InProgress
        || turn.phase != TurnPhase::Choosing
    {
        return false;
    }
    let player = turn.current_player;
    let Some(definition) = weapons.for_player_mut(player).commit_selected() else {
        return false;
    };
    let Some((player, aiming)) = turn.begin_fire() else {
        unreachable!("choosing player with a committed weapon must begin fire");
    };
    let tank = tank_for_player(tanks, player);
    let parameters = launch_parameters_for_aim(tank, aiming);
    let projectile =
        Projectile::launch_with_wind_response(parameters, definition.projectile.wind_response);
    let shot = FiredShot::new(definition, projectile);

    commands.spawn((
        Name::new("Aimed projectile"),
        ProjectileVisual,
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform::from_translation(to_bevy_position(shot.projectile.position)),
    ));
    // Successful shared launch is the sole physical fire boundary for Human and AI turns.
    commands.spawn((
        Name::new("Weapon fire audio"),
        AudioPlayer(audio.fire.clone()),
        // Keep the first audible report non-spatial until the listener mix is verified. The
        // request still originates at the authoritative tank/weapon boundary; playback never
        // participates in firing or simulation.
        PlaybackSettings { volume: Volume::Linear(if definition.id == WeaponId::HeavyShell { 1.25 } else { 1.05 }), spatial: false, ..PlaybackSettings::DESPAWN },
        Transform::from_translation(to_bevy_position(parameters.launch_position)),
    ));
    *flight = Some(shot);
    latest_impact.impact = None;
    latest_impact.explosion_visual_scale = 1.0;
    true
}

fn current_player_is_human(configuration: &MatchConfiguration, turn: &TurnState) -> bool {
    configuration
        .players
        .iter()
        .find(|player| player.id == turn.current_player)
        .is_some_and(|player| player.controller == ControllerType::Human)
}

fn shot_mode_for_player(
    configuration: &MatchConfiguration,
    player: PlayerId,
) -> ShotPresentationMode {
    match configuration
        .players
        .iter()
        .find(|configured| configured.id == player)
        .expect("active player must have match configuration")
        .controller
    {
        ControllerType::Human => ShotPresentationMode::HumanFollow,
        ControllerType::Ai => ShotPresentationMode::AiTactical,
    }
}

fn presentation_allows_new_action(presentation: &ShotPresentation) -> bool {
    !matches!(presentation.phase, ShotPresentationPhase::Impact { .. })
}

/// This runs before controllers each rendered frame. It observes already-authoritative resources:
/// no camera transition can slow flight, terrain deformation, settling, or turn completion.
fn update_shot_presentation(
    time: Res<Time>,
    configuration: Res<PendingMatchConfiguration>,
    flight: Res<ProjectileFlight>,
    latest_impact: Res<LatestTerrainImpact>,
    tanks: Res<Tanks>,
    turn: Res<CurrentTurn>,
    mut presentation: ResMut<ShotPresentation>,
) {
    match (presentation.phase, flight.0) {
        (ShotPresentationPhase::PlayerView, None) => {
            if let Some(impact) = latest_impact.impact
                && presentation.seen_impact != Some(impact)
            {
                presentation.seen_impact = Some(impact);
                presentation.phase = ShotPresentationPhase::Impact {
                    position: impact.position,
                    elapsed_seconds: 0.0,
                };
            }
        }
        (ShotPresentationPhase::PlayerView, Some(shot)) => {
            presentation.phase = ShotPresentationPhase::Flight {
                mode: shot_mode_for_player(&configuration.0, turn.0.current_player),
                shooter: turn.0.current_player,
                apex_seen: shot.projectile.velocity.y <= 0.0,
            };
        }
        (
            ShotPresentationPhase::Flight {
                mode,
                shooter,
                apex_seen,
            },
            Some(shot),
        ) => {
            presentation.phase = ShotPresentationPhase::Flight {
                mode,
                shooter,
                apex_seen: apex_seen || shot.projectile.velocity.y <= 0.0,
            };
        }
        (ShotPresentationPhase::Flight { .. }, None) if latest_impact.impact.is_some() => {
            presentation.seen_impact = latest_impact.impact;
            presentation.phase = ShotPresentationPhase::Impact {
                position: latest_impact.impact.expect("checked impact").position,
                elapsed_seconds: 0.0,
            };
        }
        (ShotPresentationPhase::Flight { .. }, None) => {
            presentation.phase = if turn.0.match_state == MatchState::InProgress {
                ShotPresentationPhase::PlayerView
            } else {
                ShotPresentationPhase::Result {
                    position: WorldPosition {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                }
            };
        }
        (
            ShotPresentationPhase::Impact {
                position,
                elapsed_seconds,
            },
            None,
        ) => {
            let elapsed_seconds = elapsed_seconds + time.delta_secs();
            if elapsed_seconds >= IMPACT_VIEW_HOLD_SECONDS && !any_living_tank_is_settling(&tanks.0)
            {
                presentation.phase = if turn.0.match_state == MatchState::InProgress {
                    ShotPresentationPhase::PlayerView
                } else {
                    ShotPresentationPhase::Result { position }
                };
            } else {
                presentation.phase = ShotPresentationPhase::Impact {
                    position,
                    elapsed_seconds,
                };
            }
        }
        (ShotPresentationPhase::Result { .. }, _) => {}
        (_, Some(_)) => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn run_ai_controller(
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    tanks: Res<Tanks>,
    assets: Res<ProjectileVisualAssets>,
    audio: Res<AudioAssets>,
    mut seed: ResMut<AiDecisionSeed>,
    mut flight: ResMut<ProjectileFlight>,
    mut latest_impact: ResMut<LatestTerrainImpact>,
    mut weapons: ResMut<WeaponState>,
    mut turn: ResMut<CurrentTurn>,
    mut commands: Commands,
) {
    if !setup.started
        || !presentation_allows_new_action(&presentation)
        || current_player_is_human(&configuration.0, &turn.0)
    {
        return;
    }
    let actor = turn.0.current_player;
    let Some(decision) = decide_firing(&mut seed.0, actor, &tanks.0) else {
        return;
    };
    if !turn.0.set_current_aim(decision.aim)
        || !weapons.0.for_player_mut(actor).select(decision.weapon)
    {
        return;
    }
    fire_current_player(
        &tanks.0,
        &assets,
        &audio,
        &mut flight.0,
        &mut latest_impact,
        &mut weapons.0,
        &mut turn.0,
        &mut commands,
    );
}

fn select_weapon_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    flight: Res<ProjectileFlight>,
    turn: Res<CurrentTurn>,
    mut weapons: ResMut<WeaponState>,
) {
    if !setup.started
        || flight.0.is_some()
        || !presentation_allows_new_action(&presentation)
        || !current_player_is_human(&configuration.0, &turn.0)
        || turn.0.match_state != MatchState::InProgress
        || turn.0.phase != TurnPhase::Choosing
    {
        return;
    }
    let weapon = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(WeaponId::BasicShell)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(WeaponId::HighExplosive)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(WeaponId::HeavyShell)
    } else {
        None
    };
    if let Some(weapon) = weapon {
        weapons
            .0
            .for_player_mut(turn.0.current_player)
            .select(weapon);
    }
}

fn select_movement_action(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    mut feedback: ResMut<MovementFeedback>,
    mut turn: ResMut<CurrentTurn>,
) {
    if setup.started
        && presentation_allows_new_action(&presentation)
        && current_player_is_human(&configuration.0, &turn.0)
        && keyboard.just_pressed(KeyCode::KeyM)
        && turn.0.begin_movement()
    {
        feedback.0 = None;
    }
}

#[allow(clippy::too_many_arguments)]
fn update_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    terrain: Res<BattlefieldState>,
    camera: Single<&BattlefieldCamera>,
    mut tanks: ResMut<Tanks>,
    mut feedback: ResMut<MovementFeedback>,
    mut turn: ResMut<CurrentTurn>,
) {
    if !setup.started
        || !current_player_is_human(&configuration.0, &turn.0)
        || !presentation_allows_new_action(&presentation)
        || turn.0.remaining_movement().is_none()
    {
        return;
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        if turn.0.finish_movement(survivors(&tanks.0)) {
            feedback.0 = None;
        }
        return;
    }
    let Some(direction) = movement_direction(&keyboard, camera.yaw) else {
        return;
    };
    let player = turn.0.current_player;
    let tank = tank_for_player(&tanks.0, player);
    match tank.step_on_terrain(&terrain.0, direction) {
        Ok(moved) => {
            *tank_for_player_mut(&mut tanks.0, player) = moved;
            assert!(
                turn.0.accept_movement_step(survivors(&tanks.0)),
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

#[allow(clippy::too_many_arguments)]
fn update_aiming_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    flight: Res<ProjectileFlight>,
    time: Res<Time>,
    mut repeat_state: ResMut<AimRepeatState>,
    audio: Res<AudioAssets>,
    mut dink_cooldown: ResMut<TurretDinkCooldown>,
    mut turn: ResMut<CurrentTurn>,
    mut commands: Commands,
) {
    if !setup.started
        || !current_player_is_human(&configuration.0, &turn.0)
        || flight.0.is_some()
        || !presentation_allows_new_action(&presentation)
        || turn.0.phase != TurnPhase::Choosing
    {
        repeat_state.reset();
        return;
    }

    dink_cooldown.0 = (dink_cooldown.0 - time.delta_secs()).max(0.0);
    let coarse = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    for (adjustment, count) in aiming_adjustments(&keyboard, time.delta_secs(), &mut repeat_state) {
        for _ in 0..count {
            let before = turn.0.current_aim();
            turn.0.apply_current_aim(adjustment, coarse);
            let after = turn.0.current_aim();
            let rotates_turret = matches!(adjustment, AimAdjustment::AzimuthDecrease | AimAdjustment::AzimuthIncrease | AimAdjustment::ElevationIncrease | AimAdjustment::ElevationDecrease);
            if rotates_turret && before != after && dink_cooldown.0 == 0.0 {
                commands.spawn((Name::new("Turret adjustment audio"), AudioPlayer(audio.turret_dink.clone()), PlaybackSettings { volume: Volume::Linear(0.38), spatial: false, ..PlaybackSettings::DESPAWN }));
                dink_cooldown.0 = TURRET_DINK_INTERVAL_SECONDS;
            }
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

fn initial_turn_state(tanks: impl AsRef<[Tank]>) -> TurnState {
    TurnState::new(
        tanks
            .as_ref()
            .iter()
            .copied()
            .map(|tank| (tank.owner, initial_aim_for_tank(tank)))
            .collect(),
    )
}

fn initial_aim_for_tank(tank: Tank) -> AimingState {
    let azimuth =
        azimuth_from_horizontal_direction(tank.pose.turret_forward.x, tank.pose.turret_forward.z)
            .expect("initial tank turret direction must be valid");
    AimingState::new(azimuth, 45.0, 18.0)
}

fn tank_for_player(tanks: impl AsRef<[Tank]>, player: PlayerId) -> Tank {
    *tanks
        .as_ref()
        .iter()
        .find(|tank| tank.owner == player)
        .expect("each current player must own one initial tank")
}

fn tank_for_player_mut(tanks: &mut [Tank], player: PlayerId) -> &mut Tank {
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
        let tank = tank_for_player(&tanks.0, owner.0);
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
        let tank = tank_for_player(&tanks.0, owner.0);
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
        let position = tank_for_player(&tanks.0, owner.0).pose.position;
        transform.translation = to_bevy_position(position);
    }
    for (owner, mut transform) in &mut bodies {
        let tank = tank_for_player(&tanks.0, owner.0);
        *transform =
            direction_transform(tank.pose.body_forward).with_translation(Vec3::new(0.0, 0.4, 0.0));
    }
}

fn sync_tank_elimination(tanks: Res<Tanks>, mut visuals: Query<(&TankVisual, &mut Visibility)>) {
    for (owner, mut visibility) in &mut visuals {
        *visibility = if tank_for_player(&tanks.0, owner.0).is_eliminated() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

/// Keeps the isolated wind viewport aligned with its UI reservation and rotates its mesh from the
/// active player's live turret aim.  Neither the camera nor its arrow participates in gameplay
/// visibility, collision, or the battlefield render layer.
#[allow(clippy::type_complexity)]
fn update_wind_indicator_overlay(
    setup: Res<MatchSetupGate>,
    turn: Res<CurrentTurn>,
    wind: Res<BattlefieldWind>,
    window: Single<&Window>,
    camera: Single<
        (&mut Camera, &mut Visibility, &mut Transform),
        (With<WindIndicatorCamera>, Without<WindIndicatorArrow>),
    >,
    arrow: Single<
        (&mut Transform, &mut Visibility),
        (With<WindIndicatorArrow>, Without<WindIndicatorCamera>),
    >,
) {
    let (mut camera, mut camera_visibility, mut camera_transform) = camera.into_inner();
    let (mut arrow, mut arrow_visibility) = arrow.into_inner();
    let visible = setup.started;
    *camera_visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    *arrow_visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    let scale = window.scale_factor();
    let width = (WIND_INDICATOR_WIDTH as f32 * scale).round() as u32;
    let height = (WIND_INDICATOR_HEIGHT as f32 * scale).round() as u32;
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(
            window.physical_width().saturating_sub(width) / 2,
            (16.0 * scale).round() as u32,
        ),
        physical_size: UVec2::new(width, height),
        ..default()
    });
    let aim = (turn.0.match_state == MatchState::InProgress).then(|| turn.0.current_aim());
    *camera_transform = wind_indicator_camera_transform(aim);
    arrow.rotation = Quat::from_rotation_y(wind_arrow_rotation(wind.0));
}

fn spawn_tactical_hud(commands: &mut Commands, players: &[PlayerConfiguration]) {
    commands
        .spawn((
            TacticalHud,
            Visibility::Hidden,
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
                for player in players {
                    scoreboard_row(p, player.id, players.len());
                }
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
                hud_text(p, HudTextField::Weapon, 16.0);
            });
            root.spawn((
                BackgroundColor(Color::srgba(0.03, 0.05, 0.08, 0.82)),
                BorderColor::all(Color::srgb(0.55, 0.7, 0.9)),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(16),
                    left: percent(50),
                    width: px(150),
                    margin: UiRect::left(px(-75)),
                    padding: UiRect::vertical(px(5)),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(px(2)),
                    ..default()
                },
            ))
            .with_children(|wind| {
                // The indicator camera occupies this reserved space. Keeping this node in the UI
                // layout places the KPH readout directly beneath the 3D overlay viewport.
                wind.spawn(Node {
                    width: px(WIND_INDICATOR_WIDTH as f32),
                    height: px(WIND_INDICATOR_HEIGHT as f32),
                    ..default()
                });
                hud_text(wind, HudTextField::Wind, 15.0);
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
fn scoreboard_row(parent: &mut ChildSpawnerCommands, player: PlayerId, player_count: usize) {
    let (font_size, bar_height, row_gap) = scoreboard_layout(player_count);
    parent
        .spawn((
            HudScoreboardRow(player),
            BackgroundColor(Color::srgba(0.08, 0.11, 0.15, 0.92)),
            BorderColor::all(Color::srgba(0.45, 0.45, 0.45, 0.7)),
            Node {
                width: percent(100),
                padding: UiRect::horizontal(px(4)),
                border: UiRect::all(px(1)),
                row_gap: px(row_gap),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn((
                HudScoreboardText(player),
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            row.spawn((
                BackgroundColor(Color::srgba(0.15, 0.18, 0.22, 0.95)),
                Node {
                    width: percent(100),
                    height: px(bar_height),
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
        });
}

fn scoreboard_layout(player_count: usize) -> (f32, f32, f32) {
    if player_count > 4 {
        (13.0, 6.0, 2.0)
    } else {
        (16.0, 8.0, 3.0)
    }
}

fn tactical_hud_view(
    turn: TurnState,
    tanks: impl AsRef<[Tank]>,
    configuration: &MatchConfiguration,
    weapons: PlayerWeaponLoadouts,
    wind: Wind,
    feedback: Option<MovementRejection>,
) -> TacticalHudView {
    let tanks = tanks.as_ref();
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
        scoreboard: scoreboard_entries(configuration, tanks, &turn),
        aim: (turn.match_state == MatchState::InProgress).then_some(turn.current_aim()),
        weapon: (turn.match_state == MatchState::InProgress).then(|| {
            let loadout = weapons.for_player(turn.current_player);
            (loadout.selected(), loadout.availability(loadout.selected()))
        }),
        wind,
        movement: turn.remaining_movement().map(|steps| (steps, feedback)),
        result: turn.match_state,
    }
}

/// The configuration owns display order and names; tanks and turns only supply current gameplay
/// condition. Keeping this projection pure prevents HUD state from becoming gameplay authority.
fn scoreboard_entries(
    configuration: &MatchConfiguration,
    tanks: &[Tank],
    turn: &TurnState,
) -> Vec<ScoreboardEntry> {
    configuration
        .players
        .iter()
        .map(|player| {
            let tank = tanks.iter().find(|tank| tank.owner == player.id);
            ScoreboardEntry {
                player: player.id,
                display_name: player.display_name.clone(),
                health: tank.map(|tank| tank.health),
                eliminated: tank.is_some_and(|tank| tank.is_eliminated()),
                active: turn.match_state == MatchState::InProgress
                    && turn.phase != TurnPhase::ResolvingFire
                    && turn.current_player == player.id,
            }
        })
        .collect()
}

/// Presentation observes state only; no HUD path mutates gameplay or gates fixed simulation.
// The grouped reads are intentionally explicit so every HUD source remains visibly read-only.
#[allow(clippy::too_many_arguments)]
fn sync_tactical_hud(
    setup: Res<MatchSetupGate>,
    turn: Res<CurrentTurn>,
    tanks: Res<Tanks>,
    weapons: Res<WeaponState>,
    wind: Res<BattlefieldWind>,
    feedback: Res<MovementFeedback>,
    configuration: Res<PendingMatchConfiguration>,
    mut text: Query<(&HudText, &mut Text), Without<HudScoreboardText>>,
    mut scoreboard_text: Query<(&HudScoreboardText, &mut Text), Without<HudText>>,
    mut fills: Query<(&HudHealthFill, &mut Node)>,
    mut rows: Query<(&HudScoreboardRow, &mut BorderColor, &mut BackgroundColor)>,
    mut decorations: HudDecorations,
    mut huds: Query<&mut Visibility, With<TacticalHud>>,
) {
    let view = tactical_hud_view(
        turn.0.clone(),
        tanks.0.clone(),
        &configuration.0,
        weapons.0.clone(),
        wind.0,
        feedback.0,
    );
    for (field, mut value) in &mut text {
        value.0 = if field.0 == HudTextField::Match {
            configured_match_text(&view, &configuration.0)
        } else {
            hud_field_text(field.0, &view)
        };
    }
    for (field, mut value) in &mut scoreboard_text {
        let entry = view
            .scoreboard
            .iter()
            .find(|entry| entry.player == field.0)
            .expect("every retained scoreboard row must have a configured player");
        value.0 = scoreboard_entry_text(entry);
    }
    for (fill, mut node) in &mut fills {
        let entry = view
            .scoreboard
            .iter()
            .find(|entry| entry.player == fill.0)
            .expect("every retained scoreboard fill must have a configured player");
        node.width = percent(entry.health.unwrap_or_default() as f32 / MAX_HEALTH as f32 * 100.0);
    }
    for (row, mut border, mut background) in &mut rows {
        let entry = view
            .scoreboard
            .iter()
            .find(|entry| entry.player == row.0)
            .expect("every retained scoreboard row must have a configured player");
        let colour = if entry.active {
            player_color(entry.player)
        } else if entry.eliminated {
            Color::srgb(0.32, 0.12, 0.12)
        } else {
            Color::srgba(0.45, 0.45, 0.45, 0.7)
        };
        border.top = colour;
        border.right = colour;
        border.bottom = colour;
        border.left = colour;
        background.0 = if entry.eliminated {
            Color::srgba(0.13, 0.05, 0.05, 0.88)
        } else {
            Color::srgba(0.08, 0.11, 0.15, 0.92)
        };
    }
    let active_colour = view
        .active_player
        .map_or(Color::srgb(0.45, 0.45, 0.45), player_color);
    for mut visibility in &mut huds {
        *visibility = if setup.started {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for (border, active_panel) in &mut decorations {
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

fn scoreboard_entry_text(entry: &ScoreboardEntry) -> String {
    if entry.eliminated {
        format!("{}: OUT", entry.display_name)
    } else if let Some(health) = entry.health {
        format!("{}: {health}/{MAX_HEALTH}", entry.display_name)
    } else {
        format!("{}: READY", entry.display_name)
    }
}

/// Names remain configuration-owned metadata; the running HUD only resolves stable IDs through
/// that metadata and never uses editable text as an identity key.
fn configured_match_text(view: &TacticalHudView, configuration: &MatchConfiguration) -> String {
    let name_for = |id| {
        configuration
            .players
            .iter()
            .find(|player| player.id == id)
            .map_or_else(|| player_name(id), |player| player.display_name.clone())
    };
    match view.result {
        MatchState::Winner(player) => format!("{} WINS", name_for(player)),
        MatchState::Draw => "DRAW".into(),
        MatchState::InProgress => format!(
            "{} - {}",
            name_for(
                view.active_player
                    .expect("active player while match is in progress")
            ),
            match view.action {
                HudAction::Choose => "CHOOSE ACTION",
                HudAction::Moving => "MOVING",
                HudAction::Resolving => "RESOLVING SHOT",
                HudAction::Finished => "MATCH OVER",
            }
        ),
    }
}

/// The arrow retains the actual world wind direction; its dedicated camera supplies the active
/// turret-relative frame, so a head-on wind visibly travels into or out of the screen.
fn wind_arrow_rotation(wind: Wind) -> f32 {
    let vector = wind.horizontal_acceleration();
    let strength = wind.strength();
    if strength == 0.0 {
        0.0
    } else {
        (-vector.z).atan2(vector.x)
    }
}

fn wind_indicator_camera_transform(aim: Option<AimingState>) -> Transform {
    let azimuth = aim.map_or(0.0, |aim| aim.azimuth_degrees.to_radians());
    let position = Quat::from_rotation_y(-azimuth) * Vec3::new(0.0, 2.2, 6.5);
    Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y)
}

fn hud_field_text(field: HudTextField, view: &TacticalHudView) -> String {
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
        HudTextField::Aim => view.aim.map_or_else(
            || "AIM LOCKED".into(),
            |a| {
                format!(
                    "AZ {:.0}\nEL {:.0}\nPOWER {:.1}",
                    a.azimuth_degrees, a.elevation_degrees, a.launch_speed
                )
            },
        ),
        HudTextField::Weapon => view.weapon.map_or_else(
            || "WEAPON LOCKED".into(),
            |(weapon, availability)| match availability {
                WeaponAvailability::Unlimited => {
                    format!("{}: UNLIMITED", weapon_definition(weapon).display_name)
                }
                WeaponAvailability::Remaining(rounds) => {
                    format!("{}: x{rounds}", weapon_definition(weapon).display_name)
                }
            },
        ),
        HudTextField::Wind => format!(
            "{:.0} KPH",
            view.wind.strength() * WIND_KPH_PER_ACCELERATION
        ),
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
            HudAction::Choose => {
                "1 BASIC | 2 HE | 3 HEAVY | M MOVE | SPACE FIRE\nARROWS AIM | -/= POWER".into()
            }
            HudAction::Moving => "ARROWS MOVE (CAMERA) | ENTER END".into(),
            HudAction::Resolving | HudAction::Finished => String::new(),
        },
    }
}
fn player_color(player: PlayerId) -> Color {
    const COLOURS: [Color; 8] = [
        Color::srgb(0.85, 0.25, 0.18),
        Color::srgb(0.18, 0.4, 0.85),
        Color::srgb(0.2, 0.72, 0.32),
        Color::srgb(0.95, 0.5, 0.12),
        Color::srgb(0.58, 0.3, 0.8),
        Color::srgb(0.1, 0.72, 0.72),
        Color::srgb(0.9, 0.28, 0.57),
        Color::srgb(0.92, 0.82, 0.18),
    ];
    COLOURS[(player.0.saturating_sub(1) as usize) % COLOURS.len()]
}

fn tank_materials(materials: &mut Assets<StandardMaterial>) -> Vec<Handle<StandardMaterial>> {
    (1..=8)
        .map(|id| materials.add(player_color(PlayerId(id))))
        .collect()
}

fn select_match_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64)
}

/// The labelled streams make visual dressing unable to perturb terrain, tank starts, or wind.
/// The captured seed is printed so an interesting or broken battlefield can be reproduced.
fn derived_seed(seed: BattlefieldSeed, label: &str) -> u64 {
    label.bytes().fold(seed.0, |value, byte| {
        value
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(byte as u64 + 1)
    })
}

fn generate_match_world(
    seed: BattlefieldSeed,
    players: &[PlayerId],
) -> (BattlefieldTerrain, Vec<Tank>, Vec<BuildingPlacement>, Wind) {
    let terrain = BattlefieldTerrain::generated(BattlefieldSeed(derived_seed(seed, "terrain")));
    let tanks = initial_tanks_for_players_seeded(&terrain, players, derived_seed(seed, "starts"));
    let starts = tanks
        .iter()
        .map(|tank| (tank.pose.position.x, tank.pose.position.z))
        .collect::<Vec<_>>();
    let dressing = generate_buildings(&terrain, &starts, derived_seed(seed, "dressing"));
    let wind = if WIND_ENABLED {
        wind_from_seed(derived_seed(seed, "wind"))
    } else {
        Wind::new(WorldVector::ZERO).expect("calm wind must be valid")
    };
    eprintln!("Azimuth battlefield seed: {}", seed.0);
    (terrain, tanks, dressing, wind)
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

fn player_name(player: PlayerId) -> String {
    format!("Player {}", player.0)
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
    let Some(mut shot) = flight.0 else {
        return;
    };

    let advance = shot.projectile.advance_with_terrain(
        gravity.0,
        wind.0,
        SimulationLimits::BATTLEFIELD,
        |x, z| terrain.0.height_if_within_bounds(x, z),
    );
    flight.0 = resolve_projectile_advance(
        shot,
        advance,
        &mut terrain.0,
        &mut tanks.0,
        &mut latest_impact,
        &mut turn.0,
    );
}

fn resolve_projectile_advance(
    shot: FiredShot,
    advance: ProjectileAdvance,
    terrain: &mut BattlefieldTerrain,
    tanks: &mut [Tank],
    latest_impact: &mut LatestTerrainImpact,
    turn: &mut TurnState,
) -> Option<FiredShot> {
    match advance {
        ProjectileAdvance::Active => Some(shot),
        ProjectileAdvance::TerrainImpact(impact) => {
            resolve_explosion(tanks, impact.position, shot.impact);
            terrain.apply_crater(impact.position, shot.impact.crater);
            latest_impact.impact = Some(impact);
            latest_impact.explosion_visual_scale = shot.impact.explosion_visual_scale;
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

fn reconcile_living_tank_support(tanks: &mut [Tank], terrain: &BattlefieldTerrain) {
    for tank in tanks {
        if !tank.is_eliminated() {
            tank.reconcile_support(terrain);
        }
    }
}

fn any_living_tank_is_settling(tanks: &[Tank]) -> bool {
    tanks
        .iter()
        .any(|tank| !tank.is_eliminated() && tank.is_settling())
}

fn complete_resolution_if_settled(tanks: &[Tank], turn: &mut TurnState) {
    if !any_living_tank_is_settling(tanks) {
        assert!(
            turn.complete_fire_resolution(survivors(tanks)),
            "only a resolving shot may terminate"
        );
    }
}

fn survivors(tanks: &[Tank]) -> Vec<bool> {
    tanks.iter().map(|tank| !tank.is_eliminated()).collect()
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
    if let Some(shot) = flight.0 {
        for (_, mut transform) in &mut visuals {
            transform.translation = to_bevy_position(shot.projectile.position);
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
    if let Some(impact) = latest_impact.impact {
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
    audio: Res<AudioAssets>,
    mut consumed: ResMut<CurrentImpactExplosionConsumed>,
    mut commands: Commands,
) {
    let Some(impact) = latest_impact.impact else {
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
            scale_multiplier: latest_impact.explosion_visual_scale,
        },
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        explosion_transform(impact.position),
    ));
    // The persistent authoritative impact is consumed once for both visual and audio
    // presentation. Neither presentation path decides damage, crater shape, or turn handoff.
    commands.spawn((
        Name::new("Terrain impact audio"),
        AudioPlayer(audio.impact.clone()),
        PlaybackSettings {
            volume: Volume::Linear(1.15 * latest_impact.explosion_visual_scale),
            spatial: false,
            ..PlaybackSettings::DESPAWN
        },
        Transform::from_translation(to_bevy_position(impact.position)),
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
            transform.scale = Vec3::splat(
                explosion.scale_multiplier
                    * explosion_scale(explosion_progress(explosion.elapsed_seconds)),
            );
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

#[derive(Resource, Clone)]
struct TankMeshes {
    body: Handle<Mesh>,
    turret: Handle<Mesh>,
    barrel: Handle<Mesh>,
    firing_origin_marker: Handle<Mesh>,
}

#[derive(Resource)]
struct TankPresentationAssets {
    materials: Vec<Handle<StandardMaterial>>,
    firing_origin_material: Handle<StandardMaterial>,
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
    let player_name = format!("Player {} tank", tank.owner.0);

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
    gameplay: (
        Res<Tanks>,
        Res<CurrentTurn>,
        Res<ProjectileFlight>,
        Res<ShotPresentation>,
    ),
    camera: Single<(&mut Transform, &mut BattlefieldCamera)>,
) {
    let (tanks, turn, flight, presentation) = gameplay;
    let (mut transform, mut controller) = camera.into_inner();
    let intent = camera_presentation_intent(
        *presentation,
        turn.0.clone(),
        flight.0.map(|shot| shot.projectile),
    );
    if controller.presentation_intent != Some(intent)
        || matches!(
            intent,
            CameraPresentationIntent::HumanShotFollow(_)
                | CameraPresentationIntent::AiTacticalShot { .. }
        )
    {
        controller.presentation_intent = Some(intent);
        controller.desired_pose = camera_pose_for_intent(intent, tanks.0.clone(), turn.0.clone());
        controller.tracked_aim_yaw = active_aim_yaw(intent, turn.0.clone());
    } else if let Some(aim_yaw) = active_aim_yaw(intent, turn.0.clone()) {
        if let Some(previous_aim_yaw) = controller.tracked_aim_yaw {
            controller.desired_pose.yaw += shortest_angle_delta(previous_aim_yaw, aim_yaw);
        }
        controller.tracked_aim_yaw = Some(aim_yaw);
    }

    if matches!(intent, CameraPresentationIntent::ActivePlayer(_))
        && mouse_buttons.pressed(MouseButton::Right)
    {
        // Mouse motion already represents the full movement since the prior frame.
        controller.yaw -= mouse_motion.delta.x * CAMERA_ORBIT_SENSITIVITY;
        controller.pitch =
            clamp_camera_pitch(controller.pitch - mouse_motion.delta.y * CAMERA_ORBIT_SENSITIVITY);
        controller.desired_pose.yaw = controller.yaw;
        controller.desired_pose.pitch = controller.pitch;
    }

    for wheel in mouse_wheel.read() {
        if !matches!(intent, CameraPresentationIntent::ActivePlayer(_)) {
            continue;
        }
        controller.distance = (controller.distance - wheel.y * CAMERA_ZOOM_SPEED)
            .clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
        controller.desired_pose.distance = controller.distance;
    }

    // Camera interpolation is presentation-only. No gameplay or fixed-update system reads this
    // controller, so a slow transition can never delay input, projectile resolution, or handoff.
    interpolate_camera_pose(&mut controller, time.delta_secs());

    *transform = camera_transform(&controller);
}

fn clamp_camera_pitch(pitch: f32) -> f32 {
    pitch.clamp(CAMERA_MIN_PITCH, CAMERA_MAX_PITCH)
}

fn camera_presentation_intent(
    presentation: ShotPresentation,
    turn: impl std::borrow::Borrow<TurnState>,
    flight: Option<Projectile>,
) -> CameraPresentationIntent {
    let turn = turn.borrow();
    match presentation.phase {
        ShotPresentationPhase::PlayerView => {
            CameraPresentationIntent::ActivePlayer(turn.current_player)
        }
        ShotPresentationPhase::Flight {
            mode: ShotPresentationMode::HumanFollow,
            ..
        } => CameraPresentationIntent::HumanShotFollow(
            flight.expect("flight presentation needs projectile"),
        ),
        ShotPresentationPhase::Flight {
            mode: ShotPresentationMode::AiTactical,
            shooter,
            ..
        } => CameraPresentationIntent::AiTacticalShot {
            shooter,
            projectile: flight.expect("flight presentation needs projectile"),
        },
        ShotPresentationPhase::Impact { position, .. } => {
            CameraPresentationIntent::Impact(position)
        }
        ShotPresentationPhase::Result { position } => CameraPresentationIntent::Result(position),
    }
}

fn active_aim_yaw(intent: CameraPresentationIntent, turn: TurnState) -> Option<f32> {
    match intent {
        CameraPresentationIntent::ActivePlayer(player) => {
            Some(-turn.aim_for(player).azimuth_degrees.to_radians())
        }
        _ => None,
    }
}

fn camera_pose_for_intent(
    intent: CameraPresentationIntent,
    tanks: impl AsRef<[Tank]>,
    turn: impl std::borrow::Borrow<TurnState>,
) -> CameraPose {
    let tanks = tanks.as_ref();
    let turn = turn.borrow();
    match intent {
        CameraPresentationIntent::ActivePlayer(player) => {
            let tank = tank_for_player(tanks, player);
            CameraPose {
                target: clamp_camera_target(
                    to_bevy_position(tank.pose.position) + Vec3::Y * ACTIVE_PLAYER_CAMERA_HEIGHT,
                ),
                yaw: active_aim_yaw(intent, turn.clone())
                    .expect("an active-player camera intent must have aiming yaw"),
                pitch: ACTIVE_PLAYER_CAMERA_PITCH,
                distance: ACTIVE_PLAYER_CAMERA_DISTANCE,
            }
        }
        CameraPresentationIntent::HumanShotFollow(projectile) => human_shot_camera_pose(projectile),
        CameraPresentationIntent::AiTacticalShot {
            shooter,
            projectile,
        } => ai_tactical_camera_pose(shooter, projectile, tanks),
        CameraPresentationIntent::Impact(position) => impact_camera_pose(position),
        CameraPresentationIntent::Result(position) => impact_camera_pose(position),
    }
}

fn human_shot_camera_pose(projectile: Projectile) -> CameraPose {
    let velocity = Vec3::new(projectile.velocity.x, 0.0, projectile.velocity.z);
    let forward = velocity.normalize_or_zero();
    let forward = if forward.length_squared() > 0.0 {
        forward
    } else {
        Vec3::NEG_Z
    };
    let apex = projectile.velocity.y <= 0.0;
    CameraPose {
        target: clamp_camera_target(
            to_bevy_position(projectile.position)
                + forward * HUMAN_LOOK_AHEAD
                + Vec3::Y * HUMAN_SHOT_HEIGHT,
        ),
        yaw: -forward.x.atan2(-forward.z),
        pitch: if apex { -0.52 } else { -0.38 },
        distance: if apex {
            HUMAN_APEX_DISTANCE
        } else {
            HUMAN_SHOT_DISTANCE
        },
    }
}

fn ai_tactical_camera_pose(
    shooter: PlayerId,
    projectile: Projectile,
    tanks: &[Tank],
) -> CameraPose {
    let shooter_position = to_bevy_position(tank_for_player(tanks, shooter).pose.position);
    let projectile_position = to_bevy_position(projectile.position);
    let direction = (projectile_position - shooter_position).normalize_or_zero();
    let direction = if direction.length_squared() > 0.0 {
        direction
    } else {
        Vec3::NEG_Z
    };
    CameraPose {
        target: clamp_camera_target(
            (shooter_position + projectile_position) * 0.5 + Vec3::Y * AI_TACTICAL_HEIGHT,
        ),
        yaw: -direction.x.atan2(-direction.z),
        pitch: SHOT_CAMERA_PITCH,
        distance: AI_TACTICAL_DISTANCE,
    }
}

fn impact_camera_pose(position: WorldPosition) -> CameraPose {
    CameraPose {
        target: clamp_camera_target(to_bevy_position(position) + Vec3::Y * IMPACT_CAMERA_HEIGHT),
        yaw: 0.0,
        pitch: SHOT_CAMERA_PITCH,
        distance: IMPACT_CAMERA_DISTANCE,
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
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, terrain.mesh_colours())
    .with_inserted_indices(Indices::U32(terrain_mesh_indices()))
    .with_computed_smooth_normals()
}

fn create_horizon_mesh(horizon: &VisualHorizon) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, horizon.positions().to_vec())
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, horizon.colours().to_vec())
    .with_inserted_indices(Indices::U32(horizon.indices().to_vec()))
    .with_computed_smooth_normals()
}

fn sky_colour() -> Color {
    Color::srgb(0.26, 0.55, 0.86)
}

fn cloud_positions() -> [(Vec3, Vec3); 6] {
    [
        (Vec3::new(-76.0, 24.0, -76.0), Vec3::new(11.0, 2.4, 5.0)),
        (Vec3::new(-63.0, 25.0, -72.0), Vec3::new(7.0, 1.8, 3.5)),
        (Vec3::new(72.0, 30.0, -92.0), Vec3::new(12.0, 2.5, 5.5)),
        (Vec3::new(87.0, 30.5, -90.0), Vec3::new(7.0, 1.8, 3.5)),
        (Vec3::new(-118.0, 26.0, 60.0), Vec3::new(11.0, 2.2, 4.5)),
        (Vec3::new(105.0, 28.0, 76.0), Vec3::new(9.0, 2.0, 4.0)),
    ]
}

fn spawn_clouds(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mesh = meshes.add(Sphere::new(1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.94, 0.97, 1.0),
        emissive: Color::srgb(0.94, 0.97, 1.0).into(),
        unlit: true,
        ..default()
    });
    for (position, scale) in cloud_positions() {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(position).with_scale(scale),
        ));
    }
}

fn spawn_buildings(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    buildings: &[BuildingPlacement],
    material: Handle<StandardMaterial>,
) {
    for building in buildings {
        commands.spawn((
            BuildingVisual,
            Mesh3d(meshes.add(Cuboid::new(building.width, building.height, building.depth))),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(
                building.position.x,
                building.position.y + building.height / 2.0,
                building.position.z,
            )
            .with_rotation(Quat::from_rotation_y(building.yaw_radians)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battlefield::Crater;
    use crate::tank::{initial_tanks, initial_tanks_for_players};

    fn basic_fired_shot(projectile: Projectile) -> FiredShot {
        FiredShot::new(weapon_definition(WeaponId::BasicShell), projectile)
    }

    #[test]
    fn configured_names_drive_active_and_winner_hud_text() {
        let mut configuration = MatchConfiguration::default();
        configuration.set_name(PlayerId::One, "Ada").unwrap();
        let choosing = TacticalHudView {
            active_player: Some(PlayerId::One),
            action: HudAction::Choose,
            scoreboard: Vec::new(),
            aim: None,
            weapon: None,
            wind: Wind::new(WorldVector::ZERO).unwrap(),
            movement: None,
            result: MatchState::InProgress,
        };
        assert_eq!(
            configured_match_text(&choosing, &configuration),
            "Ada - CHOOSE ACTION"
        );
        let winner = TacticalHudView {
            result: MatchState::Winner(PlayerId::One),
            ..choosing
        };
        assert_eq!(configured_match_text(&winner, &configuration), "Ada WINS");
    }

    #[test]
    fn scoreboard_entries_follow_configured_players_for_two_three_and_eight_player_matches() {
        for count in [2, 3, 8] {
            let mut configuration = MatchConfiguration::with_player_count(count).unwrap();
            for player in &configuration.players.clone() {
                configuration
                    .set_name(player.id, &format!("Commander {}", player.id.0))
                    .unwrap();
            }
            let ids = configuration
                .players
                .iter()
                .map(|player| player.id)
                .collect::<Vec<_>>();
            let mut tanks = initial_tanks_for_players(&BattlefieldTerrain::initial(), &ids);
            tanks[count - 1].health = 0;
            let turn = initial_turn_state(&tanks);
            let entries = scoreboard_entries(&configuration, &tanks, &turn);

            assert_eq!(entries.len(), count);
            assert_eq!(
                entries.iter().map(|entry| entry.player).collect::<Vec<_>>(),
                ids
            );
            assert_eq!(entries[0].display_name, "Commander 1");
            assert_eq!(entries[count - 1].health, Some(0));
            assert!(entries[count - 1].eliminated);
            assert!(entries[0].active);

            let mut resolving = turn.clone();
            assert!(resolving.begin_fire().is_some());
            assert!(
                scoreboard_entries(&configuration, &tanks, &resolving)
                    .iter()
                    .all(|entry| !entry.active)
            );
        }
    }

    #[test]
    fn scoreboard_keeps_new_setup_slots_visible_before_their_tanks_are_generated() {
        let configuration = MatchConfiguration::with_player_count(8).unwrap();
        let tanks = initial_tanks(&BattlefieldTerrain::initial());
        let turn = initial_turn_state(tanks);

        let entries = scoreboard_entries(&configuration, &tanks, &turn);

        assert_eq!(entries.len(), 8);
        assert_eq!(entries[0].health, Some(MAX_HEALTH));
        assert_eq!(entries[1].health, Some(MAX_HEALTH));
        assert!(entries[0].active);
        for entry in &entries[2..] {
            assert_eq!(entry.health, None);
            assert!(!entry.eliminated);
            assert!(!entry.active);
            assert_eq!(
                scoreboard_entry_text(entry),
                format!("{}: READY", entry.display_name)
            );
        }
    }

    #[test]
    fn scoreboard_layout_keeps_duels_roomy_and_large_matches_compact() {
        assert_eq!(scoreboard_layout(2), (16.0, 8.0, 3.0));
        assert_eq!(scoreboard_layout(4), (16.0, 8.0, 3.0));
        assert_eq!(scoreboard_layout(8), (13.0, 6.0, 2.0));
    }

    #[test]
    fn sky_clouds_horizon_and_camera_safety_have_a_small_presentation_plan() {
        assert_eq!(sky_colour(), Color::srgb(0.26, 0.55, 0.86));
        assert!(!cloud_positions().is_empty());
        assert_eq!(clamp_camera_pitch(-10.0), CAMERA_MIN_PITCH);
        assert_eq!(clamp_camera_pitch(10.0), CAMERA_MAX_PITCH);
        let terrain = BattlefieldTerrain::generated(BattlefieldSeed(17));
        let horizon = VisualHorizon::from_terrain(&terrain, BattlefieldSeed(17));
        assert!(!horizon.positions().is_empty());
        assert!(create_horizon_mesh(&horizon).count_vertices() > 0);
    }

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
            clamp_camera_target(Vec3::new(90.0, 0.0, -90.0)),
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
        let configuration = MatchConfiguration::default();
        let wind = Wind::new(WorldVector {
            x: 1.5,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let choosing = initial_turn_state(tanks);
        let choosing_view = tactical_hud_view(
            choosing.clone(),
            tanks,
            &configuration,
            PlayerWeaponLoadouts::default(),
            wind,
            None,
        );
        assert_eq!(choosing_view.active_player, Some(PlayerId::One));
        assert_eq!(choosing_view.action, HudAction::Choose);
        assert_eq!(choosing_view.scoreboard.len(), 2);
        assert!(choosing_view.scoreboard[0].active);
        assert_eq!(choosing_view.aim, Some(choosing.current_aim()));
        assert_eq!(choosing_view.movement, None);

        let mut moving = choosing.clone();
        assert!(moving.begin_movement());
        let moving_view = tactical_hud_view(
            moving.clone(),
            tanks,
            &configuration,
            PlayerWeaponLoadouts::default(),
            wind,
            Some(MovementRejection::Slope),
        );
        assert_eq!(moving_view.action, HudAction::Moving);
        assert_eq!(
            moving_view.movement,
            Some((
                moving.remaining_movement().unwrap(),
                Some(MovementRejection::Slope)
            ))
        );

        let mut resolving = choosing.clone();
        assert!(resolving.begin_fire().is_some());
        let resolving_view = tactical_hud_view(
            resolving.clone(),
            tanks,
            &configuration,
            PlayerWeaponLoadouts::default(),
            wind,
            None,
        );
        assert_eq!(resolving_view.action, HudAction::Resolving);
        assert_eq!(resolving_view.aim, Some(resolving.current_aim()));
    }

    #[test]
    fn hud_shows_the_active_players_selected_weapon_and_truthful_ammunition() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let configuration = MatchConfiguration::default();
        let wind = Wind::new(WorldVector::ZERO).unwrap();
        let turn = initial_turn_state(tanks);
        let mut weapons = PlayerWeaponLoadouts::default();

        assert!(
            weapons
                .for_player_mut(PlayerId::One)
                .select(WeaponId::HighExplosive)
        );
        let view = tactical_hud_view(
            turn.clone(),
            tanks,
            &configuration,
            weapons.clone(),
            wind,
            None,
        );

        assert_eq!(
            view.weapon,
            Some((WeaponId::HighExplosive, WeaponAvailability::Remaining(2)))
        );
        assert_eq!(
            hud_field_text(HudTextField::Weapon, &view),
            "HIGH EXPLOSIVE: x2"
        );
        assert_eq!(
            hud_field_text(HudTextField::Controls, &view),
            "1 BASIC | 2 HE | 3 HEAVY | M MOVE | SPACE FIRE\nARROWS AIM | -/= POWER"
        );

        assert!(
            weapons
                .for_player_mut(PlayerId::One)
                .select(WeaponId::HeavyShell)
        );
        let heavy_view =
            tactical_hud_view(turn.clone(), tanks, &configuration, weapons, wind, None);
        assert_eq!(
            heavy_view.weapon,
            Some((WeaponId::HeavyShell, WeaponAvailability::Remaining(2)))
        );
        assert_eq!(
            hud_field_text(HudTextField::Weapon, &heavy_view),
            "HEAVY SHELL: x2"
        );
    }

    #[test]
    fn conventional_weapons_use_the_common_resolver_with_their_captured_impact_profiles() {
        let impact_position = WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let projectile =
            Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(impact_position));

        let mut basic_terrain = BattlefieldTerrain::initial();
        let mut basic_tanks = initial_tanks(&basic_terrain);
        basic_tanks[0].pose.position = impact_position;
        let mut basic_turn = initial_turn_state(basic_tanks);
        let mut basic_latest_impact = LatestTerrainImpact::default();
        assert!(basic_turn.begin_fire().is_some());
        resolve_projectile_advance(
            basic_fired_shot(projectile),
            ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: impact_position,
            }),
            &mut basic_terrain,
            &mut basic_tanks,
            &mut basic_latest_impact,
            &mut basic_turn,
        );

        let mut heavy_terrain = BattlefieldTerrain::initial();
        let mut heavy_tanks = initial_tanks(&heavy_terrain);
        heavy_tanks[0].pose.position = impact_position;
        let mut heavy_turn = initial_turn_state(heavy_tanks);
        let mut heavy_latest_impact = LatestTerrainImpact::default();
        assert!(heavy_turn.begin_fire().is_some());
        resolve_projectile_advance(
            FiredShot::new(weapon_definition(WeaponId::HeavyShell), projectile),
            ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: impact_position,
            }),
            &mut heavy_terrain,
            &mut heavy_tanks,
            &mut heavy_latest_impact,
            &mut heavy_turn,
        );
        assert_eq!(heavy_tanks[0].health, basic_tanks[0].health);
        assert_eq!(
            heavy_terrain.height(0.0, 0.0),
            basic_terrain.height(0.0, 0.0)
        );
        assert_eq!(heavy_latest_impact.explosion_visual_scale, 1.0);
        assert!(heavy_tanks[0].is_settling());

        let mut he_terrain = BattlefieldTerrain::initial();
        let mut he_tanks = initial_tanks(&he_terrain);
        he_tanks[0].pose.position = impact_position;
        let mut he_turn = initial_turn_state(he_tanks);
        let mut he_latest_impact = LatestTerrainImpact::default();
        assert!(he_turn.begin_fire().is_some());
        resolve_projectile_advance(
            FiredShot::new(weapon_definition(WeaponId::HighExplosive), projectile),
            ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: impact_position,
            }),
            &mut he_terrain,
            &mut he_tanks,
            &mut he_latest_impact,
            &mut he_turn,
        );

        assert!(he_tanks[0].health < basic_tanks[0].health);
        assert!(he_terrain.height(0.0, 0.0) < basic_terrain.height(0.0, 0.0));
        assert_eq!(he_latest_impact.explosion_visual_scale, 1.5);
        assert!(he_tanks[0].is_settling());

        let mut third_terrain = BattlefieldTerrain::initial();
        let mut third_tanks = initial_tanks(&third_terrain);
        let mut third_turn = initial_turn_state(third_tanks);
        let mut third_latest_impact = LatestTerrainImpact::default();
        assert!(third_turn.begin_fire().is_some());
        resolve_projectile_advance(
            FiredShot::new(weapon::test_conventional_definition(), projectile),
            ProjectileAdvance::TerrainImpact(TerrainImpact {
                position: impact_position,
            }),
            &mut third_terrain,
            &mut third_tanks,
            &mut third_latest_impact,
            &mut third_turn,
        );
        assert!(third_terrain.height(0.0, 0.0) < BattlefieldTerrain::initial().height(0.0, 0.0));
        assert_eq!(third_latest_impact.explosion_visual_scale, 0.8);
    }

    #[test]
    fn wind_arrow_preserves_world_direction_while_its_camera_follows_the_turret() {
        let calm = Wind::new(WorldVector::ZERO).unwrap();
        let positive_x = Wind::new(WorldVector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        })
        .unwrap();
        let north = wind_indicator_camera_transform(Some(AimingState::new(0.0, 45.0, 18.0)));
        let east = wind_indicator_camera_transform(Some(AimingState::new(90.0, 45.0, 18.0)));

        assert_eq!(wind_arrow_rotation(calm), 0.0);
        assert!((wind_arrow_rotation(positive_x)).abs() < 0.01);
        assert!(north.translation.z > 6.0);
        assert!(east.translation.x < -6.0);
    }

    #[test]
    fn hud_text_hides_contextual_controls_after_actions_resolve() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let configuration = MatchConfiguration::default();
        let wind = Wind::new(WorldVector::ZERO).unwrap();
        let choosing = tactical_hud_view(
            initial_turn_state(tanks),
            tanks,
            &configuration,
            PlayerWeaponLoadouts::default(),
            wind,
            None,
        );
        assert!(hud_field_text(HudTextField::Controls, &choosing).contains("M MOVE"));

        let mut resolving_turn = initial_turn_state(tanks);
        assert!(resolving_turn.begin_fire().is_some());
        let resolving = tactical_hud_view(
            resolving_turn,
            tanks,
            &configuration,
            PlayerWeaponLoadouts::default(),
            wind,
            None,
        );
        assert!(hud_field_text(HudTextField::Controls, &resolving).is_empty());
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
            camera_presentation_intent(ShotPresentation::default(), &turn, None),
            CameraPresentationIntent::ActivePlayer(PlayerId::One)
        );

        let projectile = Projectile::launch(turn.current_aim().shot_parameters(WorldPosition {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        }));
        assert_eq!(
            camera_presentation_intent(
                ShotPresentation {
                    phase: ShotPresentationPhase::Flight {
                        mode: ShotPresentationMode::HumanFollow,
                        shooter: PlayerId::One,
                        apex_seen: false,
                    },
                    ..default()
                },
                &turn,
                Some(projectile),
            ),
            CameraPresentationIntent::HumanShotFollow(projectile)
        );

        assert!(turn.begin_movement());
        assert!(turn.finish_movement([true, true]));
        assert_eq!(
            camera_presentation_intent(ShotPresentation::default(), turn, None),
            CameraPresentationIntent::ActivePlayer(PlayerId::Two)
        );
    }

    #[test]
    fn controller_type_selects_shot_presentation_not_display_name() {
        let mut configuration = MatchConfiguration::default();
        configuration
            .set_name(PlayerId::One, "Crater Kate")
            .unwrap();
        assert_eq!(
            shot_mode_for_player(&configuration, PlayerId::One),
            ShotPresentationMode::HumanFollow
        );
        configuration.set_controller(PlayerId::One, ControllerType::Ai);
        assert_eq!(
            shot_mode_for_player(&configuration, PlayerId::One),
            ShotPresentationMode::AiTactical
        );
    }

    #[test]
    fn human_apex_widens_without_mutating_the_projectile() {
        let parameters = AimingState::new(0.0, 45.0, 18.0).shot_parameters(WorldPosition {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        });
        let climbing = Projectile::launch(parameters);
        let mut descending = climbing;
        descending.velocity.y = -0.1;
        assert_eq!(
            human_shot_camera_pose(climbing).distance,
            HUMAN_SHOT_DISTANCE
        );
        assert_eq!(
            human_shot_camera_pose(descending).distance,
            HUMAN_APEX_DISTANCE
        );
        assert_eq!(climbing.velocity.y, parameters.launch_velocity().y);
    }

    #[test]
    fn human_and_ai_flights_converge_on_one_impact_intent_and_final_stays_result() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let turn = initial_turn_state(tanks);
        let impact = WorldPosition {
            x: 4.0,
            y: 2.0,
            z: -3.0,
        };
        let shared = ShotPresentation {
            phase: ShotPresentationPhase::Impact {
                position: impact,
                elapsed_seconds: 0.0,
            },
            ..default()
        };
        assert_eq!(
            camera_presentation_intent(shared, &turn, None),
            CameraPresentationIntent::Impact(impact)
        );
        let final_presentation = ShotPresentation {
            phase: ShotPresentationPhase::Result { position: impact },
            ..default()
        };
        assert_eq!(
            camera_presentation_intent(final_presentation, &turn, None),
            CameraPresentationIntent::Result(impact)
        );
    }

    #[test]
    fn tactical_pose_is_broader_and_all_ai_safe() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let projectile = Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
        ));
        let human = human_shot_camera_pose(projectile);
        let ai = ai_tactical_camera_pose(PlayerId::One, projectile, &tanks);
        assert!(ai.distance > human.distance);
        assert!(ai.target.x.abs() <= HALF_EXTENT);
        assert!(ai.target.z.abs() <= HALF_EXTENT);
    }

    #[test]
    fn camera_pose_is_bounded_and_transition_does_not_change_turn_state() {
        let terrain = BattlefieldTerrain::initial();
        let tanks = initial_tanks(&terrain);
        let turn = initial_turn_state(tanks);
        let pose = camera_pose_for_intent(
            CameraPresentationIntent::ActivePlayer(PlayerId::One),
            tanks,
            &turn,
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
        let before = camera_pose_for_intent(intent, tanks, &turn);
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
        assert!(turn.accept_movement_step([true, true]));
        assert!(turn.finish_movement([true, true]));
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
        let mut impact = LatestTerrainImpact::default();
        let mut turn = TurnState::new(vec![
            (PlayerId::One, AimingState::new(0.0, 45.0, 18.0)),
            (PlayerId::Two, AimingState::new(180.0, 45.0, 18.0)),
        ]);
        let projectile = Projectile::launch(AimingState::new(0.0, 45.0, 18.0).shot_parameters(
            WorldPosition {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
        ));

        turn.begin_fire();
        let active = resolve_projectile_advance(
            basic_fired_shot(projectile),
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
            basic_fired_shot(projectile),
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
        assert_eq!(impact.impact.unwrap().position.y, before);
        assert!(terrain.height(0.0, 0.0) < before);
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Choosing);
    }

    #[test]
    fn wind_derived_terrain_impact_uses_the_existing_damage_crater_and_handoff_pipeline() {
        let mut terrain = BattlefieldTerrain::initial();
        let mut tanks = initial_tanks(&terrain);
        let mut latest_impact = LatestTerrainImpact::default();
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
                basic_fired_shot(projectile),
                ProjectileAdvance::TerrainImpact(impact),
                &mut terrain,
                &mut tanks,
                &mut latest_impact,
                &mut turn,
            )
            .is_none()
        );

        assert_eq!(latest_impact.impact, Some(impact));
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
        let mut impact = LatestTerrainImpact::default();
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
                basic_fired_shot(projectile),
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
        let mut impact = LatestTerrainImpact::default();
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
            basic_fired_shot(projectile),
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
        let mut impact = LatestTerrainImpact::default();
        let mut turn = TurnState::new(vec![
            (PlayerId::One, AimingState::new(0.0, 45.0, 18.0)),
            (PlayerId::Two, AimingState::new(180.0, 45.0, 18.0)),
        ]);
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
                basic_fired_shot(projectile),
                ProjectileAdvance::OutOfBounds,
                &mut terrain,
                &mut tanks,
                &mut impact,
                &mut turn,
            )
            .is_none()
        );
        assert!(impact.impact.is_none());
        assert_eq!(terrain.height(0.0, 0.0), before);
        assert_eq!(turn.current_player, PlayerId::Two);
        assert_eq!(turn.phase, TurnPhase::Choosing);
        assert!(!turn.complete_fire_resolution([true, true]));
    }
}
