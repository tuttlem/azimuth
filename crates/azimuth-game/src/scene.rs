//! Terrain, horizon, sky, and boulder presentation.

use crate::*;

pub(super) fn create_battlefield_mesh(
    terrain: &BattlefieldTerrain,
    palette: PaletteProfile,
) -> Mesh {
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

pub(super) fn create_horizon_mesh(horizon: &VisualHorizon) -> Mesh {
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

pub(super) fn sky_colour() -> Color {
    Color::srgb(0.26, 0.55, 0.86)
}

pub(super) fn sky_colour_for(profile: SkyProfile) -> Color {
    match profile {
        SkyProfile::Clear => sky_colour(),
        SkyProfile::MoonStars => Color::srgb(0.008, 0.012, 0.04),
        SkyProfile::Storm => Color::srgb(0.12, 0.18, 0.27),
        SkyProfile::CrusherHaze => Color::srgb(0.62, 0.25, 0.08),
    }
}

/// Fog lets the render-only horizon recede into each environment's sky, avoiding a visible
/// tiled plane while preserving crisp terrain and tank presentation inside the combat arena.
pub(super) fn distance_fog_for(profile: SkyProfile) -> DistanceFog {
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

pub(super) fn terrain_texture_for(
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
pub(super) fn load_repeating_terrain_texture(
    asset_server: &AssetServer,
    path: &'static str,
) -> Handle<Image> {
    asset_server.load_with_settings(path, |settings: &mut ImageLoaderSettings| {
        let mut sampler = ImageSamplerDescriptor::linear();
        sampler.set_address_mode(ImageAddressMode::Repeat);
        settings.sampler = ImageSampler::Descriptor(sampler);
    })
}

/// Palette-specific albedos preserve the visual promise of each environment without letting a
/// green Earth texture bleach Moon or Crusher into the same world.
pub(super) fn sync_terrain_textures(
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
pub(super) fn sync_sky_presentation(
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

pub(super) fn cloud_positions() -> [(Vec3, Vec3); 6] {
    [
        (Vec3::new(-76.0, 24.0, -76.0), Vec3::new(11.0, 2.4, 5.0)),
        (Vec3::new(-63.0, 25.0, -72.0), Vec3::new(7.0, 1.8, 3.5)),
        (Vec3::new(72.0, 30.0, -92.0), Vec3::new(12.0, 2.5, 5.5)),
        (Vec3::new(87.0, 30.5, -90.0), Vec3::new(7.0, 1.8, 3.5)),
        (Vec3::new(-118.0, 26.0, 60.0), Vec3::new(11.0, 2.2, 4.5)),
        (Vec3::new(105.0, 28.0, 76.0), Vec3::new(9.0, 2.0, 4.0)),
    ]
}

pub(super) fn spawn_clouds(
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

pub(super) fn spawn_buildings(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    buildings: &[BuildingPlacement],
    material: Handle<StandardMaterial>,
) {
    for building in buildings {
        let main_scale = Vec3::new(
            building.width * 0.55,
            building.height * 0.5,
            building.depth * 0.55,
        );
        let rock_mesh = meshes.add(Sphere::new(1.0));
        commands
            .spawn((
                Name::new("Indestructible boulder"),
                BuildingVisual,
                BoulderVisual {
                    half_height: building.height * 0.5,
                    radius_x: building.width * 0.62,
                    radius_z: building.depth * 0.62,
                    vertical_velocity: 0.0,
                },
                GlobalTransform::default(),
                Visibility::Inherited,
                InheritedVisibility::VISIBLE,
                ViewVisibility::default(),
                Transform::from_xyz(
                    building.position.x,
                    building.position.y + building.height / 2.0,
                    building.position.z,
                )
                .with_rotation(Quat::from_rotation_y(building.yaw_radians)),
            ))
            .with_children(|boulder| {
                boulder.spawn((
                    Mesh3d(rock_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_scale(main_scale),
                ));
                // Two smaller lobes break the perfect ellipsoid into a smooth, natural rock.
                boulder.spawn((
                    Mesh3d(rock_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(
                        building.width * 0.26,
                        -building.height * 0.08,
                        building.depth * 0.12,
                    )
                    .with_scale(main_scale * 0.58),
                ));
                boulder.spawn((
                    Mesh3d(rock_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(
                        -building.width * 0.20,
                        building.height * 0.10,
                        -building.depth * 0.20,
                    )
                    .with_scale(main_scale * 0.43),
                ));
            });
    }
}

pub(super) fn boulder_impact(
    advance: ProjectileAdvance,
    position: WorldPosition,
    boulders: &Query<(&BoulderVisual, &Transform), With<BuildingVisual>>,
) -> ProjectileAdvance {
    if !matches!(advance, ProjectileAdvance::Active) {
        return advance;
    }
    boulders
        .iter()
        .find_map(|(boulder, transform)| {
            let dx = (position.x - transform.translation.x) / boulder.radius_x;
            let dz = (position.z - transform.translation.z) / boulder.radius_z;
            let dy = (position.y - transform.translation.y) / boulder.half_height;
            (dx * dx + dy * dy + dz * dz <= 1.0)
                .then_some(ProjectileAdvance::TerrainImpact(TerrainImpact { position }))
        })
        .unwrap_or(advance)
}

pub(super) fn settle_boulders(
    setup: Res<MatchSetupGate>,
    terrain: Res<BattlefieldState>,
    gravity: Res<BattlefieldGravity>,
    time: Res<Time>,
    mut boulders: Query<(&mut BoulderVisual, &mut Transform), With<BuildingVisual>>,
) {
    if !setup.started {
        return;
    }
    for (mut boulder, mut transform) in &mut boulders {
        let ground = terrain
            .0
            .height(transform.translation.x, transform.translation.z);
        let resting_y = ground + boulder.half_height;
        if transform.translation.y > resting_y + 0.02 {
            boulder.vertical_velocity -= gravity.0.downward_acceleration() * time.delta_secs();
            transform.translation.y = (transform.translation.y
                + boulder.vertical_velocity * time.delta_secs())
            .max(resting_y);
        } else {
            transform.translation.y = resting_y;
            boulder.vertical_velocity = 0.0;
        }
    }
}
