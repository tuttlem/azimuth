//! Camera and combat-flow regression coverage.

use super::*;
use crate::battlefield::Crater;
use crate::tank::initial_tanks;

fn basic_fired_shot(projectile: Projectile) -> FiredShot {
    FiredShot::new(weapon_definition(WeaponId::BasicShell), projectile)
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
