use std::f32::consts::FRAC_PI_2;

use bevy::{
    asset::RenderAssetUsages,
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

const BATTLEFIELD_HALF_EXTENT: f32 = 20.0;
const TERRAIN_CELLS_PER_SIDE: u32 = 20;
const CAMERA_MIN_DISTANCE: f32 = 8.0;
const CAMERA_MAX_DISTANCE: f32 = 60.0;
const CAMERA_PAN_SPEED: f32 = 12.0;
const CAMERA_ORBIT_SENSITIVITY: f32 = 0.005;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_PITCH_LIMIT: f32 = FRAC_PI_2 - 0.1;

#[derive(Component)]
struct BattlefieldCamera {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
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
        .add_systems(Startup, spawn_battlefield_scene)
        .add_systems(Update, (update_battlefield_camera, draw_world_axes))
        .run();
}

fn spawn_battlefield_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        target
            .x
            .clamp(-BATTLEFIELD_HALF_EXTENT, BATTLEFIELD_HALF_EXTENT),
        target.y,
        target
            .z
            .clamp(-BATTLEFIELD_HALF_EXTENT, BATTLEFIELD_HALF_EXTENT),
    )
}

fn create_battlefield_mesh() -> Mesh {
    let cells = TERRAIN_CELLS_PER_SIDE as usize;
    let step = BATTLEFIELD_HALF_EXTENT * 2.0 / TERRAIN_CELLS_PER_SIDE as f32;
    let mut positions = Vec::with_capacity((cells + 1) * (cells + 1));

    for z_index in 0..=cells {
        for x_index in 0..=cells {
            let x = -BATTLEFIELD_HALF_EXTENT + x_index as f32 * step;
            let z = -BATTLEFIELD_HALF_EXTENT + z_index as f32 * step;
            positions.push([x, terrain_height(x, z), z]);
        }
    }

    let mut indices = Vec::with_capacity(cells * cells * 6);
    for z_index in 0..cells {
        for x_index in 0..cells {
            let lower_left = (z_index * (cells + 1) + x_index) as u32;
            let lower_right = lower_left + 1;
            let upper_left = lower_left + (cells + 1) as u32;
            let upper_right = upper_left + 1;

            indices.extend_from_slice(&[
                lower_left,
                upper_left,
                lower_right,
                lower_right,
                upper_left,
                upper_right,
            ]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(Indices::U32(indices))
    .with_computed_smooth_normals()
}

fn terrain_height(x: f32, z: f32) -> f32 {
    // This is deliberately visual relief, not the beginning of Azimuth's terrain model.
    1.8 * (x * 0.16).sin() * (z * 0.13).cos() + z * 0.08
}

fn draw_world_axes(mut gizmos: Gizmos) {
    gizmos.axes(Transform::IDENTITY, 3.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_height_is_deterministic_and_non_flat() {
        assert_eq!(terrain_height(4.0, -7.0), terrain_height(4.0, -7.0));
        assert_ne!(terrain_height(0.0, 0.0), terrain_height(8.0, 8.0));
    }

    #[test]
    fn battlefield_camera_target_stays_within_visible_bounds() {
        assert_eq!(
            clamp_camera_target(Vec3::new(30.0, 0.0, -30.0)),
            Vec3::new(BATTLEFIELD_HALF_EXTENT, 0.0, -BATTLEFIELD_HALF_EXTENT)
        );
    }
}
