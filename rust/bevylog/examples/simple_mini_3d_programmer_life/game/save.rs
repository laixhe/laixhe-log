//! 存档系统：核心进度序列化到项目根目录 save.json。
//! - 菜单「继续游戏」读档进入
//! - 每日变化自动保存（auto_save 系统）
//! - 调试键 S 手动保存

use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::PlayerRoot;
use super::job::JobPipeline;
use super::resources::*;

pub const SAVE_PATH: &str = "save.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub stats: PlayerStats,
    pub clock: GameClock,
    pub flags: GameFlags,
    pub pipeline: JobPipeline,
    pub location: Location,
    pub bonus: WorkBonus,
    #[serde(default)] // 兼容旧档（旧档没有以下字段）
    pub player_pos: [f32; 3], // 存档时玩家在地图上的世界坐标
    #[serde(default)]
    pub transit: Option<TransitState>, // 行驶中为 Some（读档后回到该位置继续行驶）
    #[serde(default)]
    pub advance_stamp: Option<(u32, u32, u32)>, // 求职推进去重标记（读档当天不重复推进）
    // 弹窗/事件/对话/笔试状态：弹窗打开时存档，读档后恢复，避免进度丢失
    #[serde(default)]
    pub modal: Modal,
    #[serde(default)]
    pub event: EventState,
    #[serde(default)]
    pub quiz: QuizState,
    #[serde(default)]
    pub dialog: DialogueState,
    #[serde(default)]
    pub settle: DialogSettle,
}

// 菜单「继续游戏」点下后暂存，Playing 进入时应用
#[derive(Resource, Default)]
pub struct PendingLoad(pub Option<SaveData>);

#[allow(clippy::too_many_arguments)]
pub fn save_game(
    stats: &PlayerStats,
    clock: &GameClock,
    flags: &GameFlags,
    pipeline: &JobPipeline,
    location: Location,
    bonus: &WorkBonus,
    player_pos: Vec3,
    transit: Option<TransitState>,
    stamp: &JobAdvanceStamp,
    modal: &Modal,
    event: &EventState,
    quiz: &QuizState,
    dialog: &DialogueState,
    settle: &DialogSettle,
) -> bool {
    let data = SaveData {
        stats: stats.clone(),
        clock: clock.clone(),
        flags: flags.clone(),
        pipeline: pipeline.clone(),
        location,
        bonus: bonus.clone(),
        player_pos: [player_pos.x, player_pos.y, player_pos.z],
        transit,
        advance_stamp: stamp.0,
        modal: modal.clone(),
        event: event.clone(),
        quiz: quiz.clone(),
        dialog: dialog.clone(),
        settle: settle.clone(),
    };
    match serde_json::to_string(&data) {
        Ok(json) => std::fs::write(SAVE_PATH, json).is_ok(),
        Err(_) => false,
    }
}

pub fn load_game() -> Option<SaveData> {
    let json = std::fs::read_to_string(SAVE_PATH).ok()?;
    serde_json::from_str(&json).ok()
}

pub fn has_save() -> bool {
    Path::new(SAVE_PATH).exists()
}

// 读档后覆盖默认资源（reset_playing 之后执行）。
// 行驶中存档：骑行一律取消；行驶状态 + 玩家地图位置交给 scene_manager 恢复，
// 读档后玩家回到原位置继续自动行驶（过程在地图上，不传送）。
#[allow(clippy::too_many_arguments)]
pub fn apply_save(
    mut pending: ResMut<PendingLoad>,
    mut stats: ResMut<PlayerStats>,
    mut clock: ResMut<GameClock>,
    mut flags: ResMut<GameFlags>,
    mut pipeline: ResMut<JobPipeline>,
    mut location: ResMut<GameLocation>,
    mut bonus: ResMut<WorkBonus>,
    mut bike: ResMut<BikeMode>,
    mut resume: ResMut<SceneResume>,
    mut stamp: ResMut<JobAdvanceStamp>,
    mut modal: ResMut<Modal>,
    mut event: ResMut<EventState>,
    mut quiz: ResMut<QuizState>,
    mut dialog: ResMut<DialogueState>,
    mut settle: ResMut<DialogSettle>,
    force: Res<SceneForce>,
) {
    let Some(data) = pending.0.take() else {
        return;
    };
    *stats = data.stats;
    *clock = data.clock;
    *flags = data.flags;
    *pipeline = data.pipeline;
    location.0 = data.location;
    *bonus = data.bonus;
    bike.0 = false;
    resume.0 = data.transit.map(|t| (Vec3::from_array(data.player_pos), t));
    // 恢复求职推进去重标记：读档当天不应再推进一次。
    // 存档里的 force 是旧进程的计数，读档后要以当前 force 为准，只保留周/日。
    stamp.0 = data.advance_stamp.map(|(_, w, d)| (force.0, w, d));
    // 恢复弹窗/事件/对话/笔试状态：弹窗打开时存档，读档后照常继续
    *modal = data.modal;
    *event = data.event;
    *quiz = data.quiz;
    *dialog = data.dialog;
    *settle = data.settle;
    info!(
        "[存档] 读取存档：第{}周 第{}天{}",
        clock.week,
        clock.day,
        if resume.0.is_some() {
            "（存档时在乘车途中，将回到地图原位置继续行驶）"
        } else {
            ""
        }
    );
}

// 每日关键状态变化时自动保存
#[allow(clippy::too_many_arguments)]
pub fn auto_save(
    stats: Res<PlayerStats>,
    clock: Res<GameClock>,
    flags: Res<GameFlags>,
    pipeline: Res<JobPipeline>,
    location: Res<GameLocation>,
    bonus: Res<WorkBonus>,
    transit: Res<TransitState>,
    stamp: Res<JobAdvanceStamp>,
    modal: Res<Modal>,
    event: Res<EventState>,
    quiz: Res<QuizState>,
    dialog: Res<DialogueState>,
    settle: Res<DialogSettle>,
    player: Single<&Transform, With<PlayerRoot>>,
    mut last: Local<(u32, u32, Phase)>,
) {
    let key = (clock.week, clock.day, clock.phase);
    if *last == key {
        return;
    }
    *last = key;
    let transit = transit.active.then(|| transit.clone());
    if save_game(
        &stats,
        &clock,
        &flags,
        &pipeline,
        location.0,
        &bonus,
        player.translation,
        transit,
        &stamp,
        &modal,
        &event,
        &quiz,
        &dialog,
        &settle,
    ) {
        info!("[存档] 已自动保存 第{}周 第{}天", clock.week, clock.day);
    } else {
        warn!("[存档] 自动保存失败（写入 {} 出错）", SAVE_PATH);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::components::CommuteChoice;

    #[test]
    fn save_roundtrip_keeps_position_and_transit() {
        let transit = TransitState {
            active: true,
            mode: CommuteChoice::Subway,
            target: Location::Office,
            waypoints: vec![[0.0, 0.0], [0.0, 42.0]],
            wp_idx: 1,
        };
        let data = SaveData {
            stats: PlayerStats::default(),
            clock: GameClock::default(),
            flags: GameFlags::default(),
            pipeline: JobPipeline::default(),
            location: Location::Home,
            bonus: WorkBonus::default(),
            player_pos: [3.0, 0.0, 12.0],
            transit: Some(transit),
            advance_stamp: Some((1, 3, 2)),
            modal: Modal::default(),
            event: EventState::default(),
            quiz: QuizState::default(),
            dialog: DialogueState::default(),
            settle: DialogSettle::default(),
        };
        let json = serde_json::to_string(&data).unwrap();
        let back: SaveData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.player_pos, [3.0, 0.0, 12.0]);
        let t = back.transit.expect("行驶状态应被保存");
        assert!(t.active);
        assert_eq!(t.target, Location::Office);
        assert_eq!(back.advance_stamp, Some((1, 3, 2)), "去重标记应被保存");
    }

    #[test]
    fn old_save_without_new_fields_still_loads() {
        // 模拟旧版存档（没有 player_pos / transit 字段），serde(default) 应补默认值
        let old = r#"{"stats":{"energy":80.0,"mentality":80.0,"health":80.0,"satiety":80.0,"money":2200.0,"skills":[18.0,18.0,18.0,18.0,18.0]},"clock":{"week":1,"day":1,"phase":"Morning"},"flags":{"intern_offer":false,"formal_offer":false,"best_tier":4294967295,"salary":0.0,"intern_ok":false,"rejected_count":0,"applied_count":0},"pipeline":{"apps":[],"last_apply_day":0},"location":"Home","bonus":{"used_today":false}}"#;
        let back: SaveData = serde_json::from_str(old).unwrap();
        assert!(back.transit.is_none());
        assert_eq!(back.player_pos, [0.0, 0.0, 0.0]);
    }
}
