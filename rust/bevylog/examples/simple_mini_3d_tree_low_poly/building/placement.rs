//! 建造预览与放置：每帧把光标位置换算成占地多边形（吸附网格 / 自由 / 旋转），
//! 用 `WorldGeometry` 校验合法性（越界 / 占用 / 入口 / 坡度），绿色（合法）或红色（非法）高亮，
//! 左键放置蓝图；农场则进入「逐角点绘制 → 校验凸多边形 → 放置」流程。

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    farm::{
        FARM_CLOSE_DISTANCE, FarmPlot, FarmVisual, farm_access_point, farm_area_cells,
        farm_build_seconds, farm_origin, validate_farm_plan,
    },
    math::{ray_terrain_intersection, terrain_pick_max_distance},
    terrain::{TerrainGenerationConfig, terrain_height},
    types::{
        BuildingKind, CELL_SIZE, ConstructionKind, entrance_local_offset, entrance_world_position,
        snap_to_grid,
    },
    world::GameAssets,
};

use super::lifecycle::{
    despawn_building_visual, despawn_farm_visual, spawn_building_visual, spawn_entrance_marker,
    spawn_entrance_preview, sync_building_visual, sync_farm_visual,
};
use super::polygon::{FOOTPRINT_SCALE, ROAD_FOOTPRINT_SCALE};
use super::{
    Blueprint, BuildPreview, BuildState, BuildingEntrance, BuildingVisual, Footprint,
    PlacementIssue, WorldGeometry, footprint_polygon,
};

pub fn update_build_preview(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    assets: Option<Res<GameAssets>>,
    mut meshes: ResMut<Assets<Mesh>>,
    geometry: Res<WorldGeometry>,
    terrain_config: Res<TerrainGenerationConfig>,
    mut build_state: ResMut<BuildState>,
    mut gizmos: Gizmos,
    mut preview_query: Query<
        (Entity, &mut Transform, &mut Visibility),
        (With<BuildPreview>, Without<BuildingVisual>),
    >,
    mut visual_query: Query<
        (
            Entity,
            &BuildingVisual,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<FarmVisual>,
    >,
    mut farm_visual_query: Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    let Some(assets) = assets else {
        return;
    };

    let Some(construction) = build_state.selected else {
        hide_preview(
            &mut commands,
            &mut build_state,
            &mut preview_query,
            &mut visual_query,
            &mut farm_visual_query,
        );
        hide_entrance_preview(&mut commands, &mut build_state);
        return;
    };

    let Some(cursor_world) = cursor_ground_position(&windows, &camera_query, terrain_config.seed)
    else {
        hide_preview(
            &mut commands,
            &mut build_state,
            &mut preview_query,
            &mut visual_query,
            &mut farm_visual_query,
        );
        hide_entrance_preview(&mut commands, &mut build_state);
        return;
    };

    if construction == ConstructionKind::Farm {
        update_farm_preview(
            &mut commands,
            &assets,
            &mut meshes,
            &geometry,
            terrain_config.seed,
            terrain_config.max_buildable_slope,
            cursor_world,
            &mut build_state,
            &mut gizmos,
            &mut preview_query,
            &mut visual_query,
            &mut farm_visual_query,
        );
        return;
    }

    let Some(kind) = construction.as_building() else {
        return;
    };
    let definition = kind.definition();
    let position = if build_state.snap_to_grid {
        snap_to_grid(cursor_world)
    } else {
        cursor_world
    };
    let snapped_position = if build_state.snap_to_grid {
        snap_to_grid(position)
    } else {
        position
    };

    let terrain_y = terrain_height(terrain_config.seed, snapped_position.x, snapped_position.z);
    let building_position = Vec3::new(snapped_position.x, terrain_y, snapped_position.z);

    let polygon = footprint_polygon(
        kind,
        building_position,
        definition.size,
        build_state.rotation_angle,
    );
    let entrance = planned_entrance(
        kind,
        building_position,
        definition.size,
        build_state.rotation_angle,
    );
    let placement_issue = geometry.placement_issue_for_polygon(
        &polygon,
        entrance.map(|entrance| entrance.world_position),
        kind != BuildingKind::Road,
        terrain_config.seed,
        terrain_config.max_buildable_slope,
    );
    let valid = placement_issue.is_none();
    let invalid_reason = if valid { None } else { placement_issue };

    build_state.last_valid = valid;
    build_state.invalid_reason = invalid_reason;
    build_state.last_position = building_position;
    build_state.last_polygon = polygon;
    let reason_label = invalid_reason
        .map(PlacementIssue::label)
        .unwrap_or("未知原因");
    build_state.status = if valid {
        format!(
            "{}蓝图就绪。造价：{} 木材。",
            definition.label, definition.wood_cost
        )
    } else {
        format!("无法放置{}：{}。", definition.label, reason_label)
    };

    let scale = preview_scale(kind, definition.size, definition.height);
    let rotation = Quat::from_rotation_y(build_state.rotation_angle);
    let material = if valid {
        assets.preview_valid_material.clone()
    } else {
        assets.preview_invalid_material.clone()
    };

    let mut active_preview_entity = None;
    if let Some(entity) = build_state.preview_entity {
        if let Ok((_, mut transform, mut visibility)) = preview_query.get_mut(entity) {
            transform.translation = build_state.last_position;
            transform.rotation = rotation;
            transform.scale = Vec3::ONE;
            *visibility = Visibility::Visible;
            active_preview_entity = Some(entity);
        }
    }

    let preview_entity = if let Some(entity) = active_preview_entity {
        entity
    } else {
        let entity = commands
            .spawn((
                Transform {
                    translation: build_state.last_position,
                    rotation,
                    scale: Vec3::ONE,
                },
                Visibility::Visible,
                BuildPreview,
            ))
            .id();
        build_state.preview_entity = Some(entity);
        entity
    };

    despawn_farm_visual(&mut commands, preview_entity, &mut farm_visual_query);
    sync_building_visual(
        &mut commands,
        &assets,
        preview_entity,
        material,
        scale,
        definition.height,
        &mut visual_query,
    );

    hide_entrance_preview(&mut commands, &mut build_state);
    if valid && kind != BuildingKind::Road {
        if let Some(entrance) = entrance {
            let ent_entity = spawn_entrance_preview(
                &mut commands,
                &assets,
                preview_entity,
                entrance.local_offset,
            );
            build_state.preview_entrance_entity = Some(ent_entity);
        }
    }
}

pub fn place_blueprint(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    button_interactions: Query<&Interaction, With<Button>>,
    assets: Option<Res<GameAssets>>,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    terrain_config: Res<TerrainGenerationConfig>,
    mut geometry: ResMut<WorldGeometry>,
    mut build_state: ResMut<BuildState>,
    mut farm_visual_query: Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    let Some(assets) = assets else {
        return;
    };
    let Some(construction) = build_state.selected else {
        return;
    };

    if construction == ConstructionKind::Farm {
        handle_farm_clicks(
            &mut commands,
            &keyboard,
            &mouse_buttons,
            &button_interactions,
            &assets,
            &mut meshes,
            &windows,
            &camera_query,
            terrain_config.seed,
            terrain_config.max_buildable_slope,
            &mut geometry,
            &mut build_state,
            &mut farm_visual_query,
        );
        return;
    }

    if !mouse_buttons.just_pressed(MouseButton::Left) || !build_state.last_valid {
        return;
    }
    if button_interactions
        .iter()
        .any(|interaction| *interaction != Interaction::None)
    {
        return;
    }

    let Some(kind) = construction.as_building() else {
        return;
    };
    let definition = kind.definition();
    let scale = preview_scale(kind, definition.size, definition.height);
    let rotation = Quat::from_rotation_y(build_state.rotation_angle);
    let passable = kind == BuildingKind::Road;
    let polygon = build_state.last_polygon.clone();
    let entrance = planned_entrance(
        kind,
        build_state.last_position,
        definition.size,
        build_state.rotation_angle,
    );
    if let Some(issue) = geometry.placement_issue_for_polygon(
        &polygon,
        entrance.map(|entrance| entrance.world_position),
        kind != BuildingKind::Road,
        terrain_config.seed,
        terrain_config.max_buildable_slope,
    ) {
        build_state.status = format!("无法放置{}：{}。", definition.label, issue.label());
        build_state.last_valid = false;
        build_state.invalid_reason = Some(issue);
        return;
    }

    let entity = commands
        .spawn((
            Transform {
                translation: build_state.last_position,
                rotation,
                scale: Vec3::ONE,
            },
            Visibility::Visible,
            Blueprint {
                kind: ConstructionKind::Building(kind),
                required_wood: definition.wood_cost,
                delivered_wood: 0,
                progress: 0.0,
                build_seconds: definition.build_seconds,
            },
            Footprint {
                polygon: polygon.clone(),
                passable,
            },
        ))
        .id();

    spawn_building_visual(
        &mut commands,
        &assets,
        entity,
        assets.blueprint_material.clone(),
        scale,
        definition.height,
    );

    geometry.occupy_polygon(polygon, entity, passable);
    if let Some(entrance) = entrance {
        commands.entity(entity).insert(BuildingEntrance {
            world_position: entrance.world_position,
            local_offset: entrance.local_offset,
        });
        geometry.reserve_entrance_point(entrance.world_position, entity);
        spawn_entrance_marker(&mut commands, &assets, entity, entrance.local_offset);
    }
    build_state.status = format!("已放置{}蓝图。", definition.label);
    build_state.last_valid = false;
    build_state.invalid_reason = None;
}

fn update_farm_preview(
    commands: &mut Commands,
    assets: &GameAssets,
    meshes: &mut Assets<Mesh>,
    geometry: &WorldGeometry,
    seed: u64,
    max_slope: f32,
    cursor_world: Vec3,
    build_state: &mut BuildState,
    gizmos: &mut Gizmos,
    preview_query: &mut Query<
        (Entity, &mut Transform, &mut Visibility),
        (With<BuildPreview>, Without<BuildingVisual>),
    >,
    building_visuals: &mut Query<
        (
            Entity,
            &BuildingVisual,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<FarmVisual>,
    >,
    farm_visuals: &mut Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    hide_entrance_preview(commands, build_state);

    let cursor_point = planned_farm_point(cursor_world, build_state.snap_to_grid);
    draw_farm_draft_gizmos(gizmos, seed, &build_state.farm_points, Some(cursor_point));

    let committed_access = farm_access_point(seed, &build_state.farm_points);
    let committed_issue = validate_farm_plan(
        geometry,
        &build_state.farm_points,
        committed_access,
        seed,
        max_slope,
    );
    build_state.last_valid = build_state.farm_points.len() >= 3 && committed_issue.is_none();
    build_state.invalid_reason = committed_issue;
    build_state.last_polygon = build_state.farm_points.clone();
    build_state.last_access_point = committed_access;

    let mut draft_polygon = build_state.farm_points.clone();
    if draft_polygon.len() < 3 || !cursor_closes_farm(&draft_polygon, cursor_point) {
        if draft_polygon
            .last()
            .map(|point| point.distance(cursor_point) > 0.05)
            .unwrap_or(true)
        {
            draft_polygon.push(cursor_point);
        }
    }

    build_state.status = farm_status(build_state, committed_issue);
    if draft_polygon.len() < 3 {
        hide_preview(
            commands,
            build_state,
            preview_query,
            building_visuals,
            farm_visuals,
        );
        return;
    }

    let draft_access = farm_access_point(seed, &draft_polygon);
    let draft_valid =
        validate_farm_plan(geometry, &draft_polygon, draft_access, seed, max_slope).is_none();
    let origin = farm_origin(seed, &draft_polygon);
    build_state.last_position = origin;
    let material = if draft_valid {
        assets.preview_valid_material.clone()
    } else {
        assets.preview_invalid_material.clone()
    };

    let preview_entity = ensure_preview_entity(commands, build_state, preview_query, origin);
    despawn_building_visual(commands, preview_entity, building_visuals);
    sync_farm_visual(
        commands,
        meshes,
        preview_entity,
        seed,
        &draft_polygon,
        material,
        farm_visuals,
    );
}

fn handle_farm_clicks(
    commands: &mut Commands,
    keyboard: &ButtonInput<KeyCode>,
    mouse_buttons: &ButtonInput<MouseButton>,
    button_interactions: &Query<&Interaction, With<Button>>,
    assets: &GameAssets,
    meshes: &mut Assets<Mesh>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    seed: u64,
    max_slope: f32,
    geometry: &mut WorldGeometry,
    build_state: &mut BuildState,
    farm_visuals: &mut Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    let finish_key =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter);
    let left_click = mouse_buttons.just_pressed(MouseButton::Left);
    if !left_click && !finish_key {
        return;
    }
    if left_click
        && button_interactions
            .iter()
            .any(|interaction| *interaction != Interaction::None)
    {
        return;
    }

    if left_click {
        let Some(cursor_world) = cursor_ground_position(windows, camera_query, seed) else {
            return;
        };
        let point = planned_farm_point(cursor_world, build_state.snap_to_grid);
        if cursor_closes_farm(&build_state.farm_points, point) {
            place_farm_blueprint(
                commands,
                assets,
                meshes,
                seed,
                max_slope,
                geometry,
                build_state,
                farm_visuals,
            );
            return;
        }

        if build_state
            .farm_points
            .iter()
            .any(|existing| existing.distance(point) <= 0.05)
        {
            build_state.status = "该角点已存在。".to_string();
            return;
        }

        build_state.farm_points.push(point);
        build_state.last_valid = false;
        build_state.invalid_reason = None;
        build_state.status = format!(
            "农场现有 {} 个角点。继续添加角点，或按回车结束。",
            build_state.farm_points.len()
        );
        return;
    }

    if finish_key {
        place_farm_blueprint(
            commands,
            assets,
            meshes,
            seed,
            max_slope,
            geometry,
            build_state,
            farm_visuals,
        );
    }
}

fn place_farm_blueprint(
    commands: &mut Commands,
    assets: &GameAssets,
    meshes: &mut Assets<Mesh>,
    seed: u64,
    max_slope: f32,
    geometry: &mut WorldGeometry,
    build_state: &mut BuildState,
    farm_visuals: &mut Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    let polygon = build_state.farm_points.clone();
    let access = farm_access_point(seed, &polygon);
    if let Some(issue) = validate_farm_plan(geometry, &polygon, access, seed, max_slope) {
        build_state.status = format!("无法放置农场：{}。", issue.label());
        build_state.last_valid = false;
        build_state.invalid_reason = Some(issue);
        return;
    }

    let access = access.expect("validated farm plan should have an access point");
    let origin = farm_origin(seed, &polygon);
    let area_cells = farm_area_cells(&polygon);
    let build_seconds = farm_build_seconds(&polygon);
    let entity = commands
        .spawn((
            Transform {
                translation: origin,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            Visibility::Visible,
            Blueprint {
                kind: ConstructionKind::Farm,
                required_wood: 0,
                delivered_wood: 0,
                progress: 0.0,
                build_seconds,
            },
            Footprint {
                polygon: polygon.clone(),
                passable: false,
            },
            FarmPlot { area_cells },
            BuildingEntrance {
                world_position: access,
                local_offset: access - origin,
            },
        ))
        .id();

    sync_farm_visual(
        commands,
        meshes,
        entity,
        seed,
        &polygon,
        assets.farm_blueprint_material.clone(),
        farm_visuals,
    );

    geometry.occupy_polygon(polygon, entity, false);
    geometry.reserve_entrance_point(access, entity);
    build_state.farm_points.clear();
    build_state.last_valid = false;
    build_state.invalid_reason = None;
    build_state.last_polygon.clear();
    build_state.last_access_point = None;
    build_state.last_position = origin;
    build_state.status = format!("已放置农场蓝图。面积：{:.1} 格。", area_cells);
}

fn ensure_preview_entity(
    commands: &mut Commands,
    build_state: &mut BuildState,
    preview_query: &mut Query<
        (Entity, &mut Transform, &mut Visibility),
        (With<BuildPreview>, Without<BuildingVisual>),
    >,
    position: Vec3,
) -> Entity {
    if let Some(entity) = build_state.preview_entity {
        if let Ok((_, mut transform, mut visibility)) = preview_query.get_mut(entity) {
            transform.translation = position;
            transform.rotation = Quat::IDENTITY;
            transform.scale = Vec3::ONE;
            *visibility = Visibility::Visible;
            return entity;
        }
    }

    let entity = commands
        .spawn((
            Transform {
                translation: position,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            Visibility::Visible,
            BuildPreview,
        ))
        .id();
    build_state.preview_entity = Some(entity);
    entity
}

fn planned_farm_point(cursor_world: Vec3, snap: bool) -> Vec2 {
    let position = if snap {
        snap_to_grid(cursor_world)
    } else {
        cursor_world
    };
    Vec2::new(position.x, position.z)
}

fn cursor_closes_farm(points: &[Vec2], point: Vec2) -> bool {
    points.len() >= 3
        && points
            .first()
            .map(|first| first.distance(point) <= FARM_CLOSE_DISTANCE)
            .unwrap_or(false)
}

fn farm_status(build_state: &BuildState, issue: Option<PlacementIssue>) -> String {
    match build_state.farm_points.len() {
        0 => "正在规划农场。点击放置第一个角点。".to_string(),
        1 => "正在规划农场。还需要至少两个角点。".to_string(),
        2 => "正在规划农场。还需要至少一个角点。".to_string(),
        _ if issue.is_none() => "农场轮廓就绪。点击第一个角点或按回车放置蓝图。".to_string(),
        _ => format!(
            "无法闭合农场：{}。",
            issue.map(PlacementIssue::label).unwrap_or("轮廓无效")
        ),
    }
}

fn draw_farm_draft_gizmos(
    gizmos: &mut Gizmos,
    seed: u64,
    points: &[Vec2],
    cursor_point: Option<Vec2>,
) {
    let color = LinearRgba::rgb(0.95, 0.78, 0.28);
    for (index, point) in points.iter().enumerate() {
        let position = farm_gizmo_position(seed, *point);
        gizmos.circle(
            Isometry3d::new(position, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            if index == 0 { 0.18 } else { 0.12 },
            color,
        );
        if let Some(next) = points.get(index + 1) {
            gizmos.line(position, farm_gizmo_position(seed, *next), color);
        }
    }

    if let (Some(last), Some(cursor)) = (points.last(), cursor_point) {
        gizmos.line(
            farm_gizmo_position(seed, *last),
            farm_gizmo_position(seed, cursor),
            color,
        );
    }
    if points.len() >= 3 {
        if let (Some(first), Some(cursor)) = (points.first(), cursor_point) {
            if cursor_closes_farm(points, cursor) {
                gizmos.line(
                    farm_gizmo_position(seed, cursor),
                    farm_gizmo_position(seed, *first),
                    color,
                );
            }
        }
    }
}

fn farm_gizmo_position(seed: u64, point: Vec2) -> Vec3 {
    Vec3::new(
        point.x,
        terrain_height(seed, point.x, point.y) + 0.12,
        point.y,
    )
}

fn hide_preview(
    commands: &mut Commands,
    build_state: &mut BuildState,
    preview_query: &mut Query<
        (Entity, &mut Transform, &mut Visibility),
        (With<BuildPreview>, Without<BuildingVisual>),
    >,
    building_visuals: &mut Query<
        (
            Entity,
            &BuildingVisual,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<FarmVisual>,
    >,
    farm_visuals: &mut Query<
        (
            Entity,
            &FarmVisual,
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<BuildingVisual>,
    >,
) {
    build_state.invalid_reason = None;
    build_state.last_valid = false;
    build_state.last_access_point = None;
    if let Some(entity) = build_state.preview_entity {
        despawn_building_visual(commands, entity, building_visuals);
        despawn_farm_visual(commands, entity, farm_visuals);
        if let Ok((_, _, mut visibility)) = preview_query.get_mut(entity) {
            *visibility = Visibility::Hidden;
        }
    }
}

fn hide_entrance_preview(commands: &mut Commands, build_state: &mut BuildState) {
    if let Some(entity) = build_state.preview_entrance_entity.take() {
        commands.entity(entity).despawn();
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

fn preview_scale(kind: BuildingKind, size: IVec2, height: f32) -> Vec3 {
    if kind == BuildingKind::Road {
        Vec3::new(
            CELL_SIZE * ROAD_FOOTPRINT_SCALE,
            height,
            CELL_SIZE * ROAD_FOOTPRINT_SCALE,
        )
    } else {
        Vec3::new(
            size.x as f32 * CELL_SIZE * FOOTPRINT_SCALE,
            height,
            size.y as f32 * CELL_SIZE * FOOTPRINT_SCALE,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PlannedEntrance {
    pub world_position: Vec3,
    pub local_offset: Vec3,
}

pub(super) fn planned_entrance(
    kind: BuildingKind,
    building_center: Vec3,
    size: IVec2,
    rotation_angle: f32,
) -> Option<PlannedEntrance> {
    kind.entrance_direction().map(|direction| {
        let local_offset = entrance_local_offset(size, direction);

        PlannedEntrance {
            world_position: entrance_world_position(
                building_center,
                size,
                rotation_angle,
                direction,
            ),
            local_offset,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::xz;
    use crate::types::BuildingKind;

    fn spawn_preview_with_farm_visual(app: &mut App) -> (Entity, Entity) {
        let preview = app
            .world_mut()
            .spawn((BuildPreview, Transform::default(), Visibility::Visible))
            .id();
        let farm_visual = app
            .world_mut()
            .spawn((
                FarmVisual { owner: preview },
                Mesh3d(Handle::default()),
                MeshMaterial3d::<StandardMaterial>(Handle::default()),
                ChildOf(preview),
            ))
            .id();
        app.world_mut().resource_mut::<BuildState>().preview_entity = Some(preview);
        (preview, farm_visual)
    }

    fn cleanup_preview_system(
        mut commands: Commands,
        mut build_state: ResMut<BuildState>,
        mut preview_query: Query<
            (Entity, &mut Transform, &mut Visibility),
            (With<BuildPreview>, Without<BuildingVisual>),
        >,
        mut building_visuals: Query<
            (
                Entity,
                &BuildingVisual,
                &mut Transform,
                &mut MeshMaterial3d<StandardMaterial>,
            ),
            Without<FarmVisual>,
        >,
        mut farm_visuals: Query<
            (
                Entity,
                &FarmVisual,
                &mut Mesh3d,
                &mut MeshMaterial3d<StandardMaterial>,
            ),
            Without<BuildingVisual>,
        >,
    ) {
        hide_preview(
            &mut commands,
            &mut build_state,
            &mut preview_query,
            &mut building_visuals,
            &mut farm_visuals,
        );
    }

    fn cleanup_cancelled_preview_system(
        mut commands: Commands,
        mut build_state: ResMut<BuildState>,
        mut preview_query: Query<
            (Entity, &mut Transform, &mut Visibility),
            (With<BuildPreview>, Without<BuildingVisual>),
        >,
        mut building_visuals: Query<
            (
                Entity,
                &BuildingVisual,
                &mut Transform,
                &mut MeshMaterial3d<StandardMaterial>,
            ),
            Without<FarmVisual>,
        >,
        mut farm_visuals: Query<
            (
                Entity,
                &FarmVisual,
                &mut Mesh3d,
                &mut MeshMaterial3d<StandardMaterial>,
            ),
            Without<BuildingVisual>,
        >,
    ) {
        if build_state.selected.is_none() {
            hide_preview(
                &mut commands,
                &mut build_state,
                &mut preview_query,
                &mut building_visuals,
                &mut farm_visuals,
            );
        }
    }

    #[test]
    fn invalid_farm_preview_cleanup_despawns_farm_visual() {
        let mut app = App::new();
        app.init_resource::<BuildState>();
        let (preview, farm_visual) = spawn_preview_with_farm_visual(&mut app);
        app.add_systems(Update, cleanup_preview_system);

        app.update();

        let mut farm_visuals = app.world_mut().query::<&FarmVisual>();
        assert_eq!(farm_visuals.iter(app.world()).count(), 0);
        assert!(app.world().get_entity(farm_visual).is_err());
        assert_eq!(
            *app.world().entity(preview).get::<Visibility>().unwrap(),
            Visibility::Hidden
        );
    }

    #[test]
    fn cancelled_build_state_cleanup_despawns_farm_visual() {
        let mut app = App::new();
        app.init_resource::<BuildState>();
        let (_, farm_visual) = spawn_preview_with_farm_visual(&mut app);
        app.world_mut()
            .resource_mut::<BuildState>()
            .select_construction(ConstructionKind::Farm);
        app.world_mut().resource_mut::<BuildState>().cancel();
        app.add_systems(Update, cleanup_cancelled_preview_system);

        app.update();

        let mut farm_visuals = app.world_mut().query::<&FarmVisual>();
        assert_eq!(farm_visuals.iter(app.world()).count(), 0);
        assert!(app.world().get_entity(farm_visual).is_err());
    }

    #[test]
    fn planned_entrance_uses_continuous_world_position() {
        let building_center = Vec3::new(1.37, 0.0, -2.22);
        let rotation_angle = 0.37;
        let definition = BuildingKind::Storage.definition();

        let entrance = planned_entrance(
            BuildingKind::Storage,
            building_center,
            definition.size,
            rotation_angle,
        )
        .unwrap();
        let visual_world_position = entrance_world_position(
            building_center,
            definition.size,
            rotation_angle,
            BuildingKind::Storage.entrance_direction().unwrap(),
        );

        assert_vec3_approx_eq(entrance.world_position, visual_world_position);
        assert_eq!(
            entrance.local_offset,
            entrance_local_offset(
                definition.size,
                BuildingKind::Storage.entrance_direction().unwrap()
            )
        );
    }

    #[test]
    fn planned_entrance_point_sits_outside_rotated_footprint() {
        let building_center = Vec3::new(1.37, 0.0, -2.22);
        let rotation_angle = 0.37;
        let definition = BuildingKind::Storage.definition();
        let polygon = footprint_polygon(
            BuildingKind::Storage,
            building_center,
            definition.size,
            rotation_angle,
        );

        let entrance = planned_entrance(
            BuildingKind::Storage,
            building_center,
            definition.size,
            rotation_angle,
        )
        .unwrap();

        assert!(!super::super::point_in_polygon(
            xz(entrance.world_position),
            &polygon
        ));
    }

    #[test]
    fn occupied_polygon_blocks_los_but_continuous_entrance_remains_reachable() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut geometry = WorldGeometry::default();
        let definition = BuildingKind::Storage.definition();
        let center = Vec3::new(1.37, 0.0, -2.22);
        let rotation = 0.37;
        let polygon = footprint_polygon(BuildingKind::Storage, center, definition.size, rotation);
        let entrance =
            planned_entrance(BuildingKind::Storage, center, definition.size, rotation).unwrap();

        geometry.occupy_polygon(polygon, entity, false);
        geometry.reserve_entrance_point(entrance.world_position, entity);

        assert!(geometry.is_walkable_point(entrance.world_position));
        assert!(
            crate::navigation::path_to_waypoints(
                &geometry,
                &mut crate::navigation::PathCache::default(),
                Vec3::new(-1.2, 0.0, 1.0),
                entrance.world_position,
                crate::terrain::DEFAULT_TERRAIN_SEED,
            )
            .is_some()
        );
    }

    fn assert_vec3_approx_eq(actual: Vec3, expected: Vec3) {
        let delta = actual - expected;
        assert!(
            delta.length() < 0.0001,
            "expected {expected:?}, got {actual:?}"
        );
    }
}
