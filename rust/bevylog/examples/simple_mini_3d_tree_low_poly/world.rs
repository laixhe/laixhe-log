//! 世界初始化（Startup）：集中创建共享材质 / 网格（`GameAssets`），
//! 生成地形高度场网格、资源节点、中央仓库与初始殖民者，
//! 并把它们的占地登记到 `WorldGeometry`（后续寻路与放置判定都依赖它）。

use bevy::{light::NotShadowCaster, mesh::Meshable, prelude::*};

#[derive(Component)]
pub struct MainLight;

use crate::{
    building::{
        BuildingEntrance, BuildingVisual, CompletedBuilding, EntranceMarker, Footprint, Profession,
        WorldGeometry, footprint_polygon, resource_obstacle_polygon,
    },
    camera,
    colonist::{Colonist, ColonistState},
    resources::{
        CENTRAL_STORAGE_CAPACITY, COLONIST_CARRY_CAPACITY, CentralStorage, Inventory,
        PublicInventory,
    },
    terrain::{
        GeneratedResource, GeneratedTerrain, TerrainGenerationConfig, TerrainKind, TerrainSeed,
        generate_terrain, terrain_height, terrain_kind_at,
    },
    types::{
        BuildingKind, CELL_SIZE, ResourceKind, building_color, entrance_local_offset,
        entrance_world_position,
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

#[derive(Component)]
pub struct ResourceNode {
    pub kind: ResourceKind,
    pub amount: i32,
}

#[derive(Resource, Clone)]
pub struct GameAssets {
    pub cube_mesh: Handle<Mesh>,
    pub preview_valid_material: Handle<StandardMaterial>,
    pub preview_invalid_material: Handle<StandardMaterial>,
    pub blueprint_material: Handle<StandardMaterial>,
    pub house_material: Handle<StandardMaterial>,
    pub storage_material: Handle<StandardMaterial>,
    pub woodcutter_material: Handle<StandardMaterial>,
    pub gatherer_material: Handle<StandardMaterial>,
    pub chopping_yard_material: Handle<StandardMaterial>,
    pub road_material: Handle<StandardMaterial>,
    pub entrance_material: Handle<StandardMaterial>,
    pub farm_blueprint_material: Handle<StandardMaterial>,
    pub farm_soil_material: Handle<StandardMaterial>,
    pub crop_mesh: Handle<Mesh>,
    pub crop_material: Handle<StandardMaterial>,
    pub colonist_mesh: Handle<Mesh>,
    pub colonist_material: Handle<StandardMaterial>,
}

impl GameAssets {
    pub fn building_material(&self, kind: BuildingKind) -> Handle<StandardMaterial> {
        match kind {
            BuildingKind::House => self.house_material.clone(),
            BuildingKind::Storage => self.storage_material.clone(),
            BuildingKind::Woodcutter => self.woodcutter_material.clone(),
            BuildingKind::Gatherer => self.gatherer_material.clone(),
            BuildingKind::ChoppingYard => self.chopping_yard_material.clone(),
            BuildingKind::Road => self.road_material.clone(),
        }
    }
}

pub fn setup_scene(
    mut commands: Commands,
    terrain_config: Res<TerrainGenerationConfig>,
    terrain_seed: Res<TerrainSeed>,
    mut geometry: ResMut<WorldGeometry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::from_length(1.0));
    let colonist_mesh = meshes.add(Cuboid::new(0.32, 0.64, 0.32));
    let crop_mesh = meshes.add(
        Cylinder::new(crate::farm::CROP_RADIUS, crate::farm::CROP_HEIGHT)
            .mesh()
            .resolution(8),
    );

    let tree_material = materials.add(Color::srgb(0.16, 0.38, 0.16));
    let food_material = materials.add(Color::srgb(0.66, 0.12, 0.18));
    let colonist_material = materials.add(Color::srgb(0.92, 0.72, 0.45));
    let crop_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.34, 0.67, 0.24),
        perceptual_roughness: 0.95,
        ..default()
    });
    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.9,
        ..default()
    });
    let preview_valid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.25, 0.85, 0.55, 0.45),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let preview_invalid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.95, 0.2, 0.16, 0.45),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let blueprint_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.22, 0.48, 0.95, 0.55),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let farm_blueprint_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.42, 0.25, 0.12, 0.7),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.95,
        ..default()
    });
    let farm_soil_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.34, 0.22, 0.12, 0.86),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.95,
        ..default()
    });

    let assets = GameAssets {
        cube_mesh: cube_mesh.clone(),
        preview_valid_material,
        preview_invalid_material,
        blueprint_material,
        house_material: materials.add(building_color(BuildingKind::House)),
        storage_material: materials.add(building_color(BuildingKind::Storage)),
        woodcutter_material: materials.add(building_color(BuildingKind::Woodcutter)),
        gatherer_material: materials.add(building_color(BuildingKind::Gatherer)),
        chopping_yard_material: materials.add(building_color(BuildingKind::ChoppingYard)),
        road_material: materials.add(building_color(BuildingKind::Road)),
        entrance_material: materials.add(Color::srgb(0.95, 0.86, 0.28)),
        farm_blueprint_material,
        farm_soil_material,
        crop_mesh: crop_mesh.clone(),
        crop_material,
        colonist_mesh: colonist_mesh.clone(),
        colonist_material: colonist_material.clone(),
    };

    commands.spawn((
        MainLight,
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let terrain = generate_terrain(*terrain_config);
    spawn_terrain_tiles(
        &mut commands,
        &mut meshes,
        &terrain_material,
        terrain_config.tile_cells,
        &terrain,
        terrain_seed.0,
    );
    spawn_resource_nodes(
        &mut commands,
        &mut geometry,
        &cube_mesh,
        &tree_material,
        &food_material,
        &terrain.resources,
    );
    spawn_central_storage(&mut commands, &mut geometry, &assets, terrain_seed.0);
    spawn_colonists(
        &mut commands,
        &colonist_mesh,
        &colonist_material,
        terrain_seed.0,
    );
    camera::spawn_camera(&mut commands, terrain_seed.0);
    commands.insert_resource(assets);
}

fn spawn_central_storage(
    commands: &mut Commands,
    geometry: &mut WorldGeometry,
    assets: &GameAssets,
    seed: u64,
) {
    let definition = BuildingKind::Storage.definition();
    let position = Vec3::new(0.0, terrain_height(seed, 0.0, 0.0), 0.0);
    let rotation = Quat::IDENTITY;
    let polygon = footprint_polygon(BuildingKind::Storage, position, definition.size, 0.0);
    let direction = BuildingKind::Storage.entrance_direction().unwrap();
    let local_offset = entrance_local_offset(definition.size, direction);
    let entrance_position = entrance_world_position(position, definition.size, 0.0, direction);
    let mut inventory = Inventory::public(CENTRAL_STORAGE_CAPACITY);
    inventory.add(ResourceKind::Wood, 40);
    inventory.add(ResourceKind::Food, 20);

    let entity = commands
        .spawn((
            Transform {
                translation: position,
                rotation,
                scale: Vec3::ONE,
            },
            Visibility::Visible,
            CompletedBuilding {
                kind: BuildingKind::Storage,
            },
            Footprint {
                polygon: polygon.clone(),
                passable: false,
            },
            BuildingEntrance {
                world_position: entrance_position,
                local_offset,
            },
            inventory,
            PublicInventory,
            CentralStorage,
        ))
        .id();

    commands.spawn((
        Mesh3d(assets.cube_mesh.clone()),
        MeshMaterial3d(assets.storage_material.clone()),
        Transform::from_translation(Vec3::new(0.0, definition.height * 0.5, 0.0)).with_scale(
            Vec3::new(
                definition.size.x as f32 * CELL_SIZE * 0.9,
                definition.height,
                definition.size.y as f32 * CELL_SIZE * 0.9,
            ),
        ),
        BuildingVisual { owner: entity },
        ChildOf(entity),
    ));

    commands.spawn((
        Mesh3d(assets.cube_mesh.clone()),
        MeshMaterial3d(assets.entrance_material.clone()),
        Transform::from_translation(Vec3::new(local_offset.x, 0.04, local_offset.z))
            .with_scale(Vec3::new(0.42, 0.08, 0.42)),
        EntranceMarker { owner: entity },
        ChildOf(entity),
    ));

    geometry.occupy_polygon(polygon, entity, false);
    geometry.reserve_entrance_point(entrance_position, entity);
}

fn spawn_terrain_tiles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    terrain_material: &Handle<StandardMaterial>,
    tile_cells: i32,
    terrain: &GeneratedTerrain,
    seed: u64,
) {
    let tile_cells = tile_cells.max(1);
    let tile_size = tile_cells as f32 * CELL_SIZE;

    for tile in &terrain.tiles {
        let mesh = heightfield_tile_mesh(seed, tile.center.x, tile.center.z, tile_size, tile_cells);
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(terrain_material.clone()),
            Transform::from_translation(Vec3::new(tile.center.x, 0.0, tile.center.z)),
            NotShadowCaster,
        ));
    }
}

// 高度场网格：逐格采样地形高度生成顶点（顶点色按植被类型 + 高度微调明暗），
// 并用手工计算的法线，避免依赖 Bevy 自动平滑导致的接缝问题。
fn heightfield_tile_mesh(
    seed: u64,
    center_x: f32,
    center_z: f32,
    tile_size: f32,
    tile_cells: i32,
) -> Mesh {
    let half = tile_size * 0.5;
    let min_x = center_x - half;
    let min_z = center_z - half;
    let verts_per_side = tile_cells.max(1) as u32 + 1;
    let cell_size = tile_size / (verts_per_side - 1) as f32;

    let mut positions = Vec::with_capacity((verts_per_side * verts_per_side) as usize);
    let mut normals = Vec::with_capacity((verts_per_side * verts_per_side) as usize);
    let mut uvs = Vec::with_capacity((verts_per_side * verts_per_side) as usize);
    let mut colors = Vec::with_capacity((verts_per_side * verts_per_side) as usize);
    let mut indices = Vec::new();

    for iz in 0..verts_per_side {
        for ix in 0..verts_per_side {
            let x = min_x + ix as f32 * cell_size;
            let z = min_z + iz as f32 * cell_size;
            let y = terrain_height(seed, x, z);

            let u = ix as f32 / (verts_per_side - 1) as f32;
            let v = iz as f32 / (verts_per_side - 1) as f32;

            positions.push([x - center_x, y, z - center_z]);
            normals.push(terrain_normal(seed, x, z));
            uvs.push([u, v]);
            colors.push(terrain_vertex_color(seed, x, z, y));
        }
    }

    for iz in 0..(verts_per_side - 1) {
        for ix in 0..(verts_per_side - 1) {
            let a = iz * verts_per_side + ix;
            let b = a + 1;
            let c = a + verts_per_side;
            let d = c + 1;

            indices.push(a);
            indices.push(c);
            indices.push(b);

            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(bevy::mesh::Indices::U32(indices))
}

fn terrain_normal(seed: u64, x: f32, z: f32) -> [f32; 3] {
    let sample_dist = 0.5;
    let left = terrain_height(seed, x - sample_dist, z);
    let right = terrain_height(seed, x + sample_dist, z);
    let down = terrain_height(seed, x, z - sample_dist);
    let up = terrain_height(seed, x, z + sample_dist);
    Vec3::new(left - right, sample_dist * 2.0, down - up)
        .normalize_or_zero()
        .to_array()
}

// 顶点色 = 植被基础色 × 高度明暗系数：同一植被类型内，高处稍亮、低处稍暗，
// 让纯程序化地形也有微妙的立体感（不依赖贴图）。
fn terrain_vertex_color(seed: u64, x: f32, z: f32, height: f32) -> [f32; 4] {
    shaded_terrain_color(terrain_base_color(seed, x, z), height)
}

fn terrain_base_color(seed: u64, x: f32, z: f32) -> [f32; 3] {
    match terrain_kind_at(seed, x, z) {
        TerrainKind::Grass => [0.5, 0.57, 0.45],
        TerrainKind::ForestFloor => [0.23, 0.35, 0.22],
        TerrainKind::ForageField => [0.38, 0.5, 0.27],
    }
}

fn shaded_terrain_color(base: [f32; 3], height: f32) -> [f32; 4] {
    let shade = (0.96 + height * 0.012).clamp(0.78, 1.12);
    [base[0] * shade, base[1] * shade, base[2] * shade, 1.0]
}

fn spawn_resource_nodes(
    commands: &mut Commands,
    geometry: &mut WorldGeometry,
    cube_mesh: &Handle<Mesh>,
    tree_material: &Handle<StandardMaterial>,
    food_material: &Handle<StandardMaterial>,
    resources: &[GeneratedResource],
) {
    for resource in resources {
        let (material, y_offset, scale) = match resource.kind {
            ResourceKind::Wood => (tree_material.clone(), 0.65, Vec3::new(0.8, 1.3, 0.8)),
            ResourceKind::Food => (food_material.clone(), 0.25, Vec3::new(0.8, 0.5, 0.8)),
            ResourceKind::Firewood => (tree_material.clone(), 0.25, Vec3::new(0.6, 0.5, 0.6)),
        };
        let position = Vec3::new(
            resource.position.x,
            resource.position.y + y_offset,
            resource.position.z,
        );
        let entity = commands
            .spawn((
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_scale(scale),
                ResourceNode {
                    kind: resource.kind,
                    amount: resource.amount,
                },
            ))
            .id();
        geometry.occupy_polygon(resource_obstacle_polygon(position), entity, false);
    }
}

fn spawn_colonists(
    commands: &mut Commands,
    colonist_mesh: &Handle<Mesh>,
    colonist_material: &Handle<StandardMaterial>,
    seed: u64,
) {
    for (index, (x, z)) in [(-1.2, 1.0), (0.0, 1.2), (1.2, 1.0)]
        .into_iter()
        .enumerate()
    {
        let y = terrain_height(seed, x, z) + 0.32;
        commands.spawn((
            Mesh3d(colonist_mesh.clone()),
            MeshMaterial3d(colonist_material.clone()),
            Transform::from_translation(Vec3::new(x, y, z)),
            Colonist {
                name: format!("定居者 {}", index + 1),
                state: ColonistState::Idle,
                profession: Profession::Unemployed,
                workplace: None,
                speed: 2.2,
                home: None,
                satiety: 100.0,
                carry_capacity: COLONIST_CARRY_CAPACITY,
            },
        ));
    }
}
