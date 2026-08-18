//! 点选系统：鼠标左键把屏幕坐标转成地面世界坐标（`math::ray_terrain_intersection`），
//! 再按“距离 + 优先级”命中殖民者 / 蓝图 / 建筑 / 农场 / 资源节点，用 Gizmos 画选中高亮。

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    building::{Blueprint, BuildState, CompletedBuilding, Footprint, point_in_polygon},
    colonist::Colonist,
    farm::CompletedFarmPlot,
    math::{ray_terrain_intersection, terrain_pick_max_distance, xz_distance},
    terrain::{TerrainGenerationConfig, terrain_height},
    types::{BuildingKind, CELL_SIZE, ConstructionKind},
    world::ResourceNode,
};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .add_systems(Update, (select_target, draw_selection_highlight));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectedTarget {
    Blueprint(Entity),
    Building(Entity),
    Farm(Entity),
    Colonist(Entity),
    Resource(Entity),
}

impl SelectedTarget {
    pub fn entity(self) -> Entity {
        match self {
            Self::Blueprint(entity)
            | Self::Building(entity)
            | Self::Farm(entity)
            | Self::Colonist(entity)
            | Self::Resource(entity) => entity,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct SelectionState {
    pub selected: Option<SelectedTarget>,
}

#[derive(Clone, Copy, Debug)]
pub struct HitCandidate {
    pub target: SelectedTarget,
    pub distance: f32,
    pub priority: u8,
}

// 点选主流程：建造模式、按下按钮或按 Esc 时直接跳过；
// 否则把光标转成地面世界坐标，依次收集各类型候选（殖民者/蓝图/建筑/农场/资源），
// 交给 best_hit 决定最终命中谁。
pub fn select_target(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    terrain_config: Res<TerrainGenerationConfig>,
    button_interactions: Query<&Interaction, With<Button>>,
    build_state: Res<BuildState>,
    mut selection: ResMut<SelectionState>,
    resource_nodes: Query<(Entity, &ResourceNode, &Transform)>,
    colonists: Query<(Entity, &Colonist, &Transform)>,
    blueprints: Query<(Entity, &Blueprint, &Transform, Option<&Footprint>)>,
    buildings: Query<(Entity, &CompletedBuilding, &Transform), Without<Blueprint>>,
    farms: Query<(Entity, &CompletedFarmPlot, &Footprint), Without<Blueprint>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        selection.selected = None;
        return;
    }

    if build_state.selected.is_some() || !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if button_interactions
        .iter()
        .any(|interaction| *interaction != Interaction::None)
    {
        return;
    }

    let Some(cursor_world) = cursor_ground_position(&windows, &camera_query, terrain_config.seed)
    else {
        return;
    };

    let mut candidates = Vec::new();
    collect_colonist_hits(cursor_world, &colonists, &mut candidates);
    collect_blueprint_hits(cursor_world, &blueprints, &mut candidates);
    collect_building_hits(cursor_world, &buildings, &mut candidates);
    collect_farm_hits(cursor_world, &farms, &mut candidates);
    collect_resource_hits(cursor_world, &resource_nodes, &mut candidates);

    selection.selected = best_hit(&candidates).map(|candidate| candidate.target);
}

pub fn draw_selection_highlight(
    selection: Res<SelectionState>,
    terrain_config: Res<TerrainGenerationConfig>,
    mut gizmos: Gizmos,
    resource_nodes: Query<(Entity, &Transform), With<ResourceNode>>,
    colonists: Query<(Entity, &Transform), With<Colonist>>,
    blueprints: Query<(Entity, &Blueprint, &Transform, Option<&Footprint>)>,
    buildings: Query<(Entity, &CompletedBuilding, &Transform), Without<Blueprint>>,
    farms: Query<(Entity, &CompletedFarmPlot, &Footprint), Without<Blueprint>>,
) {
    let Some(selected) = selection.selected else {
        return;
    };

    if draw_polygon_selection(
        selected,
        terrain_config.seed,
        &mut gizmos,
        &blueprints,
        &farms,
    ) {
        return;
    }

    let Some((position, radius)) = selected_position_and_radius(
        selected,
        &resource_nodes,
        &colonists,
        &blueprints,
        &buildings,
    ) else {
        return;
    };

    gizmos.circle(
        Isometry3d::new(
            Vec3::new(position.x, position.y + 0.08, position.z),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        ),
        radius,
        LinearRgba::rgb(1.0, 0.88, 0.18),
    );
    gizmos.cube(
        Transform::from_translation(Vec3::new(position.x, position.y + 0.08, position.z))
            .with_scale(Vec3::splat(radius * 1.6)),
        LinearRgba::rgb(1.0, 0.88, 0.18),
    );
}

// 命中规则：priority 越小越优先（殖民者 0 > 蓝图 1 > 建筑/农场 2 > 资源 3），
// 同优先级选离光标更近的；再按实体 id 兜底，保证结果确定。
pub fn best_hit(candidates: &[HitCandidate]) -> Option<HitCandidate> {
    candidates.iter().copied().min_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then_with(|| left.distance.total_cmp(&right.distance))
            .then_with(|| left.target.entity().cmp(&right.target.entity()))
    })
}

fn collect_colonist_hits(
    cursor_world: Vec3,
    colonists: &Query<(Entity, &Colonist, &Transform)>,
    candidates: &mut Vec<HitCandidate>,
) {
    for (entity, _, transform) in colonists {
        let distance = xz_distance(cursor_world, transform.translation);
        if distance <= 0.7 {
            candidates.push(HitCandidate {
                target: SelectedTarget::Colonist(entity),
                distance,
                priority: 0,
            });
        }
    }
}

fn collect_blueprint_hits(
    cursor_world: Vec3,
    blueprints: &Query<(Entity, &Blueprint, &Transform, Option<&Footprint>)>,
    candidates: &mut Vec<HitCandidate>,
) {
    for (entity, blueprint, transform, footprint) in blueprints {
        let hit = match blueprint.kind {
            ConstructionKind::Building(kind) => {
                point_in_building_box(cursor_world, transform, kind, 0.25)
            }
            ConstructionKind::Farm => footprint
                .map(|footprint| point_in_farm_footprint(cursor_world, footprint))
                .unwrap_or(false),
        };
        if hit {
            candidates.push(HitCandidate {
                target: SelectedTarget::Blueprint(entity),
                distance: xz_distance(cursor_world, transform.translation),
                priority: 1,
            });
        }
    }
}

fn collect_building_hits(
    cursor_world: Vec3,
    buildings: &Query<(Entity, &CompletedBuilding, &Transform), Without<Blueprint>>,
    candidates: &mut Vec<HitCandidate>,
) {
    for (entity, building, transform) in buildings {
        if point_in_building_box(cursor_world, transform, building.kind, 0.25) {
            candidates.push(HitCandidate {
                target: SelectedTarget::Building(entity),
                distance: xz_distance(cursor_world, transform.translation),
                priority: 2,
            });
        }
    }
}

fn collect_farm_hits(
    cursor_world: Vec3,
    farms: &Query<(Entity, &CompletedFarmPlot, &Footprint), Without<Blueprint>>,
    candidates: &mut Vec<HitCandidate>,
) {
    for (entity, _, footprint) in farms {
        if point_in_farm_footprint(cursor_world, footprint) {
            candidates.push(HitCandidate {
                target: SelectedTarget::Farm(entity),
                distance: distance_to_polygon_center(cursor_world, &footprint.polygon),
                priority: 2,
            });
        }
    }
}

fn collect_resource_hits(
    cursor_world: Vec3,
    resource_nodes: &Query<(Entity, &ResourceNode, &Transform)>,
    candidates: &mut Vec<HitCandidate>,
) {
    for (entity, _, transform) in resource_nodes {
        let distance = xz_distance(cursor_world, transform.translation);
        if distance <= transform.scale.x.max(transform.scale.z) * 0.75 + 0.3 {
            candidates.push(HitCandidate {
                target: SelectedTarget::Resource(entity),
                distance,
                priority: 3,
            });
        }
    }
}

fn selected_position_and_radius(
    selected: SelectedTarget,
    resource_nodes: &Query<(Entity, &Transform), With<ResourceNode>>,
    colonists: &Query<(Entity, &Transform), With<Colonist>>,
    blueprints: &Query<(Entity, &Blueprint, &Transform, Option<&Footprint>)>,
    buildings: &Query<(Entity, &CompletedBuilding, &Transform), Without<Blueprint>>,
) -> Option<(Vec3, f32)> {
    match selected {
        SelectedTarget::Resource(entity) => resource_nodes
            .get(entity)
            .ok()
            .map(|(_, transform)| (transform.translation, 0.75)),
        SelectedTarget::Colonist(entity) => colonists
            .get(entity)
            .ok()
            .map(|(_, transform)| (transform.translation, 0.45)),
        SelectedTarget::Blueprint(entity) => {
            blueprints
                .get(entity)
                .ok()
                .and_then(|(_, blueprint, transform, _)| {
                    let kind = blueprint.kind.as_building()?;
                    let size = building_visual_size(kind);
                    Some((transform.translation, size.x.max(size.y) * 0.65))
                })
        }
        SelectedTarget::Building(entity) => {
            buildings.get(entity).ok().map(|(_, building, transform)| {
                let size = building_visual_size(building.kind);
                (transform.translation, size.x.max(size.y) * 0.65)
            })
        }
        SelectedTarget::Farm(_) => None,
    }
}

fn draw_polygon_selection(
    selected: SelectedTarget,
    seed: u64,
    gizmos: &mut Gizmos,
    blueprints: &Query<(Entity, &Blueprint, &Transform, Option<&Footprint>)>,
    farms: &Query<(Entity, &CompletedFarmPlot, &Footprint), Without<Blueprint>>,
) -> bool {
    match selected {
        SelectedTarget::Blueprint(entity) => {
            let Ok((_, blueprint, _, Some(footprint))) = blueprints.get(entity) else {
                return false;
            };
            if blueprint.kind != ConstructionKind::Farm {
                return false;
            }
            draw_polygon_outline(gizmos, seed, &footprint.polygon);
            true
        }
        SelectedTarget::Farm(entity) => {
            let Ok((_, _, footprint)) = farms.get(entity) else {
                return false;
            };
            draw_polygon_outline(gizmos, seed, &footprint.polygon);
            true
        }
        SelectedTarget::Building(_) | SelectedTarget::Colonist(_) | SelectedTarget::Resource(_) => {
            false
        }
    }
}

fn draw_polygon_outline(gizmos: &mut Gizmos, seed: u64, polygon: &[Vec2]) {
    if polygon.len() < 2 {
        return;
    }

    let color = LinearRgba::rgb(1.0, 0.88, 0.18);
    for index in 0..polygon.len() {
        let start = polygon[index];
        let end = polygon[(index + 1) % polygon.len()];
        gizmos.line(
            Vec3::new(
                start.x,
                terrain_height(seed, start.x, start.y) + 0.1,
                start.y,
            ),
            Vec3::new(end.x, terrain_height(seed, end.x, end.y) + 0.1, end.y),
            color,
        );
    }
}

fn cursor_ground_position(
    windows: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    seed: u64,
) -> Option<Vec3> {
    let window = windows.single().ok()?;
    let cursor_position = window.cursor_position()?;
    let (camera, camera_transform) = camera_query.single().ok()?;
    let ray = camera
        .viewport_to_world(camera_transform, cursor_position)
        .ok()?;

    ray_terrain_intersection(ray, seed, terrain_pick_max_distance())
}

fn point_in_building_box(
    point: Vec3,
    transform: &Transform,
    kind: BuildingKind,
    padding: f32,
) -> bool {
    point_in_rotated_box(point, transform, building_visual_size(kind), padding)
}

fn point_in_rotated_box(point: Vec3, transform: &Transform, size: Vec2, padding: f32) -> bool {
    let offset = Vec3::new(
        point.x - transform.translation.x,
        0.0,
        point.z - transform.translation.z,
    );
    let local = transform.rotation.inverse() * offset;
    let half_x = size.x.abs() * 0.5 + padding;
    let half_z = size.y.abs() * 0.5 + padding;

    local.x.abs() <= half_x && local.z.abs() <= half_z
}

fn point_in_farm_footprint(point: Vec3, footprint: &Footprint) -> bool {
    point_in_polygon(Vec2::new(point.x, point.z), &footprint.polygon)
}

fn distance_to_polygon_center(point: Vec3, polygon: &[Vec2]) -> f32 {
    if polygon.is_empty() {
        return 0.0;
    }
    let center = polygon
        .iter()
        .copied()
        .fold(Vec2::ZERO, |sum, point| sum + point)
        / polygon.len() as f32;
    Vec2::new(point.x, point.z).distance(center)
}

fn building_visual_size(kind: BuildingKind) -> Vec2 {
    let definition = kind.definition();

    if kind == BuildingKind::Road {
        Vec2::splat(CELL_SIZE * 0.95)
    } else {
        Vec2::new(
            definition.size.x as f32 * CELL_SIZE * 0.9,
            definition.size.y as f32 * CELL_SIZE * 0.9,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_entity(index: u32) -> Entity {
        Entity::from_raw_u32(index).unwrap()
    }

    #[test]
    fn best_hit_prefers_priority_before_distance() {
        let close_resource = HitCandidate {
            target: SelectedTarget::Resource(test_entity(1)),
            distance: 0.1,
            priority: 3,
        };
        let farther_colonist = HitCandidate {
            target: SelectedTarget::Colonist(test_entity(2)),
            distance: 0.6,
            priority: 0,
        };

        assert_eq!(
            best_hit(&[close_resource, farther_colonist])
                .unwrap()
                .target,
            SelectedTarget::Colonist(test_entity(2))
        );
    }

    #[test]
    fn best_hit_uses_distance_inside_same_priority() {
        let farther = HitCandidate {
            target: SelectedTarget::Building(test_entity(1)),
            distance: 0.8,
            priority: 2,
        };
        let closer = HitCandidate {
            target: SelectedTarget::Building(test_entity(2)),
            distance: 0.2,
            priority: 2,
        };

        assert_eq!(
            best_hit(&[farther, closer]).unwrap().target,
            SelectedTarget::Building(test_entity(2))
        );
    }

    #[test]
    fn building_hit_uses_root_rotation_instead_of_scale() {
        let transform = Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            scale: Vec3::ONE,
        };

        assert!(point_in_building_box(
            Vec3::new(0.0, 0.0, -1.2),
            &transform,
            BuildingKind::Storage,
            0.0
        ));
        assert!(!point_in_building_box(
            Vec3::new(1.2, 0.0, 0.0),
            &transform,
            BuildingKind::Storage,
            0.0
        ));
    }

    #[test]
    fn farm_hit_uses_polygon_footprint() {
        let footprint = Footprint {
            polygon: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(2.0, 0.0),
                Vec2::new(2.0, 2.0),
                Vec2::new(0.0, 2.0),
            ],
            passable: false,
        };

        assert!(point_in_farm_footprint(
            Vec3::new(1.0, 0.0, 1.0),
            &footprint
        ));
        assert!(!point_in_farm_footprint(
            Vec3::new(3.0, 0.0, 1.0),
            &footprint
        ));
    }
}
