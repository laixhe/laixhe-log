//! 建造输入：数字键 `1`-`7` 选建筑、`R` 旋转（网格模式短按步进 90°，
//! 自由模式长按连续旋转）、`G` 网格吸附开关、`Esc` 或右键取消、农场右键撤销角点。

use bevy::prelude::*;

use crate::resources::CentralStorage;
use crate::selection::{SelectedTarget, SelectionState};
use crate::types::{CONSTRUCTION_KINDS, ConstructionKind};

use super::{Blueprint, BuildState, CompletedBuilding, WorldGeometry};

pub fn handle_build_hotkeys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut build_state: ResMut<BuildState>,
) {
    for construction in CONSTRUCTION_KINDS {
        if keyboard.just_pressed(construction.hotkey()) {
            build_state.select_construction(construction);
        }
    }

    if keyboard.just_pressed(KeyCode::KeyG) {
        build_state.snap_to_grid = !build_state.snap_to_grid;
        build_state.status = format!(
            "网格吸附已{}。",
            if build_state.snap_to_grid {
                "打开"
            } else {
                "关闭"
            }
        );
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        build_state.cancel();
    } else if build_state.selected == Some(ConstructionKind::Farm)
        && mouse_buttons.just_pressed(MouseButton::Right)
    {
        if build_state.farm_points.pop().is_some() {
            build_state.last_valid = false;
            build_state.invalid_reason = None;
            build_state.status = if build_state.farm_points.is_empty() {
                "正在规划农场。点击放置第一个角点。".to_string()
            } else {
                format!(
                    "已撤销角点。农场现有 {} 个角点。",
                    build_state.farm_points.len()
                )
            };
        }
    } else if build_state.selected.is_some() && mouse_buttons.just_pressed(MouseButton::Right) {
        build_state.cancel();
    }
}

// 拆除建筑：选中建筑 / 蓝图 / 农场地块后按 Delete 拆除。
// 建筑实体及其子实体（视觉体 / 入口标记 / 庄稼）会一并销毁；
// 占用与保留的入口从 WorldGeometry 释放（寻路自动增量重建），
// 相关殖民者的家 / 岗位由 assign_housing / assign_workplaces 自动重新分配。
pub fn handle_demolish(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut build_state: ResMut<BuildState>,
    mut geometry: ResMut<WorldGeometry>,
    mut selection: ResMut<SelectionState>,
    central_storages: Query<(), With<CentralStorage>>,
    blueprints: Query<&Blueprint>,
    completed: Query<&CompletedBuilding>,
) {
    if !keyboard.just_pressed(KeyCode::Delete) {
        return;
    }
    if build_state.selected.is_some() {
        return;
    }
    let Some(target) = selection.selected else {
        return;
    };
    let entity = target.entity();
    // 中央仓库是开局基础设施，禁止拆除，避免玩家把游戏拆死。
    if central_storages.contains(entity) {
        return;
    }
    let label = match target {
        SelectedTarget::Building(entity) => completed
            .get(entity)
            .map(|building| building.kind.definition().label)
            .unwrap_or("建筑"),
        SelectedTarget::Blueprint(entity) => blueprints
            .get(entity)
            .map(|blueprint| blueprint.kind.label())
            .unwrap_or("蓝图"),
        SelectedTarget::Farm(_) => "农场",
        SelectedTarget::Colonist(_) | SelectedTarget::Resource(_) => return,
    };
    geometry.release_entity(entity);
    commands.entity(entity).despawn();
    selection.selected = None;
    info!("[拆除] 已拆除 {label}（{entity:?}）");
    build_state.status = format!("已拆除：{}。", label);
}

pub fn handle_rotation_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut build_state: ResMut<BuildState>,
) {
    if build_state
        .selected
        .and_then(ConstructionKind::as_building)
        .is_none()
    {
        return;
    }

    if build_state.snap_to_grid {
        if keyboard.just_pressed(KeyCode::KeyR) {
            build_state.rotation_angle = (build_state.rotation_angle + std::f32::consts::FRAC_PI_2)
                .rem_euclid(std::f32::consts::TAU);
        }
    } else {
        if keyboard.just_pressed(KeyCode::KeyR) {
            build_state.r_hold_timer = 0.0;
        }
        if keyboard.pressed(KeyCode::KeyR) {
            build_state.r_hold_timer += time.delta_secs();
            if build_state.r_hold_timer >= 0.2 {
                build_state.rotation_angle = (build_state.rotation_angle
                    + std::f32::consts::PI * time.delta_secs())
                .rem_euclid(std::f32::consts::TAU);
            }
        }
        if keyboard.just_released(KeyCode::KeyR) {
            if build_state.r_hold_timer > 0.0 && build_state.r_hold_timer < 0.2 {
                build_state.rotation_angle = (build_state.rotation_angle
                    + std::f32::consts::FRAC_PI_2)
                    .rem_euclid(std::f32::consts::TAU);
            }
            build_state.r_hold_timer = 0.0;
        }
    }
}
