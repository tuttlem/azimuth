use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_proof_scene)
        .run();
}

fn spawn_proof_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.0, 6.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(16.0, 16.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.18, 0.32, 0.18))),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.25, 0.15))),
        Transform::from_xyz(0.0, 0.75, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.75, 0.75, 0.75))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.45, 0.85))),
        Transform::from_xyz(-3.0, 0.375, -2.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 4.0),
    ));
}
