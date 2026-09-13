//! Regression coverage for the executable's orchestration layer.

use super::*;
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
    let horizon = VisualHorizon::from_terrain(&terrain, BattlefieldSeed(17), PaletteProfile::Earth);
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
    let heavy_view = tactical_hud_view(turn.clone(), tanks, &configuration, weapons, wind, None);
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
