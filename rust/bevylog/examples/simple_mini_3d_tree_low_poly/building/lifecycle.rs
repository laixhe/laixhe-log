//! 蓝图生命周期：放置后先等殖民者搬运木材（`Blueprint.delivered_wood`），
//! 材料齐了再由殖民者按时间推进进度；完工时把蓝图替换为正式建筑，
//! 并生成视觉体、入口标记、住房 / 岗位组件（农场则生成覆盖层与作物）。

use bevy::{light::NotShadowCaster, prelude::*};

use crate::{
    farm::{
        CompletedFarmPlot, FarmCrop, FarmPlot, FarmVisual, farm_crop_positions, farm_overlay_mesh,
    },
    resources::{HOUSE_FOOD_CAPACITY, Inventory, PublicInventory, STORAGE_CAPACITY},
    terrain::TerrainSeed,
    types::{BuildingKind, ConstructionKind, ResourceKind},
    world::GameAssets,
};

use super::{
    Blueprint, BuildingEntrance, BuildingVisual, CompletedBuilding, EntranceMarker,
    EntrancePreview, Footprint, Housing, Workplace,
};

pub(super) const ENTRANCE_MARKER_SCALE: Vec3 = Vec3::new(0.42, 0.08, 0.42);

// 蓝图视觉同步：让预览方块的高度跟随施工进度从 35% 长到 100%，
// 直观显示「正在建造」的过程（方块底部始终贴地）。
pub fn update_blueprint_visuals(
    blueprints: Query<&Blueprint>,
    mut visuals: Query<(&BuildingVisual, &mut Transform)>,
) {
    for (visual, mut transform) in &mut visuals {
        let Ok(blueprint) = blueprints.get(visual.owner) else {
            continue;
        };
        let Some(kind) = blueprint.kind.as_building() else {
            continue;
        };
        let height = kind.definition().height;
        let visual_height = (height * (0.35 + blueprint.progress_ratio() * 0.65)).max(0.04);
        transform.scale.y = visual_height;
        transform.translation.y = visual_height * 0.5;
    }
}

// 蓝图完工：把 Blueprint 组件替换为正式建筑，并装上对应的功能组件——
// 房屋 → 家庭存粮 + 住房；仓库 → 公共库存；工坊 → 岗位；农场 → 覆盖层换成土壤 + 种庄稼。
pub fn finish_blueprints(
    mut commands: Commands,
    assets: Option<Res<GameAssets>>,
    terrain_seed: Res<TerrainSeed>,
    blueprint_query: Query<(Entity, &Blueprint, Option<&Footprint>, Option<&FarmPlot>)>,
    mut visuals: Query<
        (&BuildingVisual, &mut MeshMaterial3d<StandardMaterial>),
        Without<FarmVisual>,
    >,
    mut farm_visuals: Query<
        (&FarmVisual, &mut MeshMaterial3d<StandardMaterial>),
        Without<BuildingVisual>,
    >,
) {
    let Some(assets) = assets else {
        return;
    };

    for (entity, blueprint, footprint, farm_plot) in &blueprint_query {
        if !blueprint.is_complete() {
            continue;
        }

        info!(
            "[建造] {} 完工：{entity:?} 从蓝图转为正式建筑",
            blueprint.kind.label()
        );
        commands.entity(entity).remove::<Blueprint>();
        let mut completed_farm_polygon = None;

        match blueprint.kind {
            ConstructionKind::Building(kind) => {
                for (visual, mut material) in &mut visuals {
                    if visual.owner == entity {
                        material.0 = assets.building_material(kind);
                        break;
                    }
                }
                commands.entity(entity).insert(CompletedBuilding { kind });
                match kind {
                    BuildingKind::House => {
                        commands.entity(entity).insert((
                            Inventory::home_food(HOUSE_FOOD_CAPACITY),
                            Housing::default(),
                        ));
                    }
                    BuildingKind::Storage => {
                        let mut inventory = Inventory::public(STORAGE_CAPACITY);
                        inventory.add(ResourceKind::Wood, 4);
                        commands.entity(entity).insert((inventory, PublicInventory));
                    }
                    BuildingKind::Woodcutter
                    | BuildingKind::Gatherer
                    | BuildingKind::ChoppingYard => {
                        if let Some(workplace) = Workplace::for_building(kind) {
                            commands.entity(entity).insert(workplace);
                        }
                    }
                    BuildingKind::Road => {}
                }
            }
            ConstructionKind::Farm => {
                for (visual, mut material) in &mut farm_visuals {
                    if visual.owner == entity {
                        material.0 = assets.farm_soil_material.clone();
                        break;
                    }
                }
                commands.entity(entity).insert(CompletedFarmPlot {
                    area_cells: farm_plot.map(|plot| plot.area_cells).unwrap_or(0.0),
                });
                if let Some(footprint) = footprint {
                    completed_farm_polygon = Some(footprint.polygon.clone());
                }
            }
        }
        if let Some(footprint) = footprint {
            commands.entity(entity).insert(Footprint {
                polygon: footprint.polygon.clone(),
                passable: footprint.passable,
            });
        }
        if let Some(polygon) = completed_farm_polygon {
            spawn_farm_crops(&mut commands, &assets, entity, terrain_seed.0, &polygon);
        }
    }
}

pub fn sync_entrance_markers(
    entrances: Query<&BuildingEntrance>,
    mut markers: Query<(&EntranceMarker, &mut Transform)>,
) {
    for (marker, mut transform) in &mut markers {
        if let Ok(entrance) = entrances.get(marker.owner) {
            transform.translation = entrance_marker_translation(entrance.local_offset);
        }
    }
}

pub(super) fn sync_building_visual(
    commands: &mut Commands,
    assets: &GameAssets,
    owner: Entity,
    material: Handle<StandardMaterial>,
    scale: Vec3,
    height: f32,
    visuals: &mut Query<
        (
            Entity,
            &BuildingVisual,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<FarmVisual>,
    >,
) {
    for (_, visual, mut transform, mut visual_material) in visuals.iter_mut() {
        if visual.owner == owner {
            transform.translation = visual_translation(height);
            transform.scale = scale;
            visual_material.0 = material;
            return;
        }
    }

    spawn_building_visual(commands, assets, owner, material, scale, height);
}

pub(super) fn sync_farm_visual(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    owner: Entity,
    seed: u64,
    polygon: &[Vec2],
    material: Handle<StandardMaterial>,
    visuals: &mut Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    let origin = crate::farm::farm_origin(seed, polygon);
    let mesh = meshes.add(farm_overlay_mesh(seed, polygon, origin));
    for (_, visual, mut mesh_handle, mut visual_material) in visuals.iter_mut() {
        if visual.owner == owner {
            mesh_handle.0 = mesh;
            visual_material.0 = material;
            return;
        }
    }

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::default(),
        Visibility::Visible,
        FarmVisual { owner },
        ChildOf(owner),
        NotShadowCaster,
    ));
}

fn spawn_farm_crops(
    commands: &mut Commands,
    assets: &GameAssets,
    owner: Entity,
    seed: u64,
    polygon: &[Vec2],
) {
    let origin = crate::farm::farm_origin(seed, polygon);
    for translation in farm_crop_positions(seed, polygon, origin) {
        commands.spawn((
            Mesh3d(assets.crop_mesh.clone()),
            MeshMaterial3d(assets.crop_material.clone()),
            Transform::from_translation(translation),
            FarmCrop,
            ChildOf(owner),
            NotShadowCaster,
        ));
    }
}

pub(super) fn despawn_building_visual(
    commands: &mut Commands,
    owner: Entity,
    visuals: &mut Query<
        (
            Entity,
            &BuildingVisual,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<FarmVisual>,
    >,
) {
    for (entity, visual, _, _) in visuals.iter_mut() {
        if visual.owner == owner {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn despawn_farm_visual(
    commands: &mut Commands,
    owner: Entity,
    visuals: &mut Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    for (entity, visual, _, _) in visuals.iter_mut() {
        if visual.owner == owner {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn spawn_building_visual(
    commands: &mut Commands,
    assets: &GameAssets,
    owner: Entity,
    material: Handle<StandardMaterial>,
    scale: Vec3,
    height: f32,
) {
    commands.spawn((
        Mesh3d(assets.cube_mesh.clone()),
        MeshMaterial3d(material),
        Transform::from_translation(visual_translation(height)).with_scale(scale),
        BuildingVisual { owner },
        ChildOf(owner),
    ));
}

pub(super) fn spawn_entrance_marker(
    commands: &mut Commands,
    assets: &GameAssets,
    owner: Entity,
    local_offset: Vec3,
) {
    commands.spawn((
        Mesh3d(assets.cube_mesh.clone()),
        MeshMaterial3d(assets.entrance_material.clone()),
        Transform::from_translation(entrance_marker_translation(local_offset))
            .with_scale(ENTRANCE_MARKER_SCALE),
        EntranceMarker { owner },
        ChildOf(owner),
    ));
}

pub(super) fn spawn_entrance_preview(
    commands: &mut Commands,
    assets: &GameAssets,
    parent: Entity,
    local_offset: Vec3,
) -> Entity {
    commands
        .spawn((
            Mesh3d(assets.cube_mesh.clone()),
            MeshMaterial3d(assets.entrance_material.clone()),
            Transform::from_translation(entrance_marker_translation(local_offset))
                .with_scale(ENTRANCE_MARKER_SCALE),
            EntrancePreview,
            ChildOf(parent),
        ))
        .id()
}

pub(super) fn entrance_marker_translation(local_offset: Vec3) -> Vec3 {
    Vec3::new(local_offset.x, 0.04, local_offset.z)
}

pub(super) fn visual_translation(height: f32) -> Vec3 {
    Vec3::new(0.0, height * 0.5, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::building::components::Profession;
    use crate::terrain::{DEFAULT_TERRAIN_SEED, TerrainSeed};

    fn test_assets() -> GameAssets {
        let mut materials = Assets::<StandardMaterial>::default();
        GameAssets {
            cube_mesh: Handle::default(),
            preview_valid_material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
            preview_invalid_material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            blueprint_material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
            house_material: materials.add(Color::srgb(0.7, 0.3, 0.2)),
            storage_material: materials.add(Color::srgb(0.7, 0.6, 0.3)),
            woodcutter_material: materials.add(Color::srgb(0.3, 0.6, 0.2)),
            gatherer_material: materials.add(Color::srgb(0.5, 0.4, 0.7)),
            chopping_yard_material: materials.add(Color::srgb(0.6, 0.4, 0.2)),
            road_material: materials.add(Color::srgb(0.2, 0.2, 0.2)),
            entrance_material: materials.add(Color::srgb(1.0, 0.9, 0.2)),
            farm_blueprint_material: materials.add(Color::srgba(0.4, 0.25, 0.12, 0.7)),
            farm_soil_material: materials.add(Color::srgba(0.3, 0.2, 0.1, 0.9)),
            crop_mesh: Handle::default(),
            crop_material: materials.add(Color::srgb(0.34, 0.67, 0.24)),
            colonist_mesh: Handle::default(),
            colonist_material: materials.add(Color::srgb(0.9, 0.7, 0.4)),
        }
    }

    #[test]
    fn finish_blueprints_switches_farm_visual_material() {
        let mut app = App::new();
        let assets = test_assets();
        let blueprint_material = assets.farm_blueprint_material.clone();
        let soil_material = assets.farm_soil_material.clone();
        app.insert_resource(assets);
        app.insert_resource(TerrainSeed(DEFAULT_TERRAIN_SEED));
        app.add_systems(Update, finish_blueprints);

        let farm = app
            .world_mut()
            .spawn((
                Blueprint {
                    kind: ConstructionKind::Farm,
                    required_wood: 0,
                    delivered_wood: 0,
                    progress: 1.0,
                    build_seconds: 1.0,
                },
                Footprint {
                    polygon: vec![
                        Vec2::new(0.0, 0.0),
                        Vec2::new(1.0, 0.0),
                        Vec2::new(1.0, 1.0),
                        Vec2::new(0.0, 1.0),
                    ],
                    passable: false,
                },
                FarmPlot { area_cells: 1.0 },
            ))
            .id();
        app.world_mut().spawn((
            FarmVisual { owner: farm },
            MeshMaterial3d(blueprint_material),
        ));

        app.update();

        let farm_entity = app.world().entity(farm);
        assert!(farm_entity.get::<CompletedFarmPlot>().is_some());
        assert!(farm_entity.get::<Blueprint>().is_none());

        let mut query = app
            .world_mut()
            .query::<(&FarmVisual, &MeshMaterial3d<StandardMaterial>)>();
        let material = query
            .iter(app.world())
            .find(|(visual, _)| visual.owner == farm)
            .map(|(_, material)| material.0.clone())
            .unwrap();
        assert_eq!(material, soil_material);
    }

    #[test]
    fn finish_blueprints_spawns_farm_crop_children() {
        let mut app = App::new();
        app.insert_resource(test_assets());
        app.insert_resource(TerrainSeed(DEFAULT_TERRAIN_SEED));
        app.add_systems(Update, finish_blueprints);

        let polygon = vec![
            Vec2::new(20.0, 20.0),
            Vec2::new(23.0, 20.0),
            Vec2::new(23.0, 23.0),
            Vec2::new(20.0, 23.0),
        ];
        let farm = app
            .world_mut()
            .spawn((
                Transform::from_translation(crate::farm::farm_origin(
                    DEFAULT_TERRAIN_SEED,
                    &polygon,
                )),
                Visibility::Visible,
                Blueprint {
                    kind: ConstructionKind::Farm,
                    required_wood: 0,
                    delivered_wood: 0,
                    progress: 1.0,
                    build_seconds: 1.0,
                },
                Footprint {
                    polygon: polygon.clone(),
                    passable: false,
                },
                FarmPlot { area_cells: 9.0 },
            ))
            .id();
        app.world_mut().spawn((
            FarmVisual { owner: farm },
            MeshMaterial3d::<StandardMaterial>(Handle::default()),
        ));

        app.update();

        let mut crops = app.world_mut().query::<(&FarmCrop, &ChildOf)>();
        let crop_count = crops
            .iter(app.world())
            .inspect(|(_, child_of)| {
                assert_eq!(child_of.parent(), farm);
            })
            .count();

        assert!(crop_count > 0);
    }

    #[test]
    fn finish_blueprints_adds_workplace_to_chopping_yard() {
        let mut app = App::new();
        app.insert_resource(test_assets());
        app.insert_resource(TerrainSeed(DEFAULT_TERRAIN_SEED));
        app.add_systems(Update, finish_blueprints);

        let yard = app
            .world_mut()
            .spawn(Blueprint {
                kind: ConstructionKind::Building(BuildingKind::ChoppingYard),
                required_wood: 0,
                delivered_wood: 0,
                progress: 1.0,
                build_seconds: 1.0,
            })
            .id();

        app.update();

        let entity = app.world().entity(yard);
        let workplace = entity.get::<Workplace>().unwrap();
        assert_eq!(workplace.profession, Profession::WoodSplitter);
        assert_eq!(workplace.desired_slots, Workplace::DEFAULT_MAX_SLOTS);
        assert_eq!(workplace.max_slots, Workplace::DEFAULT_MAX_SLOTS);
    }
}
