//! 组件定义：把「这个游戏有哪些组件」集中放在一个文件里，
//! 便于新手总览。标记组件（GameRoot / SceneRoot）用于批量清理实体，
//! 数据组件（Hotspot / SealFill…）用于区分实体行为。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::resources::{HotspotKind, Location};

// ==================== 组件定义 ====================
// 游戏根标记：Playing 页所有实体都挂上，OnExit 时一键清理
#[derive(Component, Clone, Default)]
pub struct GameRoot;

// 主角标记：行走 / 相机跟随都用它定位
#[derive(Component)]
pub struct PlayerRoot;

// 多段式主角的肢体挂点标记（程序化行走动画用）
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum LimbKind {
    ArmL,
    ArmR,
    LegL,
    LegR,
    Body,
}

// 场景根标记：切换地点时重建场景用（地面 / 装饰 / 热点）
#[derive(Component)]
pub struct SceneRoot;

// 场景热点：点击 / 自动寻路到达后触发行为
#[derive(Component)]
pub struct Hotspot {
    pub kind: HotspotKind,
}

// 碰撞盒（XZ 平面 AABB）：玩家行走 / 寻路 / NPC 巡逻都不能穿过。
// half = 半宽/半深；bottom = 底部高度，>1.0 的高空装饰（屋顶横梁等）不阻挡地面行走。
#[derive(Component)]
pub struct Solid {
    pub half: Vec2,
    pub bottom: f32,
}

// ==================== HUD 标记 ====================
#[derive(Component)]
pub struct BannerText;

#[derive(Component)]
pub struct MoneyText;

// 三种圆形字章的填充圆（用高度百分比表示数值）
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SealKind {
    Energy,
    Mentality,
    Health,
}

#[derive(Component)]
pub struct SealFill {
    pub which: SealKind,
}

#[derive(Component)]
pub struct SealValue {
    pub which: SealKind,
}

#[derive(Component)]
pub struct SkillPill {
    pub idx: usize,
}

#[derive(Component)]
pub struct ToastText;

// ==================== 弹窗标记 ====================
#[derive(Component)]
pub struct DlgRoot;

#[derive(Component)]
pub struct QuizRoot;

#[derive(Component)]
pub struct CompanyRoot;

#[derive(Component)]
pub struct CommuteRoot;

// 随机事件弹窗
#[derive(Component)]
pub struct EventRoot;

// 事件选项按钮（携带选项下标）
#[derive(Component)]
pub struct EventOption(pub usize);

// 对话选项按钮（携带选项下标）
#[derive(Component)]
pub struct OptionButton(pub usize);

// 笔试选项按钮（携带选项下标）
#[derive(Component)]
pub struct QuizOption(pub usize);

// 笔试「放弃」按钮：视为未通过（被拒），给玩家一条不答题的退出路径
#[derive(Component)]
pub struct QuizGiveUp;

// 交通面板目的地按钮（携带目的地区域）
#[derive(Component)]
pub struct CommuteButton(pub Location);

// 公司行（下标暂只作标记，供未来扩展）
#[derive(Component)]
#[allow(dead_code)]
pub struct CompanyRow(pub usize);

#[derive(Component)]
pub struct ApplyButton(pub usize);

#[derive(Component)]
pub struct ClosePanelButton;

// 交通方式：地铁（快）/ 公交（慢）；共享单车走骑行状态（BikeMode 资源）
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CommuteChoice {
    Subway,
    Bus,
}

impl CommuteChoice {
    pub fn label(self) -> &'static str {
        match self {
            CommuteChoice::Subway => "地铁",
            CommuteChoice::Bus => "公交",
        }
    }
}

// ==================== 食堂排队 NPC ====================
#[derive(Component)]
pub struct QueueNpc {
    pub phase: f32, // 浮动相位
}

// 场景中可点击对话的 NPC 实体（dialogue 下标 1-7）
#[derive(Component)]
pub struct NpcMarker {
    pub idx: usize,
}

// 巡逻 NPC：在两点间往返走动（A* 绕行路径，不穿建筑）
#[derive(Component)]
pub struct WanderNpc {
    pub from: Vec3,
    pub to: Vec3,
    pub speed: f32,
    pub t: f32,          // 相位
    pub path: Vec<Vec2>, // 绕行路径点（世界坐标 XZ，首次巡逻时计算）
}

// 主方向光（昼夜光照系统控制）
#[derive(Component)]
pub struct DayLight;

// UI 世界标签：把 NPC 名牌 / 热点标签做成 UI 文字（屏幕像素），
// 每帧由 update_world_labels 将 target 实体的世界坐标投影到屏幕并更新 UI 位置。
// 绕开 Bevy 0.19 Text2d 无法缩放的问题（Text2d 加任何非单位 scale 都会导致文字不渲染）。
// 显示状态机：showing 表示「是否进入显示区」（进半径亮、出迟滞距离灭），
// alpha 为当前透明度（0..1），由 update_world_labels 每帧朝目标值淡入/淡出。
#[derive(Component)]
pub struct WorldLabel {
    // —— 定位配置（spawn 时一次写好，之后只读）——
    pub target: Entity,   // 被标注的世界实体（NPC 或热点）
    pub offset: f32,      // 世界高度偏移：名牌锚点相对目标脚下抬多高（数值见 scenes.rs 的 *_LABEL_OFFSET 常量）
    pub est_width: f32,   // 估算文本宽度（用于水平居中，见 spawn_world_labels 的估算说明）
    pub font_size: f32,   // 字号（用于把文字整体上移到头顶上方）
    // —— 屏幕位置状态（update_world_labels 每帧维护）——
    pub last_left: f32,   // 上一帧写入 node 的取整 left（用于判断是否变化，避免触发重布局）
    pub last_top: f32,    // 上一帧写入 node 的取整 top（同上）
    pub cur_left: f32,    // 帧间插值后的屏幕 left（NAN = 未定位），见 LABEL_SMOOTHING
    pub cur_top: f32,     // 帧间插值后的屏幕 top（NAN = 未定位）
    // —— 显隐状态（update_world_labels 每帧维护）——
    pub showing: bool,    // 是否处于「显示区」（进半径亮、出迟滞距离灭，倍数见 LABEL_HYSTERESIS）
    pub alpha: f32,       // 当前透明度 0..1（淡入淡出，避免名牌瞬间出现/消失的生硬感）
}
