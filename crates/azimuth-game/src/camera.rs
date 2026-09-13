//! Tactical-camera behaviour and its setup-screen presentation orbit.
//!
//! The camera deliberately reads gameplay state but never writes it: this keeps visual
//! transitions from affecting aiming, projectile simulation, or turn progression.

use crate::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn update_battlefield_camera(
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
        if let Some(previous) = controller.tracked_aim_yaw {
            controller.desired_pose.yaw += shortest_angle_delta(previous, aim_yaw);
        }
        controller.tracked_aim_yaw = Some(aim_yaw);
    }
    if matches!(intent, CameraPresentationIntent::ActivePlayer(_))
        && mouse_buttons.pressed(MouseButton::Left)
        && weapon_slots
            .iter()
            .all(|interaction| *interaction == Interaction::None)
    {
        controller.yaw -= mouse_motion.delta.x * CAMERA_ORBIT_SENSITIVITY;
        controller.pitch =
            clamp_camera_pitch(controller.pitch - mouse_motion.delta.y * CAMERA_ORBIT_SENSITIVITY);
        controller.desired_pose.yaw = controller.yaw;
        controller.desired_pose.pitch = controller.pitch;
    }
    for wheel in mouse_wheel.read() {
        if matches!(intent, CameraPresentationIntent::ActivePlayer(_)) {
            controller.distance = (controller.distance - wheel.y * CAMERA_ZOOM_SPEED)
                .clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
            controller.desired_pose.distance = controller.distance;
        }
    }
    interpolate_camera_pose(&mut controller, time.delta_secs());
    *transform = camera_transform(&controller);
}

/// Setup stays visually alive by orbiting the prepared battlefield. Tactical control resumes at
/// the exact moment the match starts.
pub(super) fn rotate_menu_camera(
    setup: Res<MatchSetupGate>,
    time: Res<Time>,
    camera: Single<(&mut Transform, &mut BattlefieldCamera)>,
) {
    if setup.started {
        return;
    }
    let (mut transform, mut controller) = camera.into_inner();
    controller.target = Vec3::new(0.0, 5.0, 0.0);
    controller.yaw += MENU_ORBIT_RADIANS_PER_SECOND * time.delta_secs();
    controller.pitch = -0.42;
    controller.distance = 108.0;
    controller.desired_pose = CameraPose {
        target: controller.target,
        yaw: controller.yaw,
        pitch: controller.pitch,
        distance: controller.distance,
    };
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
pub(super) fn clamp_camera_pitch(pitch: f32) -> f32 {
    pitch.clamp(CAMERA_MIN_PITCH, CAMERA_MAX_PITCH)
}
pub(super) fn camera_presentation_intent(
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
pub(super) fn camera_pose_for_intent(
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
        CameraPresentationIntent::Impact(position) | CameraPresentationIntent::Result(position) => {
            impact_camera_pose(position)
        }
    }
}
pub(super) fn human_shot_camera_pose(projectile: Projectile) -> CameraPose {
    let velocity = Vec3::new(projectile.velocity.x, 0.0, projectile.velocity.z);
    let forward = if velocity.length_squared() > 0.0 {
        velocity.normalize()
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
pub(super) fn ai_tactical_camera_pose(
    shooter: PlayerId,
    projectile: Projectile,
    tanks: &[Tank],
) -> CameraPose {
    let shooter_position = to_bevy_position(tank_for_player(tanks, shooter).pose.position);
    let projectile_position = to_bevy_position(projectile.position);
    let raw_direction = (projectile_position - shooter_position).normalize_or_zero();
    let direction = if raw_direction.length_squared() > 0.0 {
        raw_direction
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
pub(super) fn interpolate_camera_pose(camera: &mut BattlefieldCamera, delta_seconds: f32) {
    let factor = camera_transition_factor(delta_seconds);
    camera.target = camera.target.lerp(camera.desired_pose.target, factor);
    camera.yaw += shortest_angle_delta(camera.yaw, camera.desired_pose.yaw) * factor;
    camera.pitch = camera.pitch.lerp(camera.desired_pose.pitch, factor);
    camera.distance = camera.distance.lerp(camera.desired_pose.distance, factor);
}
fn camera_transition_factor(delta_seconds: f32) -> f32 {
    1.0 - (-CAMERA_TRANSITION_SPEED * delta_seconds.max(0.0)).exp()
}
pub(super) fn shortest_angle_delta(from: f32, to: f32) -> f32 {
    (to - from + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}
pub(super) fn camera_transform(camera: &BattlefieldCamera) -> Transform {
    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    Transform {
        translation: camera.target - rotation * Vec3::NEG_Z * camera.distance,
        rotation,
        ..default()
    }
}
pub(super) fn clamp_camera_target(target: Vec3) -> Vec3 {
    Vec3::new(
        target.x.clamp(-HALF_EXTENT, HALF_EXTENT),
        target.y,
        target.z.clamp(-HALF_EXTENT, HALF_EXTENT),
    )
}
