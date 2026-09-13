mod ai;
mod aiming;
mod battlefield;
mod combat;
mod environment;
mod match_setup;
mod projectile;
#[allow(dead_code)]
mod session;
mod tank;
mod turn;
mod weapon;
mod world;

use std::time::{SystemTime, UNIX_EPOCH};

use ai::{decide_firing_with_terrain, planned_shop_purchases};
use aiming::{AimAdjustment, AimingState};
use battlefield::{
    BattlefieldSeed, BattlefieldTerrain, BuildingPlacement, HALF_EXTENT, VisualHorizon,
    WATER_TABLE, generate_buildings, terrain_mesh_indices,
};
use bevy::{
    asset::RenderAssetUsages,
    audio::{AudioPlayer, AudioSource, PlaybackSettings, SpatialListener, Volume},
    camera::{Viewport, visibility::RenderLayers},
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    mesh::Indices,
    pbr::{DistanceFog, FogFalloff},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use combat::resolve_explosion;
use environment::{EnvironmentPreset, PaletteProfile, SkyProfile, WindPolicy};
use match_setup::{ControllerType, MatchConfiguration, PlayerConfiguration};
use projectile::{
    Gravity, Projectile, ProjectileAdvance, SimulationLimits, TerrainImpact, Wind,
    azimuth_from_horizontal_direction,
};
use session::{GameSession, SessionPhase, weapon_shop_items};
use tank::{
    HorizontalDirection, MAX_HEALTH, MovementDirection, MovementRejection, PlayerId, Tank,
    TankFiringRepresentation, initial_tanks_for_players_seeded,
};
use turn::{MatchState, TurnPhase, TurnState};
use weapon::{
    CLUSTER_BOMB_CHILD_COUNT, ContactState, FiredShot, MIRV_CHILD_COUNT, PlayerWeaponLoadouts,
    WeaponAvailability, WeaponId, weapon_definition, weapon_price,
};
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
#[cfg(test)]
const DEVELOPMENT_GRAVITY: f32 = 8.0;
#[cfg(test)]
const MINIMUM_WIND_STRENGTH: f32 = 0.75;
#[cfg(test)]
const MAXIMUM_WIND_STRENGTH: f32 = 1.75;
/// The simulation's wind value is an acceleration.  The HUD presents the same relative
/// intensity on a familiar, player-facing kilometres-per-hour scale.
const WIND_KPH_PER_ACCELERATION: f32 = 10.0;
const WIND_INDICATOR_RENDER_LAYER: usize = 1;
const WIND_INDICATOR_WIDTH: u32 = 150;
const WIND_INDICATOR_HEIGHT: u32 = 52;
const WIND_INDICATOR_LIGHT_INTENSITY: f32 = 150_000.0;
const WIND_ENABLED: bool = true;
const EXPLOSION_VISUAL_DURATION_SECONDS: f32 = 0.6;
const EXPLOSION_INITIAL_SCALE: f32 = 0.35;
const EXPLOSION_MAXIMUM_SCALE: f32 = 5.0;
const IMPACT_FLASH_BASE_DURATION_SECONDS: f32 = 0.22;
const IMPACT_FLASH_BASE_OPACITY: f32 = 0.52;
const MAX_ACTIVE_IMPACT_PARTICLES: usize = 128;
const MAX_ACTIVE_SMOKE_PUFFS: usize = 32;
const MAX_PENDING_IMPACT_EFFECTS: usize = 64;
const PARTICLE_GRAVITY: f32 = 7.0;

// A deliberately small visual language: these values are shared by the HUD and every overlay,
// but layouts remain local so this does not become a CSS framework in Rust.
const UI_OVERLAY: Color = Color::srgba(0.015, 0.025, 0.05, 0.90);
const UI_PANEL: Color = Color::srgba(0.035, 0.065, 0.105, 0.94);
const UI_ELEVATED: Color = Color::srgba(0.075, 0.12, 0.18, 0.98);
const UI_BORDER: Color = Color::srgb(0.28, 0.46, 0.66);
const UI_ACCENT: Color = Color::srgb(1.0, 0.72, 0.18);
const UI_TEXT: Color = Color::srgb(0.94, 0.92, 0.82);
const UI_MUTED: Color = Color::srgb(0.53, 0.62, 0.70);
const UI_DISABLED: Color = Color::srgb(0.22, 0.26, 0.30);
const UI_SUCCESS: Color = Color::srgb(0.18, 0.58, 0.36);

fn ui_button_shadow() -> BoxShadow {
    BoxShadow::new(
        Color::srgba(0.0, 0.0, 0.0, 0.48),
        px(0),
        px(3),
        px(0),
        px(3),
    )
}

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
    MovementPlayer(PlayerId),
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

#[derive(Resource, Clone)]
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
    hit_tank: bool,
    effect_requests: Vec<ImpactPresentationRequest>,
    next_effect_serial: u64,
}

impl Default for LatestTerrainImpact {
    fn default() -> Self {
        Self {
            impact: None,
            explosion_visual_scale: 1.0,
            hit_tank: false,
            effect_requests: Vec::new(),
            next_effect_serial: 0,
        }
    }
}

/// Records whether the persistent latest impact has already created its one presentation effect.
/// The impact remains available for the diagnostic marker until the next shot clears it.
#[derive(Resource, Default)]
struct CurrentImpactExplosionConsumed(bool);

#[derive(Resource, Default)]
struct ImpactFlash(Option<(f32, f32, bool)>);

#[derive(Resource)]
struct BattlefieldGravity(Gravity);

/// Match-constant, projectile-only wind. Tank movement and settling intentionally consume no
/// wind because this is an artillery aiming variable, not vehicle physics.
#[derive(Resource)]
struct BattlefieldWind(Wind);

#[derive(Resource, Default)]
struct TurnwindState {
    last_player: Option<PlayerId>,
    handoff_ordinal: u64,
}

#[derive(Resource)]
struct SkyPresentation(SkyProfile);

#[derive(Resource, Default)]
struct MovementFeedback(Option<MovementRejection>);

/// Space completes movement, but normally also fires. This frame-local latch prevents the
/// completion press from being interpreted as the newly active player's shot.
#[derive(Resource, Default)]
struct MovementCompletionThisFrame(bool);

/// Captures the hull orientation on entering movement mode so repeated arrow presses remain in
/// one player-facing frame even though each successful step updates the tank's body direction.
#[derive(Resource, Default)]
struct MovementFrame(Option<HorizontalDirection>);

/// Keyboard turret adjustments deliberately end free camera exploration. This presentation-only
/// latch restores the behind-the-tank aiming view on the next camera update.
#[derive(Resource, Default)]
struct CameraAimReset(bool);

/// The existing tactical scene is prepared behind setup so the start boundary is explicit:
/// configuration is chosen before any human action can mutate authoritative match state.
#[derive(Resource, Default)]
struct MatchSetupGate {
    started: bool,
    selected_slot: usize,
}

#[derive(Resource)]
struct PendingMatchConfiguration(MatchConfiguration);

/// A shot snapshot brackets authoritative resolution so currency is based on health that was
/// actually removed after saturation, rather than a weapon's advertised damage.
#[derive(Resource, Default)]
struct DamageAccounting {
    owner: Option<PlayerId>,
    health_before: Vec<(PlayerId, u8)>,
}

#[derive(Component)]
struct MatchSetupOverlay;
#[derive(Component)]
struct MatchSetupDetails;

#[derive(Resource)]
struct ProjectileVisualAssets {
    mesh: Handle<Mesh>,
    basic_material: Handle<StandardMaterial>,
    high_explosive_material: Handle<StandardMaterial>,
    heavy_material: Handle<StandardMaterial>,
    mirv_carrier_material: Handle<StandardMaterial>,
    mirv_child_material: Handle<StandardMaterial>,
    cluster_material: Handle<StandardMaterial>,
    roller_material: Handle<StandardMaterial>,
    bunker_material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct AudioAssets {
    fire: Handle<AudioSource>,
    impact: Handle<AudioSource>,
    turret_dink: Handle<AudioSource>,
    purchase: Handle<AudioSource>,
}

/// Bevy handles stay at the presentation boundary; `weapon::WeaponPresentation` only exposes
/// stable paths so the same artwork can be used by both the HUD and shop.
#[derive(Resource, Clone)]
struct WeaponIconAssets {
    icons: std::collections::HashMap<WeaponId, Handle<Image>>,
}

impl WeaponIconAssets {
    fn icon(&self, weapon: WeaponId) -> Handle<Image> {
        self.icons
            .get(&weapon)
            .cloned()
            .unwrap_or_else(|| panic!("missing required UI icon for {weapon:?}"))
    }
}

#[derive(Resource, Default)]
struct TurretDinkCooldown(f32);

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

#[derive(Clone, Copy, Debug, PartialEq)]
struct ImpactPresentationRequest {
    position: WorldPosition,
    scale: f32,
    serial: u64,
}

#[derive(Resource)]
struct ImpactEffectAssets {
    particle_mesh: Handle<Mesh>,
    smoke_mesh: Handle<Mesh>,
    hot_material: Handle<StandardMaterial>,
    dirt_material: Handle<StandardMaterial>,
    smoke_material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct WorldDressingAssets {
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct TerrainTextureAssets {
    earth: Handle<Image>,
    moon: Handle<Image>,
    crusher: Handle<Image>,
}

#[derive(Resource)]
struct TerrainPresentation(PaletteProfile);

#[derive(Component)]
struct ProjectileVisual;

#[derive(Component)]
struct ImpactMarker;

#[derive(Component)]
struct BattlefieldVisual;

#[derive(Component)]
struct HorizonVisual;
#[derive(Component)]
struct CloudVisual;

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
struct ImpactParticle {
    velocity: Vec3,
    elapsed_seconds: f32,
    lifetime_seconds: f32,
    initial_scale: f32,
}

#[derive(Component)]
struct SmokePuff {
    velocity: Vec3,
    elapsed_seconds: f32,
    lifetime_seconds: f32,
    initial_scale: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TankVisualModel {
    Classic,
    Heavy,
    LowProfile,
    Compact,
    Angular,
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
struct SessionFlowOverlay;

#[derive(Component)]
struct SessionFlowText;
#[derive(Component)]
struct ShopPanel;
#[derive(Component)]
struct ShopTitle;
#[derive(Component)]
struct ShopCardText(WeaponId);
#[derive(Component)]
struct ShopBuy(WeaponId);
#[derive(Component)]
struct ShopDone;

#[derive(Component)]
struct ImpactFlashOverlay;

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

/// A presentation button maps to one stable gameplay weapon identity. It never owns ammunition
/// or selection state; the HUD merely projects the current player's loadout into clickable slots.
#[derive(Component)]
struct WeaponSlot(WeaponId);

#[derive(Component)]
struct WeaponSlotAmmo(WeaponId);

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
    movement: Option<MovementRejection>,
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
    (
        Without<HudHealthFill>,
        Without<HudScoreboardRow>,
        Without<WeaponSlot>,
    ),
>;
type ShopDoneInteractions<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (With<ShopDone>, Without<ShopBuy>),
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
    ResMut<'w, MatchSeed>,
    ResMut<'w, WorldDressing>,
    ResMut<'w, BattlefieldWind>,
    ResMut<'w, AiDecisionSeed>,
    ResMut<'w, GameSession>,
    ResMut<'w, ShotPresentation>,
    ResMut<'w, CameraAimReset>,
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
    let (terrain, tanks, dressing, wind) = generate_match_world(
        round_seed(match_seed.0, 1),
        &player_ids,
        configuration.environment,
    );
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
        .insert_resource(ImpactFlash::default())
        .insert_resource(MovementFeedback::default())
        .insert_resource(MovementCompletionThisFrame::default())
        .insert_resource(MovementFrame::default())
        .insert_resource(CameraAimReset::default())
        .insert_resource(MatchSetupGate::default())
        .insert_resource(PendingMatchConfiguration(configuration.clone()))
        .insert_resource(GameSession::new(MatchConfiguration::default().players))
        .insert_resource(DamageAccounting::default())
        .insert_resource(AimRepeatState::default())
        .insert_resource(TurretDinkCooldown::default())
        .insert_resource(BattlefieldGravity(
            Gravity::new(configuration.environment.gravity())
                .expect("preset gravity must be valid"),
        ))
        .insert_resource(BattlefieldWind(wind))
        .insert_resource(TurnwindState::default())
        .insert_resource(SkyPresentation(SkyProfile::Clear))
        .insert_resource(Time::<Fixed>::from_hz(PROJECTILE_FIXED_HZ))
        .add_systems(
            Startup,
            (
                load_weapon_icons,
                spawn_battlefield_scene,
                spawn_match_setup,
                spawn_session_flow_overlay,
                spawn_shop_overlay,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    update_shot_presentation,
                    update_battlefield_camera,
                    update_match_setup,
                    select_weapon_input,
                    select_weapon_slot,
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
                    update_impact_flash,
                    update_explosion_visuals,
                    spawn_impact_effects,
                    update_impact_particles,
                    update_smoke_puffs,
                    sync_battlefield_mesh,
                )
                    .chain(),
            )
                .chain(),
        )
        .add_systems(Update, update_turnwind)
        .add_systems(Update, sync_sky_presentation.after(update_match_setup))
        .add_systems(Update, sync_terrain_textures.after(update_match_setup))
        .add_systems(
            Update,
            (
                update_session_flow,
                sync_session_flow_overlay,
                sync_shop_overlay,
                handle_shop_input,
                run_ai_shop,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                begin_damage_accounting,
                advance_projectile,
                credit_resolved_damage,
                advance_tank_settling,
            )
                .chain(),
        )
        .run();
}

fn spawn_battlefield_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    icons: Res<WeaponIconAssets>,
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
        distance_fog_for(configuration.0.environment.sky()),
        camera,
        transform,
    ));
    commands.insert_resource(AudioAssets {
        fire: asset_server.load("audio/fire.ogg"),
        impact: asset_server.load("audio/impact.ogg"),
        turret_dink: asset_server.load("audio/turret-dink.ogg"),
        purchase: asset_server.load("audio/ui-purchase.ogg"),
    });
    let terrain_textures = TerrainTextureAssets {
        earth: load_repeating_terrain_texture(&asset_server, "textures/terrain-ground.png"),
        moon: load_repeating_terrain_texture(&asset_server, "textures/moon-regolith.png"),
        crusher: load_repeating_terrain_texture(&asset_server, "textures/crusher-sand.png"),
    };
    let water_texture = asset_server.load("textures/water-ripples.png");
    let tank_texture = asset_server.load("textures/tank-metal.png");
    spawn_wind_indicator_overlay(&mut commands, &mut meshes, &mut materials);

    commands.spawn((
        BattlefieldVisual,
        Mesh3d(meshes.add(create_battlefield_mesh(
            &terrain.0,
            configuration.0.environment.palette(),
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: terrain_texture_for(
                configuration.0.environment.palette(),
                &terrain_textures,
            ),
            perceptual_roughness: 0.88,
            ..default()
        })),
    ));

    commands.spawn((
        HorizonVisual,
        Mesh3d(meshes.add(create_horizon_mesh(&VisualHorizon::from_terrain(
            &terrain.0,
            BattlefieldSeed(derived_seed(match_seed.0, "terrain")),
            configuration.0.environment.palette(),
        )))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: terrain_texture_for(
                configuration.0.environment.palette(),
                &terrain_textures,
            ),
            perceptual_roughness: 0.88,
            ..default()
        })),
    ));
    spawn_clouds(
        &mut commands,
        &mut meshes,
        &mut materials,
        configuration.0.environment.sky(),
    );

    commands.spawn((
        WaterVisual,
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(HALF_EXTENT * 2.0, HALF_EXTENT * 2.0),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.035, 0.20, 0.62),
            base_color_texture: Some(water_texture),
            perceptual_roughness: 0.24,
            reflectance: 0.65,
            ..default()
        })),
        Transform::from_xyz(0.0, WATER_TABLE, 0.0),
    ));
    let building_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.36, 0.38, 0.40),
        base_color_texture: Some(tank_texture.clone()),
        perceptual_roughness: 0.78,
        ..default()
    });
    spawn_buildings(
        &mut commands,
        &mut meshes,
        &dressing.0,
        building_material.clone(),
    );
    commands.insert_resource(WorldDressingAssets {
        material: building_material,
    });
    commands.insert_resource(TerrainPresentation(configuration.0.environment.palette()));
    commands.insert_resource(terrain_textures);

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
        track: meshes.add(Cuboid::new(0.34, 0.42, 2.25)),
        firing_origin_marker: meshes.add(Sphere::new(0.12)),
    };
    let player_materials = tank_materials(&mut materials, tank_texture.clone());
    let track_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.07, 0.08, 0.08),
        base_color_texture: Some(tank_texture.clone()),
        metallic: 0.35,
        perceptual_roughness: 0.72,
        ..default()
    });
    let barrel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.14, 0.15),
        base_color_texture: Some(tank_texture),
        metallic: 0.65,
        perceptual_roughness: 0.42,
        ..default()
    });
    let firing_origin_material = materials.add(Color::srgb(0.95, 0.85, 0.2));
    commands.insert_resource(TankPresentationAssets {
        materials: player_materials.clone(),
        track_material: track_material.clone(),
        barrel_material: barrel_material.clone(),
        firing_origin_material: firing_origin_material.clone(),
    });
    commands.insert_resource(tank_meshes.clone());

    for tank in tanks.0.iter().copied() {
        spawn_tank(
            &mut commands,
            tank,
            &tank_meshes,
            player_materials[(tank.owner.0.saturating_sub(1) as usize) % player_materials.len()]
                .clone(),
            track_material.clone(),
            barrel_material.clone(),
            firing_origin_material.clone(),
            active_firing_representation(tank, turn.0.aim_for(tank.owner)),
        );
    }

    spawn_tactical_hud(&mut commands, &configuration.0.players, &icons);

    commands.insert_resource(ProjectileVisualAssets {
        // A small, single-piece rocket stays inexpensive for cluster and MIRV salvos while its
        // pointed nose and rear fins keep it from reading as a capsule at tactical distance.
        mesh: meshes.add(rocket_mesh()),
        // Projectiles remain deliberately simple, but hot saturated colours make them read as
        // ordnance rather than friendly UI marbles at normal tactical-camera distance.
        basic_material: projectile_material_asset(&mut materials, Color::srgb(0.95, 0.32, 0.08)),
        high_explosive_material: projectile_material_asset(
            &mut materials,
            Color::srgb(1.0, 0.08, 0.02),
        ),
        heavy_material: projectile_material_asset(&mut materials, Color::srgb(0.20, 0.22, 0.24)),
        mirv_carrier_material: projectile_material_asset(
            &mut materials,
            Color::srgb(0.65, 0.08, 0.08),
        ),
        mirv_child_material: projectile_material_asset(
            &mut materials,
            Color::srgb(0.18, 0.02, 0.02),
        ),
        cluster_material: projectile_material_asset(&mut materials, Color::srgb(1.0, 0.42, 0.02)),
        roller_material: projectile_material_asset(&mut materials, Color::srgb(0.16, 0.30, 0.12)),
        bunker_material: projectile_material_asset(&mut materials, Color::srgb(0.08, 0.09, 0.11)),
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
    commands.insert_resource(ImpactEffectAssets {
        // Deliberately chunky: these need to read from the tactical camera like an old-school
        // artillery firework, rather than disappearing behind the impact marker.
        particle_mesh: meshes.add(Sphere::new(0.32)),
        smoke_mesh: meshes.add(Sphere::new(0.55)),
        hot_material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.72, 0.08),
            emissive: Color::srgb(5.0, 1.55, 0.04).into(),
            ..default()
        }),
        dirt_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.16, 0.055),
            perceptual_roughness: 0.9,
            ..default()
        }),
        smoke_material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.13, 0.15, 0.16, 0.48),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.95,
            ..default()
        }),
    });
}

/// Load once at startup so overlays and the rebuilt HUD share the exact same handles. The lookup
/// is deliberately validated by `WeaponIconAssets::icon` instead of allowing blank UI controls.
fn load_weapon_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(WeaponIconAssets {
        icons: weapon::ACTIVE_WEAPONS
            .into_iter()
            .map(|weapon| {
                (
                    weapon,
                    asset_server.load(weapon::weapon_presentation(weapon).icon_path),
                )
            })
            .collect(),
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
            intensity: WIND_INDICATOR_LIGHT_INTENSITY,
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
                    base_color: Color::srgb(0.18, 0.72, 1.0),
                    emissive: Color::srgb(0.08, 0.38, 0.9).into(),
                    metallic: 0.2,
                    perceptual_roughness: 0.3,
                    ..default()
                })),
                Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                layer.clone(),
            ));
            arrow.spawn((
                Mesh3d(meshes.add(Cone::new(0.48, 1.15))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.84, 0.22),
                    emissive: Color::srgb(0.45, 0.25, 0.02).into(),
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
            BackgroundColor(UI_OVERLAY),
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
                BackgroundColor(UI_PANEL),
                BorderColor::all(UI_ACCENT),
            ))
            .with_children(|panel| {
                setup_text(panel, "AZIMUTH - MATCH SETUP", 30.0, UI_ACCENT);
                panel.spawn((
                    MatchSetupDetails,
                    Text::new(""),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(UI_TEXT),
                ));
                setup_text(
                    panel,
                    "2-8: COUNT | UP/DOWN: SLOT | E: ENVIRONMENT | TAB: HUMAN/AI | D: AI LEVEL | CTRL+R: REROLL AI",
                    16.0,
                    UI_MUTED,
                );
                setup_text(panel, "PRESS ENTER TO START MATCH", 18.0, UI_ACCENT);
            });
        });
}

fn spawn_session_flow_overlay(mut commands: Commands) {
    commands
        .spawn((
            SessionFlowOverlay,
            Visibility::Hidden,
            Node {
                width: percent(100.0),
                height: percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(UI_OVERLAY),
            GlobalZIndex(200),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: px(860.0),
                    max_width: percent(92.0),
                    padding: UiRect::all(px(26.0)),
                    border: UiRect::all(px(2.0)),
                    ..default()
                },
                BackgroundColor(UI_PANEL),
                BorderColor::all(UI_ACCENT),
            ))
            .with_children(|panel| {
                panel.spawn((
                    SessionFlowText,
                    Text::new(""),
                    TextFont {
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(UI_TEXT),
                ));
            });
        });
}

fn accounting_text(session: &GameSession) -> String {
    let rows = session
        .players
        .iter()
        .map(|player| {
            let earnings = session
                .earnings
                .iter()
                .find(|(id, _)| *id == player.configuration.id)
                .map(|(_, earnings)| *earnings)
                .unwrap_or_default();
            format!(
                "{:<18}  ${:>5}  ${:>5}  ${:>5}  ${:>5}",
                player.configuration.display_name,
                earnings.damage_income,
                earnings.placement_income,
                earnings.total(),
                player.cash
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "ROUND EARNINGS\n\nPLAYER               DAMAGE  PLACE  TOTAL  WALLET\n{rows}\n\nPRESS ENTER FOR THE ARMOURY"
    )
}

fn spawn_shop_overlay(mut commands: Commands, icons: Res<WeaponIconAssets>) {
    commands
        .spawn((
            ShopPanel,
            Visibility::Hidden,
            Node {
                width: percent(100.0),
                height: percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(12),
                ..default()
            },
            BackgroundColor(UI_OVERLAY),
            GlobalZIndex(220),
        ))
        .with_children(|root| {
            root.spawn((
                ShopTitle,
                Text::new(""),
                TextFont {
                    font_size: 34.0,
                    ..default()
                },
                TextColor(UI_ACCENT),
            ));
            root.spawn((Node {
                width: percent(92.0),
                max_width: px(1120),
                justify_content: JustifyContent::Center,
                column_gap: px(8),
                row_gap: px(8),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },))
                .with_children(|grid| {
                    for item in weapon_shop_items() {
                        grid.spawn((
                            Node {
                                width: px(200),
                                height: px(246),
                                padding: UiRect::all(px(12)),
                                border_radius: BorderRadius::all(px(10)),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(UI_ELEVATED),
                            BorderColor::all(UI_BORDER),
                        ))
                        .with_children(|card| {
                            card.spawn((
                                ImageNode::new(icons.icon(item.weapon)),
                                Node {
                                    width: px(116),
                                    height: px(104),
                                    margin: UiRect::bottom(px(4)),
                                    ..default()
                                },
                            ));
                            card.spawn((
                                ShopCardText(item.weapon),
                                Text::new(""),
                                TextFont {
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(UI_TEXT),
                            ));
                            card.spawn((
                                Button,
                                ShopBuy(item.weapon),
                                Text::new("BUY"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.04, 0.06, 0.08)),
                                BackgroundColor(UI_ACCENT),
                                BorderColor::all(Color::srgb(1.0, 0.88, 0.46)),
                                ui_button_shadow(),
                                Node {
                                    width: px(92),
                                    height: px(26),
                                    border: UiRect::all(px(1)),
                                    border_radius: BorderRadius::all(px(7)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(px(4)),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
            root.spawn((
                Button,
                ShopDone,
                Text::new("DONE"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(UI_TEXT),
                BackgroundColor(UI_SUCCESS),
                BorderColor::all(Color::srgb(0.48, 0.86, 0.60)),
                ui_button_shadow(),
                Node {
                    width: px(160),
                    height: px(38),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(10)),
                    position_type: PositionType::Absolute,
                    right: px(42),
                    bottom: px(30),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ));
        });
}

fn sync_shop_overlay(
    session: Res<GameSession>,
    mut panels: Query<&mut Visibility, With<ShopPanel>>,
    mut title: Query<&mut Text, (With<ShopTitle>, Without<ShopCardText>)>,
    mut cards: Query<(&ShopCardText, &mut Text)>,
    mut buys: Query<
        (&Interaction, &ShopBuy, &mut BackgroundColor, &mut TextColor),
        Without<ShopDone>,
    >,
    mut done: ShopDoneInteractions,
) {
    let active = session.active_shop_player();
    for mut panel in &mut panels {
        *panel = if session.phase == SessionPhase::Shopping {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let Some(player_id) = active else {
        return;
    };
    let player = session.player(player_id);
    for mut text in &mut title {
        text.0 = format!(
            "{}'S ARMOURY | CASH ${}",
            player.configuration.display_name.to_uppercase(),
            player.cash
        );
    }
    for (card, mut text) in &mut cards {
        let definition = weapon_definition(card.0);
        let item = weapon_shop_items()
            .into_iter()
            .find(|item| item.weapon == card.0)
            .expect("shop weapon");
        let owned = match player.loadout.availability(card.0) {
            WeaponAvailability::Unlimited => "UNLTD".to_owned(),
            WeaponAvailability::Remaining(n) => n.to_string(),
        };
        let visual = weapon::weapon_presentation(card.0);
        text.0 = format!(
            "{}\n{}\nOWNED  x{}\n${}\n{}",
            definition.display_name, visual.compact_name, owned, item.price, visual.description
        );
    }
    for (interaction, buy, mut color, mut text) in &mut buys {
        let price = weapon_price(buy.0).expect("shop weapon");
        let affordable = player.cash >= price;
        color.0 = if !affordable {
            UI_DISABLED
        } else if *interaction == Interaction::Pressed {
            Color::srgb(0.78, 0.48, 0.08)
        } else if *interaction == Interaction::Hovered {
            Color::srgb(1.0, 0.84, 0.34)
        } else {
            UI_ACCENT
        };
        text.0 = if affordable {
            Color::srgb(0.04, 0.06, 0.08)
        } else {
            UI_MUTED
        };
    }
    for (interaction, mut color) in &mut done {
        color.0 = match *interaction {
            Interaction::Pressed => Color::srgb(0.10, 0.34, 0.20),
            Interaction::Hovered => Color::srgb(0.28, 0.72, 0.44),
            Interaction::None => UI_SUCCESS,
        };
    }
}

fn handle_shop_input(
    mut commands: Commands,
    mut writable: ResMut<GameSession>,
    audio: Res<AudioAssets>,
    buys: Query<(&Interaction, &ShopBuy), Changed<Interaction>>,
    done: Query<&Interaction, (With<ShopDone>, Changed<Interaction>)>,
) {
    if writable.phase != SessionPhase::Shopping
        || writable.active_shop_controller() != Some(ControllerType::Human)
    {
        return;
    }
    let active = writable.active_shop_player().expect("human shopper");
    for (interaction, buy) in &buys {
        if *interaction == Interaction::Pressed && writable.purchase(active, buy.0) {
            commands.spawn((
                AudioPlayer(audio.purchase.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
    for interaction in &done {
        if *interaction == Interaction::Pressed {
            writable.complete_active_shopper();
        }
    }
}

fn run_ai_shop(mut session: ResMut<GameSession>) {
    if session.phase != SessionPhase::Shopping
        || session.active_shop_controller() != Some(ControllerType::Ai)
    {
        return;
    }
    let player = session.active_shop_player().expect("AI shopper");
    let shopper = session.player(player);
    let purchases = planned_shop_purchases(
        shopper.configuration.ai_difficulty,
        shopper.cash,
        shopper.loadout,
    );
    for weapon in purchases {
        session.purchase(player, weapon);
    }
    session.complete_active_shopper();
}

fn sync_session_flow_overlay(
    session: Res<GameSession>,
    mut overlays: Query<&mut Visibility, With<SessionFlowOverlay>>,
    mut texts: Query<&mut Text, With<SessionFlowText>>,
) {
    let message = match session.phase {
        SessionPhase::Playing | SessionPhase::Setup | SessionPhase::Transition => None,
        SessionPhase::Celebrating => Some("ROUND COMPLETE\n\nPRESS ENTER FOR EARNINGS".to_owned()),
        SessionPhase::Accounting => Some(accounting_text(&session)),
        SessionPhase::Shopping => None,
    };
    for mut visibility in &mut overlays {
        *visibility = if message.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Some(message) = message {
        for mut text in &mut texts {
            text.0 = message.clone();
        }
    }
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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn update_match_setup(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gate: ResMut<MatchSetupGate>,
    mut configuration: ResMut<PendingMatchConfiguration>,
    mut tanks: ResMut<Tanks>,
    mut turn: ResMut<CurrentTurn>,
    mut weapons: ResMut<WeaponState>,
    world: MatchSetupWorldResources,
    dressing_render: (
        ResMut<Assets<Mesh>>,
        Res<WorldDressingAssets>,
        Res<TerrainTextureAssets>,
        ResMut<Assets<StandardMaterial>>,
    ),
    tank_meshes: Res<TankMeshes>,
    presentation_assets: (Res<TankPresentationAssets>, Res<WeaponIconAssets>),
    overlays: Query<Entity, With<MatchSetupOverlay>>,
    huds: Query<Entity, With<TacticalHud>>,
    tank_visuals: Query<Entity, With<TankVisual>>,
    round_visuals: Query<
        Entity,
        Or<(
            With<BuildingVisual>,
            With<ProjectileVisual>,
            With<ImpactMarker>,
            With<ExplosionVisual>,
            With<ImpactParticle>,
            With<SmokePuff>,
            With<HorizonVisual>,
        )>,
    >,
    mut commands: Commands,
    mut details: Query<&mut Text, With<MatchSetupDetails>>,
) {
    let (
        mut terrain,
        mut match_seed,
        mut dressing,
        mut wind,
        mut ai_seed,
        mut session,
        mut shot_presentation,
        mut camera_aim_reset,
    ) = world;
    let (mut meshes, dressing_assets, terrain_textures, mut materials) = dressing_render;
    let (presentation, icons) = presentation_assets;
    let transition_start = gate.started && session.phase == SessionPhase::Transition;
    if gate.started && !transition_start {
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
    if keyboard.just_pressed(KeyCode::KeyE) {
        configuration.0.cycle_environment();
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
    if setup_controller_toggle_requested(&keyboard) {
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
    let selected_id = configuration.0.players[gate.selected_slot].id;
    if keyboard.just_pressed(KeyCode::KeyD) {
        configuration.0.cycle_ai_difficulty(selected_id);
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
                "{marker} {}. [{}] {}    {:?}{}",
                index + 1,
                player.visual.label(),
                player.display_name,
                player.controller,
                if player.controller == ControllerType::Ai {
                    format!(" ({})", player.ai_difficulty.label())
                } else {
                    String::new()
                }
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
            "PLAYERS: {}\nENVIRONMENT: {}\n{}\n\n{lines}\n\n{validation_message}",
            configuration.0.players.len(),
            configuration.0.environment.label(),
            configuration.0.environment.summary()
        );
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        configuration.0.trim_human_names();
    }
    if (keyboard.just_pressed(KeyCode::Enter) || transition_start)
        && configuration.0.validate().is_ok()
    {
        let ids = configuration
            .0
            .players
            .iter()
            .map(|player| player.id)
            .collect::<Vec<_>>();
        let next_round = if transition_start {
            session.round_number + 1
        } else {
            match_seed.0 = BattlefieldSeed(select_match_seed());
            1
        };
        let selected_round_seed = round_seed(match_seed.0, next_round);
        let (generated, generated_tanks, generated_dressing, generated_wind) =
            generate_match_world(selected_round_seed, &ids, configuration.0.environment);
        terrain.0 = generated;
        tanks.0 = generated_tanks;
        dressing.0 = generated_dressing;
        wind.0 = generated_wind;
        commands.insert_resource(BattlefieldGravity(
            Gravity::new(configuration.0.environment.gravity())
                .expect("preset gravity must be valid"),
        ));
        turn.0 = initial_turn_state(&tanks.0);
        // A new round must not inherit the old impact/result camera. PlayerView lets the normal
        // active-turret camera own the next turn immediately.
        *shot_presentation = ShotPresentation::default();
        camera_aim_reset.0 = true;
        commands.insert_resource(ProjectileFlight::default());
        commands.insert_resource(LatestTerrainImpact::default());
        commands.insert_resource(CurrentImpactExplosionConsumed::default());
        commands.insert_resource(ImpactFlash::default());
        commands.insert_resource(MovementFeedback::default());
        commands.insert_resource(AimRepeatState::default());
        commands.insert_resource(TurretDinkCooldown::default());
        if transition_start {
            weapons.0 = PlayerWeaponLoadouts(
                session
                    .players
                    .iter()
                    .map(|player| {
                        let mut loadout = player.loadout;
                        loadout.reset_selection();
                        (player.configuration.id, loadout)
                    })
                    .collect(),
            );
            session.round_number = next_round;
            session.clear_round_earnings();
            session.phase = SessionPhase::Playing;
        } else {
            *session = GameSession::new(configuration.0.players.clone());
            weapons.0 = PlayerWeaponLoadouts::new(&ids);
        }
        // Gameplay AI gets its own labeled stream; setup names and scenery cannot perturb it.
        ai_seed.0 = derived_seed(selected_round_seed, "ai");
        for entity in &tank_visuals {
            commands.entity(entity).despawn();
        }
        for tank in tanks.0.iter().copied() {
            spawn_tank(
                &mut commands,
                tank,
                &tank_meshes,
                presentation.materials
                    [(tank.owner.0.saturating_sub(1) as usize) % presentation.materials.len()]
                .clone(),
                presentation.track_material.clone(),
                presentation.barrel_material.clone(),
                presentation.firing_origin_material.clone(),
                active_firing_representation(tank, turn.0.aim_for(tank.owner)),
            );
        }
        for entity in &round_visuals {
            commands.entity(entity).despawn();
        }
        commands.spawn((
            HorizonVisual,
            Mesh3d(meshes.add(create_horizon_mesh(&VisualHorizon::from_terrain(
                &terrain.0,
                BattlefieldSeed(derived_seed(selected_round_seed, "terrain")),
                configuration.0.environment.palette(),
            )))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: terrain_texture_for(
                    configuration.0.environment.palette(),
                    &terrain_textures,
                ),
                perceptual_roughness: 0.88,
                ..default()
            })),
        ));
        spawn_buildings(
            &mut commands,
            &mut meshes,
            &dressing.0,
            dressing_assets.material.clone(),
        );
        for entity in &huds {
            commands.entity(entity).despawn();
        }
        spawn_tactical_hud(&mut commands, &configuration.0.players, &icons);
        gate.started = true;
        for overlay in &overlays {
            commands.entity(overlay).despawn();
        }
    }
}

fn setup_controller_toggle_requested(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.just_pressed(KeyCode::Tab)
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
    movement_completion: Res<MovementCompletionThisFrame>,
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
        || movement_completion.0
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
    let mut shot = FiredShot::new(definition, projectile);
    if definition.id == WeaponId::Bouncer {
        shot.contact = ContactState::Bouncing {
            remaining_bounces: 3,
        };
    }

    commands.spawn((
        Name::new("Aimed projectile"),
        ProjectileVisual,
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(projectile_material(assets, definition.id)),
        projectile_transform(shot.projectile),
    ));
    // Successful shared launch is the sole physical fire boundary for Human and AI turns.
    commands.spawn((
        Name::new("Weapon fire audio"),
        AudioPlayer(audio.fire.clone()),
        // Keep the first audible report non-spatial until the listener mix is verified. The
        // request still originates at the authoritative tank/weapon boundary; playback never
        // participates in firing or simulation.
        PlaybackSettings {
            volume: Volume::Linear(if definition.id == WeaponId::HeavyShell {
                1.25
            } else {
                1.05
            }),
            spatial: false,
            ..PlaybackSettings::DESPAWN
        },
        Transform::from_translation(to_bevy_position(parameters.launch_position)),
    ));
    *flight = Some(shot);
    latest_impact.impact = None;
    latest_impact.explosion_visual_scale = 1.0;
    latest_impact.hit_tank = false;
    latest_impact.effect_requests.clear();
    true
}

fn projectile_material(
    assets: &ProjectileVisualAssets,
    weapon: WeaponId,
) -> Handle<StandardMaterial> {
    match weapon {
        WeaponId::BasicShell => assets.basic_material.clone(),
        WeaponId::HighExplosive => assets.high_explosive_material.clone(),
        WeaponId::HeavyShell => assets.heavy_material.clone(),
        WeaponId::Mirv => assets.mirv_carrier_material.clone(),
        WeaponId::ClusterBomb => assets.cluster_material.clone(),
        WeaponId::Roller => assets.roller_material.clone(),
        WeaponId::BunkerBuster => assets.bunker_material.clone(),
        WeaponId::DirtBomb => assets.cluster_material.clone(),
        WeaponId::CurveBall => assets.mirv_carrier_material.clone(),
        WeaponId::Bouncer => assets.roller_material.clone(),
        WeaponId::Nuke => assets.high_explosive_material.clone(),
        #[cfg(test)]
        WeaponId::TestConventional => assets.basic_material.clone(),
    }
}

fn projectile_material_asset(
    materials: &mut Assets<StandardMaterial>,
    colour: Color,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: colour,
        metallic: 0.5,
        perceptual_roughness: 0.38,
        ..default()
    })
}

fn projectile_transform(projectile: Projectile) -> Transform {
    projectile_visual_transform(projectile.position, projectile.velocity)
}

/// Builds a light, self-contained rocket aligned to +Y. Keeping its fins in the same mesh means
/// split salvos need no child entities and retain the existing projectile lifecycle.
fn rocket_mesh() -> Mesh {
    fn triangle(
        positions: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        a: Vec3,
        b: Vec3,
        c: Vec3,
    ) {
        let normal = (b - a).cross(c - a).normalize_or_zero().to_array();
        let index = positions.len() as u32;
        positions.extend([a.to_array(), b.to_array(), c.to_array()]);
        normals.extend([normal; 3]);
        indices.extend([index, index + 1, index + 2]);
    }

    fn quad(
        positions: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        d: Vec3,
    ) {
        triangle(positions, normals, indices, a, b, c);
        triangle(positions, normals, indices, a, c, d);
    }

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    const SIDES: usize = 10;
    const RADIUS: f32 = 0.14;
    const REAR_Y: f32 = -0.42;
    const NOSE_BASE_Y: f32 = 0.20;
    let nose = Vec3::new(0.0, 0.56, 0.0);

    for side in 0..SIDES {
        let angle = side as f32 * std::f32::consts::TAU / SIDES as f32;
        let next_angle = (side + 1) as f32 * std::f32::consts::TAU / SIDES as f32;
        let radial = Vec3::new(angle.cos() * RADIUS, 0.0, angle.sin() * RADIUS);
        let next_radial = Vec3::new(next_angle.cos() * RADIUS, 0.0, next_angle.sin() * RADIUS);
        let rear = radial.with_y(REAR_Y);
        let front = radial.with_y(NOSE_BASE_Y);
        let next_front = next_radial.with_y(NOSE_BASE_Y);
        let next_rear = next_radial.with_y(REAR_Y);
        quad(
            &mut positions,
            &mut normals,
            &mut indices,
            rear,
            front,
            next_front,
            next_rear,
        );
        triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            front,
            nose,
            next_front,
        );
    }

    // Four shallow triangular prisms form durable, visible fins without overpowering the body.
    for direction in [Vec3::X, Vec3::Z, -Vec3::X, -Vec3::Z] {
        let tangent = Vec3::new(-direction.z, 0.0, direction.x) * 0.025;
        let root_top = direction * 0.11 + Vec3::Y * -0.12;
        let root_bottom = direction * 0.11 + Vec3::Y * REAR_Y;
        let tip = direction * 0.39 + Vec3::Y * -0.46;
        let front = [root_top + tangent, root_bottom + tangent, tip + tangent];
        let back = [root_top - tangent, root_bottom - tangent, tip - tangent];
        triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            front[0],
            front[1],
            front[2],
        );
        triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            back[2],
            back[1],
            back[0],
        );
        quad(
            &mut positions,
            &mut normals,
            &mut indices,
            front[0],
            back[0],
            back[1],
            front[1],
        );
        quad(
            &mut positions,
            &mut normals,
            &mut indices,
            front[1],
            back[1],
            back[2],
            front[2],
        );
        quad(
            &mut positions,
            &mut normals,
            &mut indices,
            front[2],
            back[2],
            back[0],
            front[0],
        );
    }

    let vertex_count = positions.len();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertex_count])
    .with_inserted_indices(Indices::U32(indices))
}

fn projectile_visual_transform(position: WorldPosition, velocity: WorldVector) -> Transform {
    let velocity = Vec3::new(velocity.x, velocity.y, velocity.z);
    let rotation = if velocity.length_squared() > f32::EPSILON {
        Quat::from_rotation_arc(Vec3::Y, velocity.normalize())
    } else {
        Quat::IDENTITY
    };
    Transform::from_translation(to_bevy_position(position)).with_rotation(rotation)
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

/// The first continuing-loop UI is intentionally keyboard-simple while the existing HUD remains
/// visible: Enter advances celebration -> accounting -> shop -> a fresh round.  Combat systems
/// remain blocked because the result turn state is already Finished.
fn update_session_flow(
    keyboard: Res<ButtonInput<KeyCode>>,
    turn: Res<CurrentTurn>,
    weapons: Res<WeaponState>,
    mut session: ResMut<GameSession>,
) {
    if session.phase == SessionPhase::Playing && turn.0.match_state != MatchState::InProgress {
        for player in &mut session.players {
            player.loadout = weapons.0.for_player(player.configuration.id);
        }
        session.finalise(&turn.0.placement_order());
        return;
    }
    if !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }
    match session.phase {
        SessionPhase::Celebrating => session.phase = SessionPhase::Accounting,
        SessionPhase::Accounting => session.begin_shop(),
        _ => {}
    }
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
                apex_seen: shot
                    .presentation_projectile()
                    .is_some_and(|projectile| projectile.velocity.y <= 0.0),
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
                apex_seen: apex_seen
                    || shot
                        .presentation_projectile()
                        .is_some_and(|projectile| projectile.velocity.y <= 0.0),
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
                    position: result_focus_position(
                        &tanks.0,
                        turn.0.match_state,
                        WorldPosition {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ),
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
                    ShotPresentationPhase::Result {
                        position: result_focus_position(&tanks.0, turn.0.match_state, position),
                    }
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

/// A result celebrates the survivor, not the location that happened to receive the final blast.
/// Draws retain the final impact framing because there is no winning tank to focus.
fn result_focus_position(
    tanks: &[Tank],
    state: MatchState,
    fallback: WorldPosition,
) -> WorldPosition {
    match state {
        MatchState::Winner(winner) => tanks
            .iter()
            .find(|tank| tank.owner == winner)
            .map_or(fallback, |tank| tank.pose.position),
        MatchState::InProgress | MatchState::Draw => fallback,
    }
}

#[allow(clippy::too_many_arguments)]
fn run_ai_controller(
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    tanks: Res<Tanks>,
    terrain: Res<BattlefieldState>,
    wind: Res<BattlefieldWind>,
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
    let difficulty = configuration
        .0
        .players
        .iter()
        .find(|player| player.id == actor)
        .expect("active player is configured")
        .ai_difficulty;
    let Some(decision) = decide_firing_with_terrain(
        &mut seed.0,
        actor,
        difficulty,
        &tanks.0,
        weapons.0.for_player(actor),
        wind.0,
        &terrain.0,
    ) else {
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
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(WeaponId::Mirv)
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        Some(WeaponId::ClusterBomb)
    } else if keyboard.just_pressed(KeyCode::Digit6) {
        Some(WeaponId::Roller)
    } else if keyboard.just_pressed(KeyCode::Digit7) {
        Some(WeaponId::BunkerBuster)
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

/// The bottom weapon strip is a presentation affordance over the same authoritative selection
/// boundary as number keys. A slot cannot consume ammunition and is ignored outside a human
/// choosing turn.
fn select_weapon_slot(
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    flight: Res<ProjectileFlight>,
    turn: Res<CurrentTurn>,
    mut weapons: ResMut<WeaponState>,
    slots: Query<(&Interaction, &WeaponSlot), Changed<Interaction>>,
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
    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed {
            weapons
                .0
                .for_player_mut(turn.0.current_player)
                .select(slot.0);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn select_movement_action(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    tanks: Res<Tanks>,
    mut feedback: ResMut<MovementFeedback>,
    mut movement_frame: ResMut<MovementFrame>,
    mut turn: ResMut<CurrentTurn>,
) {
    if setup.started
        && presentation_allows_new_action(&presentation)
        && current_player_is_human(&configuration.0, &turn.0)
        && keyboard.just_pressed(KeyCode::KeyM)
        && turn.0.begin_movement()
    {
        feedback.0 = None;
        movement_frame.0 = Some(
            tank_for_player(&tanks.0, turn.0.current_player)
                .pose
                .body_forward,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn update_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    setup: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    presentation: Res<ShotPresentation>,
    terrain: Res<BattlefieldState>,
    mut movement_frame: ResMut<MovementFrame>,
    mut tanks: ResMut<Tanks>,
    mut feedback: ResMut<MovementFeedback>,
    mut completion: ResMut<MovementCompletionThisFrame>,
    mut turn: ResMut<CurrentTurn>,
) {
    completion.0 = false;
    if !setup.started
        || !current_player_is_human(&configuration.0, &turn.0)
        || !presentation_allows_new_action(&presentation)
        || !turn.0.is_moving()
    {
        return;
    }
    if movement_completion_requested(&keyboard) {
        if turn.0.finish_movement(survivors(&tanks.0)) {
            feedback.0 = None;
            movement_frame.0 = None;
            completion.0 = keyboard.just_pressed(KeyCode::Space);
        }
        return;
    }
    let player = turn.0.current_player;
    let tank = tank_for_player(&tanks.0, player);
    let Some(direction) = movement_direction(
        &keyboard,
        movement_frame.0.unwrap_or(tank.pose.body_forward),
    ) else {
        return;
    };
    match tank.step_on_terrain(&terrain.0, direction) {
        Ok(moved) => {
            *tank_for_player_mut(&mut tanks.0, player) = moved;
            assert!(
                turn.0.accept_movement_step(),
                "moving turn must retain accepted step"
            );
            feedback.0 = None;
        }
        Err(rejection) => feedback.0 = Some(rejection),
    }
}

fn movement_completion_requested(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space)
}

/// Movement uses the tank's hull as its reference: up is forward, rather than whichever way the
/// player happened to orbit the tactical camera before entering movement mode.
fn movement_direction(
    keyboard: &ButtonInput<KeyCode>,
    forward: HorizontalDirection,
) -> Option<MovementDirection> {
    let right = (-forward.z, forward.x);
    let directions = [
        (KeyCode::ArrowUp, (forward.x, forward.z)),
        (KeyCode::ArrowLeft, (-right.0, -right.1)),
        (KeyCode::ArrowDown, (-forward.x, -forward.z)),
        (KeyCode::ArrowRight, right),
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
    mut camera_reset: ResMut<CameraAimReset>,
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
            let rotates_turret = matches!(
                adjustment,
                AimAdjustment::AzimuthDecrease
                    | AimAdjustment::AzimuthIncrease
                    | AimAdjustment::ElevationIncrease
                    | AimAdjustment::ElevationDecrease
            );
            if rotates_turret && before != after && dink_cooldown.0 == 0.0 {
                commands.spawn((
                    Name::new("Turret adjustment audio"),
                    AudioPlayer(audio.turret_dink.clone()),
                    PlaybackSettings {
                        volume: Volume::Linear(0.38),
                        spatial: false,
                        ..PlaybackSettings::DESPAWN
                    },
                ));
                dink_cooldown.0 = TURRET_DINK_INTERVAL_SECONDS;
            }
            if rotates_turret && before != after {
                camera_reset.0 = true;
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
        let (body_scale, _, body_offset) = TankVisualModel::for_player(owner.0).dimensions();
        *transform = direction_transform(tank.pose.body_forward)
            .with_translation(Vec3::new(0.0, 0.4, body_offset))
            .with_scale(body_scale);
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

fn spawn_tactical_hud(
    commands: &mut Commands,
    players: &[PlayerConfiguration],
    icons: &WeaponIconAssets,
) {
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
                ImpactFlashOverlay,
                Node {
                    width: percent(100),
                    height: percent(100),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                GlobalZIndex(100),
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
                Pickable::IGNORE,
            ));
            root.spawn((
                BackgroundColor(UI_PANEL),
                BorderColor::all(UI_ACCENT),
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
                BackgroundColor(UI_PANEL),
                BorderColor::all(UI_ACCENT),
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
                BackgroundColor(UI_PANEL),
                BorderColor::all(UI_BORDER),
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
                BackgroundColor(UI_PANEL),
                BorderColor::all(UI_BORDER),
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
            root.spawn((Node {
                position_type: PositionType::Absolute,
                bottom: px(18),
                left: percent(50),
                width: percent(82.0),
                max_width: px(980),
                height: px(86),
                // The strip's capped width is 980px; anchoring its exact half-width at the
                // viewport midpoint keeps the weapon inventory centred instead of depending on
                // percentage-margin behaviour in Bevy's flex layout.
                margin: UiRect::left(px(-490)),
                column_gap: px(5),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },))
                .with_children(|bar| {
                    for weapon in weapon_strip_order() {
                        bar.spawn((
                            Button,
                            WeaponSlot(weapon),
                            Visibility::Hidden,
                            BackgroundColor(UI_PANEL),
                            BorderColor::all(UI_BORDER),
                            ui_button_shadow(),
                            Node {
                                width: px(76),
                                height: px(78),
                                border: UiRect::all(px(2)),
                                border_radius: BorderRadius::all(px(8)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|slot| {
                            slot.spawn((
                                ImageNode::new(icons.icon(weapon)),
                                Node {
                                    width: px(50),
                                    height: px(43),
                                    position_type: PositionType::Absolute,
                                    top: px(1),
                                    ..default()
                                },
                            ));
                            slot.spawn((
                                Text::new(weapon_strip_label(weapon)),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(UI_TEXT),
                                Node {
                                    position_type: PositionType::Absolute,
                                    bottom: px(4),
                                    width: percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                            ));
                            slot.spawn((
                                WeaponSlotAmmo(weapon),
                                Text::default(),
                                TextFont {
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: px(4),
                                    right: px(5),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}

fn weapon_strip_order() -> [WeaponId; 11] {
    weapon::ACTIVE_WEAPONS
}

fn weapon_strip_label(weapon: WeaponId) -> &'static str {
    weapon::weapon_presentation(weapon).compact_name
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
            TurnPhase::Moving => HudAction::Moving,
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
        movement: turn.is_moving().then_some(feedback).flatten(),
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
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn sync_tactical_hud(
    setup: Res<MatchSetupGate>,
    turn: Res<CurrentTurn>,
    tanks: Res<Tanks>,
    weapons: Res<WeaponState>,
    wind: Res<BattlefieldWind>,
    feedback: Res<MovementFeedback>,
    configuration: Res<PendingMatchConfiguration>,
    mut text: Query<(&HudText, &mut Text), (Without<HudScoreboardText>, Without<WeaponSlotAmmo>)>,
    mut scoreboard_text: Query<
        (&HudScoreboardText, &mut Text),
        (Without<HudText>, Without<WeaponSlotAmmo>),
    >,
    mut fills: Query<(&HudHealthFill, &mut Node), Without<WeaponSlot>>,
    mut rows: Query<
        (&HudScoreboardRow, &mut BorderColor, &mut BackgroundColor),
        Without<WeaponSlot>,
    >,
    mut weapon_slots: Query<(
        &WeaponSlot,
        &mut Visibility,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut weapon_ammunition: Query<
        (&WeaponSlotAmmo, &mut Text),
        (Without<HudText>, Without<HudScoreboardText>),
    >,
    mut decorations: HudDecorations,
    mut huds: Query<&mut Visibility, (With<TacticalHud>, Without<WeaponSlot>)>,
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
    let selectable =
        setup.started && view.action == HudAction::Choose && view.active_player.is_some();
    if let Some(player) = view.active_player {
        let loadout = weapons.0.for_player(player);
        for (slot, mut visibility, mut background, mut border) in &mut weapon_slots {
            let availability = loadout.availability(slot.0);
            *visibility = if selectable {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            let selected = slot.0 == loadout.selected();
            let enabled = availability.is_available();
            border.top = if selected && enabled {
                UI_ACCENT
            } else if enabled {
                UI_BORDER
            } else {
                UI_DISABLED
            };
            border.right = border.top;
            border.bottom = border.top;
            border.left = border.top;
            background.0 = if !enabled {
                Color::srgba(0.04, 0.05, 0.06, 0.78)
            } else if selected {
                Color::srgba(0.25, 0.19, 0.06, 0.98)
            } else {
                UI_PANEL
            };
        }
        for (ammo, mut text) in &mut weapon_ammunition {
            text.0 = match loadout.availability(ammo.0) {
                WeaponAvailability::Unlimited => "UNLTD".into(),
                WeaponAvailability::Remaining(rounds) => rounds.to_string(),
            };
        }
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
        HudTextField::Movement => match view.action {
            HudAction::Moving => format!(
                "MOVE: UNLIMITED{}",
                match view.movement {
                    Some(MovementRejection::Bounds) => " - EDGE",
                    None => "",
                }
            ),
            _ => String::new(),
        },
        HudTextField::Controls => match view.action {
            HudAction::Choose => {
                "CLICK WEAPON BAR | M MOVE | SPACE FIRE\nLEFT-DRAG CAMERA | SCROLL ZOOM | ARROWS AIM | CURVE: ARROWS STEER IN FLIGHT | -/= POWER".into()
            }
            HudAction::Moving => "ARROWS MOVE (CAMERA) | SPACE/ENTER END".into(),
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

fn tank_materials(
    materials: &mut Assets<StandardMaterial>,
    texture: Handle<Image>,
) -> Vec<Handle<StandardMaterial>> {
    (1..=8)
        .map(|id| {
            materials.add(StandardMaterial {
                base_color: player_color(PlayerId(id)),
                base_color_texture: Some(texture.clone()),
                metallic: 0.18,
                perceptual_roughness: 0.58,
                ..default()
            })
        })
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

/// A session root identifies a local game; the round number selects one reproducible fresh world
/// without allowing UI, cosmetic, or AI randomness to perturb terrain or player starts.
fn round_seed(root: BattlefieldSeed, round_number: u32) -> BattlefieldSeed {
    BattlefieldSeed(
        derived_seed(root, "round")
            .wrapping_add((round_number as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)),
    )
}

fn generate_match_world(
    seed: BattlefieldSeed,
    players: &[PlayerId],
    environment: EnvironmentPreset,
) -> (BattlefieldTerrain, Vec<Tank>, Vec<BuildingPlacement>, Wind) {
    const MAX_GENERATION_ATTEMPTS: u64 = 32;
    let (seed, terrain, tanks) = (0..MAX_GENERATION_ATTEMPTS)
        .find_map(|attempt| {
            let candidate = BattlefieldSeed(
                seed.0
                    .wrapping_add(attempt.wrapping_mul(0xd1b5_4a32_d192_ed03)),
            );
            let terrain = BattlefieldTerrain::generated(
                BattlefieldSeed(derived_seed(candidate, "terrain")),
                environment.terrain_profile(),
            );
            initial_tanks_for_players_seeded(&terrain, players, derived_seed(candidate, "starts"))
                .map(|tanks| (candidate, terrain, tanks))
        })
        .expect("bounded generated terrain candidates must support the configured 2-8 players");
    let starts = tanks
        .iter()
        .map(|tank| (tank.pose.position.x, tank.pose.position.z))
        .collect::<Vec<_>>();
    let dressing = generate_buildings(&terrain, &starts, derived_seed(seed, "dressing"));
    let wind = if WIND_ENABLED {
        wind_from_seed(derived_seed(seed, "wind"), environment.wind_range())
    } else {
        Wind::new(WorldVector::ZERO).expect("calm wind must be valid")
    };
    eprintln!("Azimuth round battlefield seed: {}", seed.0);
    (terrain, tanks, dressing, wind)
}

/// Kept pure so a recorded seed recreates the same match condition in tests or diagnostics.
fn wind_from_seed(mut seed: u64, range: (f32, f32)) -> Wind {
    let direction_fraction = next_random_fraction(&mut seed);
    let strength_fraction = next_random_fraction(&mut seed);
    let angle = direction_fraction * std::f32::consts::TAU;
    let strength = range.0 + (range.1 - range.0) * strength_fraction;
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

/// Turnwind is sampled only when control reaches a new choosing turn; projectile flight and
/// resolution continue using the retained resource value.
fn update_turnwind(
    configuration: Res<PendingMatchConfiguration>,
    match_seed: Res<MatchSeed>,
    turn: Res<CurrentTurn>,
    mut state: ResMut<TurnwindState>,
    mut wind: ResMut<BattlefieldWind>,
) {
    if configuration.0.environment.wind_policy() != WindPolicy::RerollEachTurn
        || turn.0.phase != TurnPhase::Choosing
    {
        return;
    }
    if state.last_player == Some(turn.0.current_player) {
        return;
    }
    state.last_player = Some(turn.0.current_player);
    state.handoff_ordinal = state.handoff_ordinal.wrapping_add(1);
    wind.0 = wind_from_seed(
        derived_seed(match_seed.0, &format!("turnwind-{}", state.handoff_ordinal)),
        configuration.0.environment.wind_range(),
    );
}

fn player_name(player: PlayerId) -> String {
    format!("Player {}", player.0)
}

fn begin_damage_accounting(
    flight: Res<ProjectileFlight>,
    turn: Res<CurrentTurn>,
    tanks: Res<Tanks>,
    mut accounting: ResMut<DamageAccounting>,
) {
    if flight.0.is_some() && accounting.owner.is_none() {
        accounting.owner = Some(turn.0.current_player);
        accounting.health_before = tanks
            .0
            .iter()
            .map(|tank| (tank.owner, tank.health))
            .collect();
    }
}

fn credit_resolved_damage(
    tanks: Res<Tanks>,
    flight: Res<ProjectileFlight>,
    mut session: ResMut<GameSession>,
    mut accounting: ResMut<DamageAccounting>,
) {
    let Some(owner) = accounting.owner else {
        return;
    };
    for (target, before) in &mut accounting.health_before {
        let after = tanks
            .0
            .iter()
            .find(|tank| tank.owner == *target)
            .expect("accounted tank remains configured")
            .health;
        if after < *before {
            session.credit_damage(owner, *target, *before - after);
            *before = after;
        }
    }
    if flight.0.is_none() {
        accounting.owner = None;
        accounting.health_before.clear();
    }
}

#[allow(clippy::too_many_arguments)]
fn advance_projectile(
    keyboard: Res<ButtonInput<KeyCode>>,
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

    match shot.contact {
        ContactState::Rolling { remaining_steps } => {
            let downhill = terrain
                .0
                .downhill_direction_if_within_bounds(
                    shot.projectile.position.x,
                    shot.projectile.position.z,
                )
                .unwrap_or(WorldVector::ZERO);
            shot.projectile.velocity = shot
                .projectile
                .velocity
                .scaled(0.992)
                .added(downhill.scaled(0.035));
            let next = shot.projectile.position.translated(WorldVector {
                x: shot.projectile.velocity.x * crate::projectile::FIXED_STEP_SECONDS,
                y: 0.0,
                z: shot.projectile.velocity.z * crate::projectile::FIXED_STEP_SECONDS,
            });
            if let Some(height) = terrain.0.height_if_within_bounds(next.x, next.z) {
                shot.projectile.position = WorldPosition {
                    x: next.x,
                    y: height + 0.18,
                    z: next.z,
                };
            } else {
                shot.contact = ContactState::Rolling { remaining_steps: 0 };
            }
            // A Roller is a ground-hugging weapon: reaching an opponent is an impact, not a
            // cosmetic near-miss. The resolving turn still identifies its firing owner.
            if let Some(target) =
                rolling_enemy_contact(shot.projectile.position, turn.0.current_player, &tanks.0)
            {
                flight.0 = resolve_projectile_advance(
                    shot,
                    ProjectileAdvance::TerrainImpact(TerrainImpact { position: target }),
                    &mut terrain.0,
                    &mut tanks.0,
                    &mut latest_impact,
                    &mut turn.0,
                );
                return;
            }
            let speed =
                (shot.projectile.velocity.x.powi(2) + shot.projectile.velocity.z.powi(2)).sqrt();
            if remaining_steps <= 1 || speed < 0.35 {
                let impact = TerrainImpact {
                    position: WorldPosition {
                        y: terrain
                            .0
                            .height(shot.projectile.position.x, shot.projectile.position.z),
                        ..shot.projectile.position
                    },
                };
                flight.0 = resolve_projectile_advance(
                    shot,
                    ProjectileAdvance::TerrainImpact(impact),
                    &mut terrain.0,
                    &mut tanks.0,
                    &mut latest_impact,
                    &mut turn.0,
                );
            } else {
                shot.contact = ContactState::Rolling {
                    remaining_steps: remaining_steps - 1,
                };
                flight.0 = Some(shot);
            }
            return;
        }
        ContactState::Penetrating {
            remaining_steps,
            direction,
        } => {
            shot.projectile.position = shot.projectile.position.translated(direction.scaled(0.18));
            if remaining_steps <= 1 {
                let impact = TerrainImpact {
                    position: shot.projectile.position,
                };
                flight.0 = resolve_projectile_advance(
                    shot,
                    ProjectileAdvance::TerrainImpact(impact),
                    &mut terrain.0,
                    &mut tanks.0,
                    &mut latest_impact,
                    &mut turn.0,
                );
            } else {
                shot.contact = ContactState::Penetrating {
                    remaining_steps: remaining_steps - 1,
                    direction,
                };
                flight.0 = Some(shot);
            }
            return;
        }
        ContactState::Airborne | ContactState::Bouncing { .. } => {}
    }

    if shot.split {
        for index in 0..shot.children.len() {
            let Some(mut child) = shot.children[index] else {
                continue;
            };
            match child.advance_with_terrain(
                gravity.0,
                wind.0,
                SimulationLimits::BATTLEFIELD,
                |x, z| terrain.0.height_if_within_bounds(x, z),
            ) {
                ProjectileAdvance::Active => shot.children[index] = Some(child),
                ProjectileAdvance::TerrainImpact(impact) => {
                    let health_before = tanks.0.iter().map(|tank| tank.health).collect::<Vec<_>>();
                    resolve_explosion(&mut tanks.0, impact.position, shot.impact);
                    terrain.0.apply_crater(impact.position, shot.impact.crater);
                    latest_impact.impact = Some(impact);
                    latest_impact.explosion_visual_scale = shot.impact.explosion_visual_scale;
                    latest_impact.hit_tank = tanks
                        .0
                        .iter()
                        .zip(health_before)
                        .any(|(tank, health)| tank.health < health);
                    queue_impact_effect(
                        &mut latest_impact,
                        impact.position,
                        shot.impact.explosion_visual_scale,
                    );
                    reconcile_living_tank_support(&mut tanks.0, &terrain.0);
                    shot.children[index] = None;
                }
                ProjectileAdvance::OutOfBounds => shot.children[index] = None,
            }
        }
        if shot.has_active_projectiles() {
            flight.0 = Some(shot);
        } else if !any_living_tank_is_settling(&tanks.0) {
            assert!(turn.0.complete_fire_resolution(survivors(&tanks.0)));
            flight.0 = None;
        } else {
            flight.0 = None;
        }
        return;
    }

    let curve_acceleration = if shot.weapon == WeaponId::CurveBall {
        let forward = WorldVector {
            x: shot.projectile.velocity.x,
            y: 0.0,
            z: shot.projectile.velocity.z,
        }
        .normalized();
        let steering = (keyboard.pressed(KeyCode::ArrowRight) as i8
            - keyboard.pressed(KeyCode::ArrowLeft) as i8) as f32;
        // Steering is deliberately gentle and only exists while an arrow is held in the fixed
        // simulation. It bends relative to current flight direction, never toward a target.
        WorldVector {
            x: -forward.z * steering * 2.2,
            y: 0.0,
            z: forward.x * steering * 2.2,
        }
    } else {
        WorldVector::ZERO
    };
    let advance = shot.projectile.advance_with_terrain_and_acceleration(
        gravity.0,
        wind.0,
        curve_acceleration,
        SimulationLimits::BATTLEFIELD,
        |x, z| terrain.0.height_if_within_bounds(x, z),
    );
    if shot.is_deployment_carrier()
        && matches!(advance, ProjectileAdvance::Active)
        && shot.projectile.velocity.y <= 0.0
    {
        let carrier = shot.projectile;
        let forward = WorldVector {
            x: carrier.velocity.x,
            y: 0.0,
            z: carrier.velocity.z,
        };
        let length = (forward.x * forward.x + forward.z * forward.z)
            .sqrt()
            .max(0.001);
        let right = WorldVector {
            x: -forward.z / length,
            y: 0.0,
            z: forward.x / length,
        };
        let child_count = match shot.weapon {
            WeaponId::Mirv => MIRV_CHILD_COUNT,
            WeaponId::ClusterBomb => CLUSTER_BOMB_CHILD_COUNT,
            _ => unreachable!("only deployment carriers reach this branch"),
        };
        for index in 0..child_count {
            let mut child = carrier;
            let (lateral, forward_offset) = match shot.weapon {
                WeaponId::Mirv => (index as f32 - 2.0, (index as f32 - 2.0) * 0.08),
                WeaponId::ClusterBomb => {
                    let ring = index as f32 - 4.5;
                    (ring * 0.42, ((index % 3) as f32 - 1.0) * 0.24)
                }
                _ => unreachable!(),
            };
            child.velocity = child
                .velocity
                .added(right.scaled(lateral * 0.8))
                .added(forward.scaled(forward_offset));
            shot.children[index] = Some(child);
        }
        shot.split = true;
        flight.0 = Some(shot);
    } else {
        match (shot.weapon, advance) {
            (WeaponId::Roller, ProjectileAdvance::TerrainImpact(impact)) => {
                shot.projectile.position = impact.position;
                shot.projectile.velocity.y = 0.0;
                shot.contact = ContactState::Rolling {
                    remaining_steps: 720,
                };
                flight.0 = Some(shot);
            }
            (WeaponId::BunkerBuster, ProjectileAdvance::TerrainImpact(impact)) => {
                let velocity = shot.projectile.velocity;
                let length =
                    (velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z)
                        .sqrt()
                        .max(0.001);
                shot.projectile.position = impact.position;
                shot.contact = ContactState::Penetrating {
                    remaining_steps: 14,
                    direction: WorldVector {
                        x: velocity.x / length,
                        y: velocity.y.min(-0.25) / length,
                        z: velocity.z / length,
                    },
                };
                flight.0 = Some(shot);
            }
            (WeaponId::Bouncer, ProjectileAdvance::TerrainImpact(impact)) => {
                let ContactState::Bouncing { remaining_bounces } = shot.contact else {
                    unreachable!()
                };
                if remaining_bounces == 0 {
                    flight.0 = resolve_projectile_advance(
                        shot,
                        ProjectileAdvance::TerrainImpact(impact),
                        &mut terrain.0,
                        &mut tanks.0,
                        &mut latest_impact,
                        &mut turn.0,
                    );
                } else {
                    let normal = terrain
                        .0
                        .surface_normal_if_within_bounds(impact.position.x, impact.position.z)
                        .unwrap_or(WorldVector {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        });
                    let velocity = shot.projectile.velocity;
                    let reflected = velocity
                        .added(normal.scaled(-2.0 * velocity.dot(normal)))
                        .scaled(0.62);
                    shot.projectile.position = impact.position.translated(normal.scaled(0.08));
                    shot.projectile.velocity = reflected;
                    shot.contact = ContactState::Bouncing {
                        remaining_bounces: remaining_bounces - 1,
                    };
                    flight.0 = Some(shot);
                }
            }
            (_, advance) => {
                flight.0 = resolve_projectile_advance(
                    shot,
                    advance,
                    &mut terrain.0,
                    &mut tanks.0,
                    &mut latest_impact,
                    &mut turn.0,
                )
            }
        }
    }
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
            let health_before = tanks.iter().map(|tank| tank.health).collect::<Vec<_>>();
            resolve_explosion(tanks, impact.position, shot.impact);
            if let Some(mound) = shot.impact.mound {
                terrain.apply_mound(impact.position, mound);
            } else {
                terrain.apply_crater(impact.position, shot.impact.crater);
            }
            latest_impact.impact = Some(impact);
            latest_impact.explosion_visual_scale = shot.impact.explosion_visual_scale;
            latest_impact.hit_tank = tanks
                .iter()
                .zip(health_before)
                .any(|(tank, health)| tank.health < health);
            queue_impact_effect(
                latest_impact,
                impact.position,
                shot.impact.explosion_visual_scale,
            );
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

/// Requests cross from resolved impacts into presentation only after all authoritative outcomes
/// have been computed. A full queue reduces visual density, never impact resolution.
fn queue_impact_effect(latest: &mut LatestTerrainImpact, position: WorldPosition, scale: f32) {
    if latest.effect_requests.len() >= MAX_PENDING_IMPACT_EFFECTS {
        return;
    }
    latest.effect_requests.push(ImpactPresentationRequest {
        position,
        scale,
        serial: latest.next_effect_serial,
    });
    latest.next_effect_serial = latest.next_effect_serial.wrapping_add(1);
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

fn rolling_enemy_contact(
    position: WorldPosition,
    owner: PlayerId,
    tanks: &[Tank],
) -> Option<WorldPosition> {
    const ROLLER_HIT_RADIUS: f32 = 1.35;
    tanks
        .iter()
        .find(|tank| {
            tank.owner != owner
                && !tank.is_eliminated()
                && (tank.pose.position.x - position.x).hypot(tank.pose.position.z - position.z)
                    <= ROLLER_HIT_RADIUS
        })
        .map(|tank| tank.pose.position)
}

fn sync_battlefield_mesh(
    terrain: Res<BattlefieldState>,
    configuration: Res<PendingMatchConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    visuals: Query<&Mesh3d, With<BattlefieldVisual>>,
) {
    if !terrain.is_changed() {
        return;
    }
    for mesh_handle in &visuals {
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = create_battlefield_mesh(&terrain.0, configuration.0.environment.palette());
        }
    }
}

fn sync_projectile_visual(
    flight: Res<ProjectileFlight>,
    assets: Res<ProjectileVisualAssets>,
    mut commands: Commands,
    mut visuals: Query<
        (
            Entity,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        With<ProjectileVisual>,
    >,
) {
    if let Some(shot) = flight.0 {
        let positions = if shot.split {
            shot.children
                .iter()
                .filter_map(|child| *child)
                .map(|child| (child.position, child.velocity))
                .collect::<Vec<_>>()
        } else {
            vec![(shot.projectile.position, shot.projectile.velocity)]
        };
        let mut existing = visuals.iter_mut();
        let child_material = match shot.weapon {
            WeaponId::Mirv => assets.mirv_child_material.clone(),
            WeaponId::ClusterBomb => assets.cluster_material.clone(),
            _ => assets.basic_material.clone(),
        };
        for (position, velocity) in &positions {
            if let Some((_, mut transform, mut material)) = existing.next() {
                *transform = projectile_visual_transform(*position, *velocity);
                if shot.split {
                    material.0 = child_material.clone();
                }
            } else {
                commands.spawn((
                    Name::new("Barrage child projectile"),
                    ProjectileVisual,
                    Mesh3d(assets.mesh.clone()),
                    MeshMaterial3d(child_material.clone()),
                    projectile_visual_transform(*position, *velocity),
                ));
            }
        }
        for (entity, _, _) in existing {
            commands.entity(entity).despawn();
        }
    } else {
        for (entity, _, _) in &mut visuals {
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
    mut flash: ResMut<ImpactFlash>,
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
            // Large impacts keep their visual scale, but presentation volume stays comfortable.
            volume: Volume::Linear((1.15 * latest_impact.explosion_visual_scale).min(1.6)),
            spatial: false,
            ..PlaybackSettings::DESPAWN
        },
        Transform::from_translation(to_bevy_position(impact.position)),
    ));
    flash.0 = Some((
        0.0,
        latest_impact.explosion_visual_scale,
        latest_impact.hit_tank,
    ));
    consumed.0 = true;
}

/// A single impact-owned UI pulse reinforces force without owning any impact consequence.
fn update_impact_flash(
    time: Res<Time>,
    mut flash: ResMut<ImpactFlash>,
    mut overlays: Query<&mut BackgroundColor, With<ImpactFlashOverlay>>,
) {
    let (opacity, hit_tank) = if let Some((elapsed, scale, hit_tank)) = &mut flash.0 {
        *elapsed += time.delta_secs();
        let duration = IMPACT_FLASH_BASE_DURATION_SECONDS * scale.clamp(1.0, 1.5);
        let progress = (*elapsed / duration).clamp(0.0, 1.0);
        if progress >= 1.0 {
            flash.0 = None;
            (0.0, false)
        } else {
            (
                IMPACT_FLASH_BASE_OPACITY * scale.clamp(1.0, 1.5) * (1.0 - progress),
                *hit_tank,
            )
        }
    } else {
        (0.0, false)
    };
    for mut color in &mut overlays {
        let (red, green_blue) = if hit_tank { (1.0, 0.12) } else { (1.0, 1.0) };
        *color = BackgroundColor(Color::srgba(red, green_blue, green_blue, opacity));
    }
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

fn particle_budget(scale: f32) -> usize {
    (10.0 + scale.clamp(0.35, 4.5) * 8.0).round() as usize
}

fn smoke_budget(scale: f32) -> usize {
    (1.0 + scale.clamp(0.35, 4.5) * 1.5).round() as usize
}

fn cosmetic_fraction(serial: u64, index: u64) -> f32 {
    let value = serial
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(index.wrapping_mul(1_442_695_040_888_963_407));
    ((value >> 40) as f32) / ((1_u32 << 24) as f32)
}

fn effect_has_expired(elapsed_seconds: f32, lifetime_seconds: f32) -> bool {
    elapsed_seconds >= lifetime_seconds
}

fn smoke_scale(initial_scale: f32, elapsed_seconds: f32, lifetime_seconds: f32) -> Vec3 {
    let progress = (elapsed_seconds / lifetime_seconds).clamp(0.0, 1.0);
    Vec3::new(1.25, 0.72, 1.25) * initial_scale * (1.0 + progress * 1.7)
}

fn spawn_impact_effects(
    mut latest: ResMut<LatestTerrainImpact>,
    assets: Res<ImpactEffectAssets>,
    particles: Query<Entity, With<ImpactParticle>>,
    smoke: Query<Entity, With<SmokePuff>>,
    mut commands: Commands,
) {
    let requests = std::mem::take(&mut latest.effect_requests);
    let mut remaining_particles =
        MAX_ACTIVE_IMPACT_PARTICLES.saturating_sub(particles.iter().count());
    let mut remaining_smoke = MAX_ACTIVE_SMOKE_PUFFS.saturating_sub(smoke.iter().count());
    for request in requests {
        let particle_count = particle_budget(request.scale)
            .min(32)
            .min(remaining_particles);
        remaining_particles -= particle_count;
        for index in 0..particle_count {
            let a = cosmetic_fraction(request.serial, index as u64) * std::f32::consts::TAU;
            let speed = 4.0
                + request.scale
                    * (2.0 + cosmetic_fraction(request.serial, index as u64 + 31) * 3.0);
            let velocity = Vec3::new(a.cos() * speed, 3.6 + request.scale * 1.8, a.sin() * speed);
            let material = if index % 2 == 0 {
                assets.hot_material.clone()
            } else {
                assets.dirt_material.clone()
            };
            commands.spawn((
                Name::new("Impact debris"),
                ImpactParticle {
                    velocity,
                    elapsed_seconds: 0.0,
                    lifetime_seconds: 0.85 + request.scale * 0.16,
                    initial_scale: 0.85 + request.scale * 0.18,
                },
                Mesh3d(assets.particle_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(
                    to_bevy_position(request.position)
                        + Vec3::new(a.cos() * 0.24, 0.38, a.sin() * 0.24),
                ),
            ));
        }
        let smoke_count = smoke_budget(request.scale).min(remaining_smoke);
        remaining_smoke -= smoke_count;
        for index in 0..smoke_count {
            let fraction = cosmetic_fraction(request.serial, index as u64 + 91);
            commands.spawn((
                Name::new("Impact smoke"),
                SmokePuff {
                    velocity: Vec3::new(
                        (fraction - 0.5) * 0.35,
                        0.32 + fraction * 0.35,
                        (0.5 - fraction) * 0.25,
                    ),
                    elapsed_seconds: 0.0,
                    lifetime_seconds: 1.8 + request.scale * 0.55,
                    initial_scale: 0.45 + request.scale * 0.28,
                },
                Mesh3d(assets.smoke_mesh.clone()),
                MeshMaterial3d(assets.smoke_material.clone()),
                Transform::from_translation(to_bevy_position(request.position) + Vec3::Y * 0.2),
            ));
        }
    }
}

fn update_impact_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut ImpactParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in &mut particles {
        particle.elapsed_seconds += time.delta_secs();
        if effect_has_expired(particle.elapsed_seconds, particle.lifetime_seconds) {
            commands.entity(entity).despawn();
            continue;
        }
        particle.velocity.y -= PARTICLE_GRAVITY * time.delta_secs();
        transform.translation += particle.velocity * time.delta_secs();
        transform.scale = Vec3::splat(
            particle.initial_scale * (1.0 - particle.elapsed_seconds / particle.lifetime_seconds),
        );
    }
}

fn update_smoke_puffs(
    time: Res<Time>,
    wind: Res<BattlefieldWind>,
    mut commands: Commands,
    mut smoke: Query<(Entity, &mut SmokePuff, &mut Transform)>,
) {
    let drift = wind.0.horizontal_acceleration();
    for (entity, mut puff, mut transform) in &mut smoke {
        puff.elapsed_seconds += time.delta_secs();
        if effect_has_expired(puff.elapsed_seconds, puff.lifetime_seconds) {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation +=
            (puff.velocity + Vec3::new(drift.x * 0.08, 0.0, drift.z * 0.08)) * time.delta_secs();
        transform.scale = smoke_scale(
            puff.initial_scale,
            puff.elapsed_seconds,
            puff.lifetime_seconds,
        );
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
    track: Handle<Mesh>,
    firing_origin_marker: Handle<Mesh>,
}

#[derive(Resource)]
struct TankPresentationAssets {
    materials: Vec<Handle<StandardMaterial>>,
    track_material: Handle<StandardMaterial>,
    barrel_material: Handle<StandardMaterial>,
    firing_origin_material: Handle<StandardMaterial>,
}

#[allow(clippy::too_many_arguments)]
fn spawn_tank(
    commands: &mut Commands,
    tank: Tank,
    meshes: &TankMeshes,
    material: Handle<StandardMaterial>,
    track_material: Handle<StandardMaterial>,
    barrel_material: Handle<StandardMaterial>,
    firing_origin_material: Handle<StandardMaterial>,
    firing: TankFiringRepresentation,
) {
    let position = tank.pose.position;
    let firing_origin = firing.muzzle_position;
    let player_name = format!("Player {} tank", tank.owner.0);
    let model = TankVisualModel::for_player(tank.owner);
    let (body_scale, turret_scale, body_offset) = model.dimensions();

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
                    .with_translation(Vec3::new(0.0, 0.4, body_offset))
                    .with_scale(body_scale),
                TankBody(tank.owner),
            ));
            for side in [-1.0, 1.0] {
                tank_parent.spawn((
                    Mesh3d(meshes.track.clone()),
                    MeshMaterial3d(track_material.clone()),
                    direction_transform(tank.pose.body_forward)
                        .with_translation(Vec3::new(side * 0.76 * body_scale.x, 0.25, body_offset))
                        .with_scale(Vec3::new(body_scale.x, body_scale.y, body_scale.z)),
                ));
            }

            let mut turret_entity = tank_parent.spawn((
                Mesh3d(meshes.turret.clone()),
                MeshMaterial3d(material.clone()),
                direction_transform(tank.pose.turret_forward)
                    .with_translation(Vec3::new(0.0, 1.0, body_offset * 0.3))
                    .with_scale(turret_scale),
                TankTurret(tank.owner),
            ));
            turret_entity.with_children(|turret| {
                turret.spawn((
                    Mesh3d(meshes.barrel.clone()),
                    MeshMaterial3d(barrel_material.clone()),
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

impl TankVisualModel {
    const ALL: [Self; 5] = [
        Self::Classic,
        Self::Heavy,
        Self::LowProfile,
        Self::Compact,
        Self::Angular,
    ];

    /// This is a pure cosmetic assignment: it cannot consume the match's authoritative streams.
    fn for_player(player: PlayerId) -> Self {
        Self::ALL[(player.0.saturating_sub(1) as usize) % Self::ALL.len()]
    }

    fn dimensions(self) -> (Vec3, Vec3, f32) {
        match self {
            Self::Classic => (Vec3::ONE, Vec3::ONE, 0.0),
            Self::Heavy => (
                Vec3::new(1.28, 1.15, 1.05),
                Vec3::new(1.22, 1.25, 1.12),
                0.0,
            ),
            Self::LowProfile => (
                Vec3::new(0.95, 0.68, 1.35),
                Vec3::new(1.12, 0.65, 1.15),
                -0.12,
            ),
            Self::Compact => (
                Vec3::new(0.78, 1.05, 0.78),
                Vec3::new(0.82, 1.35, 0.82),
                0.12,
            ),
            Self::Angular => (
                Vec3::new(1.12, 0.82, 1.18),
                Vec3::new(0.78, 0.9, 1.28),
                -0.08,
            ),
        }
    }
}

#[allow(clippy::too_many_arguments)]
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
    movement_frame: Res<MovementFrame>,
    mut aim_reset: ResMut<CameraAimReset>,
    weapon_slots: Query<&Interaction, With<WeaponSlot>>,
    camera: Single<(&mut Transform, &mut BattlefieldCamera)>,
) {
    let (tanks, turn, flight, presentation) = gameplay;
    let (mut transform, mut controller) = camera.into_inner();
    let intent = camera_presentation_intent(
        *presentation,
        turn.0.clone(),
        flight.0.and_then(FiredShot::presentation_projectile),
    );
    if aim_reset.0 && matches!(intent, CameraPresentationIntent::ActivePlayer(_)) {
        let pose = camera_pose_for_intent(intent, tanks.0.clone(), turn.0.clone());
        controller.target = pose.target;
        controller.yaw = pose.yaw;
        controller.pitch = pose.pitch;
        controller.distance = pose.distance;
        controller.desired_pose = pose;
        controller.tracked_aim_yaw = active_aim_yaw(intent, turn.0.clone());
        aim_reset.0 = false;
    }
    if controller.presentation_intent != Some(intent)
        || matches!(
            intent,
            CameraPresentationIntent::HumanShotFollow(_)
                | CameraPresentationIntent::AiTacticalShot { .. }
        )
        || matches!(intent, CameraPresentationIntent::MovementPlayer(_))
    {
        controller.presentation_intent = Some(intent);
        controller.desired_pose = match intent {
            CameraPresentationIntent::MovementPlayer(player) => movement_camera_pose(
                tank_for_player(&tanks.0, player),
                movement_frame
                    .0
                    .unwrap_or(tank_for_player(&tanks.0, player).pose.body_forward),
            ),
            _ => camera_pose_for_intent(intent, tanks.0.clone(), turn.0.clone()),
        };
        controller.tracked_aim_yaw = active_aim_yaw(intent, turn.0.clone());
    } else if let Some(aim_yaw) = active_aim_yaw(intent, turn.0.clone()) {
        if let Some(previous_aim_yaw) = controller.tracked_aim_yaw {
            controller.desired_pose.yaw += shortest_angle_delta(previous_aim_yaw, aim_yaw);
        }
        controller.tracked_aim_yaw = Some(aim_yaw);
    }

    if matches!(intent, CameraPresentationIntent::ActivePlayer(_))
        && mouse_buttons.pressed(MouseButton::Left)
        && weapon_slots
            .iter()
            .all(|interaction| *interaction == Interaction::None)
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

fn movement_camera_pose(tank: Tank, forward: HorizontalDirection) -> CameraPose {
    CameraPose {
        target: clamp_camera_target(to_bevy_position(tank.pose.position) + Vec3::Y * 1.4),
        yaw: (-forward.x).atan2(-forward.z),
        pitch: -0.26,
        distance: 10.0,
    }
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
            if turn.is_moving() {
                CameraPresentationIntent::MovementPlayer(turn.current_player)
            } else {
                CameraPresentationIntent::ActivePlayer(turn.current_player)
            }
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
        CameraPresentationIntent::MovementPlayer(player) => {
            let tank = tank_for_player(tanks, player);
            CameraPose {
                target: clamp_camera_target(to_bevy_position(tank.pose.position) + Vec3::Y * 1.4),
                yaw: (-tank.pose.body_forward.x).atan2(-tank.pose.body_forward.z),
                pitch: -0.26,
                distance: 10.0,
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

fn create_battlefield_mesh(terrain: &BattlefieldTerrain, palette: PaletteProfile) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, terrain.mesh_positions())
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, terrain.mesh_colours(palette))
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, terrain.mesh_uvs())
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
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, horizon.mesh_uvs())
    .with_inserted_indices(Indices::U32(horizon.indices().to_vec()))
    .with_computed_smooth_normals()
}

fn sky_colour() -> Color {
    Color::srgb(0.26, 0.55, 0.86)
}

fn sky_colour_for(profile: SkyProfile) -> Color {
    match profile {
        SkyProfile::Clear => sky_colour(),
        SkyProfile::MoonStars => Color::srgb(0.008, 0.012, 0.04),
        SkyProfile::Storm => Color::srgb(0.12, 0.18, 0.27),
        SkyProfile::CrusherHaze => Color::srgb(0.62, 0.25, 0.08),
    }
}

/// Fog lets the render-only horizon recede into each environment's sky, avoiding a visible
/// tiled plane while preserving crisp terrain and tank presentation inside the combat arena.
fn distance_fog_for(profile: SkyProfile) -> DistanceFog {
    let (start, end) = match profile {
        SkyProfile::Clear => (72.0, 190.0),
        SkyProfile::MoonStars => (58.0, 150.0),
        SkyProfile::Storm => (42.0, 135.0),
        SkyProfile::CrusherHaze => (52.0, 155.0),
    };
    DistanceFog {
        color: sky_colour_for(profile),
        directional_light_color: Color::NONE,
        falloff: FogFalloff::Linear { start, end },
        ..default()
    }
}

fn terrain_texture_for(
    palette: PaletteProfile,
    textures: &TerrainTextureAssets,
) -> Option<Handle<Image>> {
    match palette {
        PaletteProfile::Earth => Some(textures.earth.clone()),
        PaletteProfile::Moon => Some(textures.moon.clone()),
        PaletteProfile::Crusher => Some(textures.crusher.clone()),
        PaletteProfile::Storm => Some(textures.earth.clone()),
    }
}

/// The combat square occupies one tile; the render-only skirt continues that grid into the
/// distance. Repetition is explicit because Bevy's default image sampler clamps at a texture's
/// edge, which would otherwise smear a single border texel over the entire background.
fn load_repeating_terrain_texture(asset_server: &AssetServer, path: &'static str) -> Handle<Image> {
    asset_server.load_with_settings(path, |settings: &mut ImageLoaderSettings| {
        let mut sampler = ImageSamplerDescriptor::linear();
        sampler.set_address_mode(ImageAddressMode::Repeat);
        settings.sampler = ImageSampler::Descriptor(sampler);
    })
}

/// Palette-specific albedos preserve the visual promise of each environment without letting a
/// green Earth texture bleach Moon or Crusher into the same world.
fn sync_terrain_textures(
    gate: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    mut current: ResMut<TerrainPresentation>,
    textures: Res<TerrainTextureAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    battlefield: Query<&MeshMaterial3d<StandardMaterial>, With<BattlefieldVisual>>,
) {
    let palette = configuration.0.environment.palette();
    if !gate.started || current.0 == palette {
        return;
    }
    current.0 = palette;
    let texture = terrain_texture_for(palette, &textures);
    for handle in &battlefield {
        if let Some(material) = materials.get_mut(&handle.0) {
            material.base_color_texture = texture.clone();
        }
    }
}

/// Sky objects are presentation-only. Rebuilding them at the setup boundary keeps Moon's calm,
/// cloudless night independent from terrain and simulation state.
#[allow(clippy::type_complexity)]
fn sync_sky_presentation(
    gate: Res<MatchSetupGate>,
    configuration: Res<PendingMatchConfiguration>,
    mut current: ResMut<SkyPresentation>,
    mut clear_colour: ResMut<ClearColor>,
    assets: (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
        Query<&mut DistanceFog, With<BattlefieldCamera>>,
    ),
    clouds: Query<Entity, With<CloudVisual>>,
    mut commands: Commands,
) {
    if !gate.started {
        return;
    }
    let selected = configuration.0.environment.sky();
    if current.0 == selected {
        return;
    }
    let (mut meshes, mut materials, mut fog) = assets;
    current.0 = selected;
    clear_colour.0 = sky_colour_for(selected);
    for mut fog in &mut fog {
        *fog = distance_fog_for(selected);
    }
    for entity in &clouds {
        commands.entity(entity).despawn();
    }
    spawn_clouds(&mut commands, &mut meshes, &mut materials, selected);
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
    sky: SkyProfile,
) {
    if sky == SkyProfile::MoonStars {
        let mesh = meshes.add(Sphere::new(0.18));
        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::WHITE.into(),
            unlit: true,
            ..default()
        });
        for index in 0..72 {
            let angle = index as f32 * 2.399_963;
            let radius = 90.0 + (index % 9) as f32 * 8.0;
            commands.spawn((
                CloudVisual,
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(
                    angle.cos() * radius,
                    35.0 + (index % 7) as f32 * 5.0,
                    angle.sin() * radius,
                ),
            ));
        }
        return;
    }
    let mesh = meshes.add(Sphere::new(1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.94, 0.97, 1.0),
        emissive: Color::srgb(0.94, 0.97, 1.0).into(),
        unlit: true,
        ..default()
    });
    for (position, scale) in cloud_positions() {
        commands.spawn((
            CloudVisual,
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
    use crate::environment::TerrainProfile;
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
        let terrain = BattlefieldTerrain::generated(BattlefieldSeed(17), TerrainProfile::Standard);
        let horizon =
            VisualHorizon::from_terrain(&terrain, BattlefieldSeed(17), PaletteProfile::Earth);
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
    fn cosmetic_tank_models_repeat_only_after_the_five_unique_silhouettes() {
        let models = (1..=8)
            .map(|id| TankVisualModel::for_player(PlayerId(id)))
            .collect::<Vec<_>>();
        assert_eq!(models[..5], TankVisualModel::ALL);
        assert_eq!(models[5], TankVisualModel::Classic);
        assert_eq!(models[7], TankVisualModel::LowProfile);
    }

    #[test]
    fn cosmetic_models_do_not_perturb_authoritative_world_generation() {
        let seed = BattlefieldSeed(91);
        let players = (1..=8).map(PlayerId).collect::<Vec<_>>();
        let before = generate_match_world(seed, &players, EnvironmentPreset::Earth);
        let _models = players
            .iter()
            .copied()
            .map(TankVisualModel::for_player)
            .collect::<Vec<_>>();
        let after = generate_match_world(seed, &players, EnvironmentPreset::Earth);
        assert_eq!(before, after);
    }

    #[test]
    fn round_seeds_reproduce_one_round_and_vary_across_consecutive_rounds() {
        let root = BattlefieldSeed(91);
        let players = (1..=8).map(PlayerId).collect::<Vec<_>>();
        let first = generate_match_world(round_seed(root, 1), &players, EnvironmentPreset::Earth);
        assert_eq!(
            first,
            generate_match_world(round_seed(root, 1), &players, EnvironmentPreset::Earth)
        );

        let rounds = (1..=5)
            .map(|round| {
                generate_match_world(round_seed(root, round), &players, EnvironmentPreset::Earth)
            })
            .collect::<Vec<_>>();
        for pair in rounds.windows(2) {
            assert_ne!(pair[0].0, pair[1].0);
        }
    }

    #[test]
    fn generated_rounds_always_publish_valid_full_health_starts() {
        for root in 0..100 {
            for count in 2..=8 {
                let players = (1..=count).map(PlayerId).collect::<Vec<_>>();
                let (terrain, tanks, _, _) = generate_match_world(
                    round_seed(BattlefieldSeed(root), 1),
                    &players,
                    EnvironmentPreset::Earth,
                );
                assert_eq!(tanks.len(), count as usize);
                for (index, tank) in tanks.iter().enumerate() {
                    assert_eq!(tank.health, MAX_HEALTH);
                    assert!(crate::battlefield::is_dry_and_gentle(
                        &terrain,
                        tank.pose.position.x,
                        tank.pose.position.z,
                    ));
                    assert_eq!(
                        tank.pose.position.y,
                        terrain.height(tank.pose.position.x, tank.pose.position.z)
                    );
                    for other in &tanks[..index] {
                        assert!(
                            (tank.pose.position.x - other.pose.position.x)
                                .hypot(tank.pose.position.z - other.pose.position.z)
                                >= crate::tank::MINIMUM_START_SEPARATION
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn impact_effect_budgets_are_monotonic_and_bounded() {
        assert!(particle_budget(1.5) > particle_budget(1.0));
        assert!(particle_budget(4.5) > particle_budget(1.5));
        assert!(smoke_budget(4.5) > smoke_budget(1.0));
        assert!(particle_budget(4.5) <= MAX_ACTIVE_IMPACT_PARTICLES);
        assert!(smoke_budget(4.5) <= MAX_ACTIVE_SMOKE_PUFFS);
        assert_eq!(cosmetic_fraction(4, 7), cosmetic_fraction(4, 7));
    }

    #[test]
    fn smoke_lifecycle_is_finite_and_expands_without_gameplay_state() {
        assert!(!effect_has_expired(1.0, 1.5));
        assert!(effect_has_expired(1.5, 1.5));
        assert!(smoke_scale(1.0, 1.0, 2.0).x > smoke_scale(1.0, 0.0, 2.0).x);
        assert!(smoke_budget(4.5) <= MAX_ACTIVE_SMOKE_PUFFS);
    }

    #[test]
    fn cosmetic_impact_queue_is_bounded_without_changing_existing_impact_fields() {
        let impact = WorldPosition {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mut latest = LatestTerrainImpact {
            impact: Some(TerrainImpact { position: impact }),
            explosion_visual_scale: 1.5,
            ..default()
        };
        for _ in 0..MAX_PENDING_IMPACT_EFFECTS + 5 {
            queue_impact_effect(&mut latest, impact, 1.0);
        }
        assert_eq!(latest.effect_requests.len(), MAX_PENDING_IMPACT_EFFECTS);
        assert_eq!(latest.impact, Some(TerrainImpact { position: impact }));
        assert_eq!(latest.explosion_visual_scale, 1.5);
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
    fn movement_mode_maps_arrows_to_hull_relative_cardinal_steps() {
        let cases = [
            (KeyCode::ArrowUp, MovementDirection::NegativeZ),
            (KeyCode::ArrowLeft, MovementDirection::NegativeX),
            (KeyCode::ArrowDown, MovementDirection::PositiveZ),
            (KeyCode::ArrowRight, MovementDirection::PositiveX),
        ];
        for (key, expected) in cases {
            let mut keyboard = ButtonInput::default();
            keyboard.press(key);
            assert_eq!(
                movement_direction(&keyboard, HorizontalDirection::new(0.0, -1.0)),
                Some(expected)
            );
        }

        let mut rotated = ButtonInput::default();
        rotated.press(KeyCode::ArrowUp);
        assert_eq!(
            movement_direction(&rotated, HorizontalDirection::new(1.0, 0.0)),
            Some(MovementDirection::PositiveX)
        );

        let mut retired = ButtonInput::default();
        retired.press(KeyCode::KeyI);
        assert_eq!(
            movement_direction(&retired, HorizontalDirection::new(0.0, -1.0)),
            None
        );
    }

    #[test]
    fn move_completion_accepts_space_or_enter_and_setup_toggle_uses_tab() {
        for key in [KeyCode::Space, KeyCode::Enter] {
            let mut keyboard = ButtonInput::default();
            keyboard.press(key);
            assert!(movement_completion_requested(&keyboard));
        }
        let mut tab = ButtonInput::default();
        tab.press(KeyCode::Tab);
        assert!(setup_controller_toggle_requested(&tab));
        let mut legacy = ButtonInput::default();
        legacy.press(KeyCode::KeyC);
        assert!(!setup_controller_toggle_requested(&legacy));
    }

    #[test]
    fn accounting_text_compacts_all_supported_players_into_one_table() {
        let mut configuration = MatchConfiguration::default();
        configuration.set_player_count(8).unwrap();
        let session = GameSession::new(configuration.players);
        let text = accounting_text(&session);
        assert!(text.contains("PLAYER               DAMAGE  PLACE  TOTAL  WALLET"));
        for player in &session.players {
            assert!(text.contains(&player.configuration.display_name));
        }
        assert!(text.contains("PRESS ENTER FOR THE ARMOURY"));
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
            Some(MovementRejection::Bounds),
        );
        assert_eq!(moving_view.action, HudAction::Moving);
        assert_eq!(moving_view.movement, Some(MovementRejection::Bounds));

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
                .add_round(WeaponId::HighExplosive)
        );
        assert!(
            weapons
                .for_player_mut(PlayerId::One)
                .add_round(WeaponId::HighExplosive)
        );

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
            "CLICK WEAPON BAR | M MOVE | SPACE FIRE\nLEFT-DRAG CAMERA | SCROLL ZOOM | ARROWS AIM | CURVE: ARROWS STEER IN FLIGHT | -/= POWER"
        );

        assert!(
            weapons
                .for_player_mut(PlayerId::One)
                .add_round(WeaponId::HeavyShell)
        );
        assert!(
            weapons
                .for_player_mut(PlayerId::One)
                .add_round(WeaponId::HeavyShell)
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
        let first = wind_from_seed(42, (MINIMUM_WIND_STRENGTH, MAXIMUM_WIND_STRENGTH));
        let second = wind_from_seed(42, (MINIMUM_WIND_STRENGTH, MAXIMUM_WIND_STRENGTH));
        let different = wind_from_seed(43, (MINIMUM_WIND_STRENGTH, MAXIMUM_WIND_STRENGTH));

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
        assert!(turn.accept_movement_step());
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
