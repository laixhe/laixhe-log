//! 全局资源与数值核心：玩家属性、游戏时钟、地点、弹窗状态、
//! 求职管线等所有 Resource 都定义在这里，并附章节 / 星期 / 技能成长
//! 等纯函数（chapter_name / day_label / skill_gain / work_income）。

use bevy::image::Image;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::components::CommuteChoice;

// ==================== 全局常量 ====================
pub const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
pub const WORLD_HALF: f32 = 60.0; // 城市半尺寸（120×120 的大城市）
pub const PLAYER_SPEED: f32 = 7.5; // 步行速度
pub const BIKE_SPEED_MULT: f32 = 2.5; // 共享单车速度倍率
pub const TOTAL_WEEKS: u32 = 26; // 总周数

// ==================== 城市区域 ====================
// 统一大地图：区域错位分布在十字主路四周（不压在道路上），
// 市中心 (0,0) 为交通枢纽，主路 / 环路 / 支路构成城市路网。
// 区域内容以各中心为原点搭建，中心偏移量保证内容不覆盖主路。
pub const HOME_CENTER: Vec3 = Vec3::new(-42.0, 0.0, 28.0); // 城西·偏北
pub const CAMPUS_CENTER: Vec3 = Vec3::new(28.0, 0.0, -42.0); // 城南·偏东
pub const CAFETERIA_CENTER: Vec3 = Vec3::new(42.0, 0.0, -28.0); // 城东·偏南
pub const OFFICE_CENTER: Vec3 = Vec3::new(-28.0, 0.0, 42.0); // 城北·偏西
pub const PARK_CENTER: Vec3 = Vec3::new(42.0, 0.0, 28.0); // 城东北·公园（环路外）

// 各区域的站点坐标（区域门口、支路终点，靠近主路一侧）
pub fn station_pos(loc: Location) -> Vec3 {
    match loc {
        Location::Home => Vec3::new(-42.0, 0.0, 15.0),
        Location::Campus => Vec3::new(15.0, 0.0, -42.0),
        Location::Cafeteria => Vec3::new(42.0, 0.0, -15.0),
        Location::Office => Vec3::new(-15.0, 0.0, 42.0),
        Location::Park => Vec3::new(42.0, 0.0, 15.0),
        Location::Road => Vec3::ZERO,
    }
}

// 区域归属判定半径：距离区域中心 < 该值视为在该区域内
const REGION_RADIUS: f32 = 18.0;

// 根据世界坐标判断所在区域；市中心/道路等公共区域返回 Road
pub fn region_of(pos: Vec3) -> Location {
    let mut best = Location::Road;
    let mut best_d = REGION_RADIUS;
    for (loc, c) in [
        (Location::Home, HOME_CENTER),
        (Location::Campus, CAMPUS_CENTER),
        (Location::Cafeteria, CAFETERIA_CENTER),
        (Location::Office, OFFICE_CENTER),
        (Location::Park, PARK_CENTER),
    ] {
        let d = pos.xz().distance(c.xz());
        if d < best_d {
            best_d = d;
            best = loc;
        }
    }
    best
}

// ==================== 章节 ====================
pub fn chapter_of(week: u32) -> u32 {
    match week {
        1..=4 => 1,
        5..=10 => 2,
        11..=18 => 3,
        19..=22 => 4,
        _ => 5,
    }
}

pub fn chapter_name(ch: u32) -> &'static str {
    match ch {
        1 => "大三暑期",
        2 => "实习期",
        3 => "秋招季",
        4 => "毕业入职",
        _ => "职场日常",
    }
}

pub fn day_label(day: u32) -> &'static str {
    match day {
        1 => "周一",
        2 => "周二",
        3 => "周三",
        4 => "周四",
        5 => "周五",
        6 => "周六",
        7 => "周日",
        _ => "?",
    }
}

// ==================== 时段 / 地点 ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Morning,
    Work,
    Lunch,
    Evening,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Morning => "上午",
            Phase::Work => "工作",
            Phase::Lunch => "午饭",
            Phase::Evening => "晚上",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum Location {
    #[default]
    Home,
    Campus,
    Cafeteria,
    Office,
    Park, // 公园（公共休闲区）
    Road, // 城市道路 / 市中心（公共区域）
}

// ==================== 交通状态 ====================
// 统一地图后：地铁/公交在站点上车，沿道路网（支路 → 主路 → 市中心 → 主路 → 支路）
// 自动行驶到目的区域站点，全程不穿建筑；共享单车是"骑行状态"（BikeMode 资源）。
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct TransitState {
    pub active: bool,        // 是否正在乘车自动行驶
    pub mode: CommuteChoice, // 交通方式（地铁 / 公交）
    pub target: Location,    // 目的地区域
    #[serde(default)]
    pub waypoints: Vec<[f32; 2]>, // 沿道路网的路点（世界坐标 XZ）
    #[serde(default)]
    pub wp_idx: usize, // 当前行驶到第几个路点
}

impl Default for TransitState {
    fn default() -> Self {
        Self {
            active: false,
            mode: CommuteChoice::Subway,
            target: Location::Office,
            waypoints: Vec::new(),
            wp_idx: 0,
        }
    }
}

// 共享单车骑行状态（骑上后步行速度 ×BIKE_SPEED_MULT）
#[derive(Resource, Default)]
pub struct BikeMode(pub bool);

// 读档标记：行驶中存档时记录「玩家地图位置 + 行驶状态」，
// 读档后由 scene_manager 放回原位置继续行驶（过程在地图上，不传送）。
#[derive(Resource, Default)]
pub struct SceneResume(pub Option<(Vec3, TransitState)>);

// ==================== 玩家属性 ====================
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub energy: f32,      // 精力 0-100
    pub mentality: f32,   // 心态 0-100，归零即崩溃
    pub health: f32,      // 健康 0-100
    pub satiety: f32,     // 饱食 0-100
    pub money: f32,       // 金钱
    pub skills: [f32; 5], // 算法 / 八股 / 项目 / 社交 / 简历
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            energy: 80.0,
            mentality: 80.0,
            health: 80.0,
            satiety: 80.0,
            money: 2200.0,
            skills: [18.0; 5],
        }
    }
}

pub const SKILL_NAMES: [&str; 5] = ["算法", "八股", "项目", "社交", "简历"];

// 技能成长：精力缩放 + 边际递减。状态越好练得越快，越接近满分越难涨。
// 满级（100）直接返回 0，避免「toast 显示 +0.5、实际数值撞顶不动」的误导；
// 非满级时每次至少涨 0.5（保底，保证练习永远有正反馈）。
pub fn skill_gain(stats: &PlayerStats, idx: usize, base: f32) -> f32 {
    if stats.skills[idx] >= 100.0 {
        return 0.0;
    }
    let energy_factor = 0.35 + 0.65 * (stats.energy / 100.0);
    let diminishing = 1.0 - stats.skills[idx] / 100.0;
    (base * energy_factor * diminishing).max(0.5)
}

// 修改属性并夹取到 0..100，返回是否发生变化
pub fn change(v: &mut f32, delta: f32) {
    *v = (*v + delta).clamp(0.0, 100.0);
}

// ==================== 时段推进 ====================
// 每个时段按真实秒数自动推进（由 progression::phase_tick 驱动）：
// 上午 → 工作 → 午饭 → 晚上；晚上停留在深夜，等玩家上床睡觉才推进天数。
// 这样恢复"早上去上班、晚上回家睡觉"的工作日节奏感。
pub const MORNING_DURATION: f32 = 18.0;
pub const WORK_DURATION: f32 = 22.0;
pub const LUNCH_DURATION: f32 = 16.0;
pub const EVENING_DURATION: f32 = 25.0;

pub fn phase_duration(phase: Phase) -> f32 {
    match phase {
        Phase::Morning => MORNING_DURATION,
        Phase::Work => WORK_DURATION,
        Phase::Lunch => LUNCH_DURATION,
        Phase::Evening => EVENING_DURATION,
    }
}

// 下一个时段；晚上是当日终点，停留在晚上等玩家睡觉
pub fn next_phase(phase: Phase) -> Phase {
    match phase {
        Phase::Morning => Phase::Work,
        Phase::Work => Phase::Lunch,
        Phase::Lunch => Phase::Evening,
        Phase::Evening => Phase::Evening,
    }
}

// ==================== 游戏时钟 ====================
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GameClock {
    pub week: u32, // 1-26
    pub day: u32,  // 1-7（1 = 周一）
    pub phase: Phase,
    #[serde(default)] // 兼容旧存档（旧档没有该字段）
    pub phase_t: f32, // 当前时段已流逝的秒数（推进到下一时段用）
}

impl Default for GameClock {
    fn default() -> Self {
        Self {
            week: 1,
            day: 1,
            phase: Phase::Morning,
            phase_t: 0.0,
        }
    }
}

impl GameClock {
    pub fn chapter(&self) -> u32 {
        chapter_of(self.week)
    }
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct GameLocation(pub Location);

// ==================== 行走状态 ====================
#[derive(Clone, Copy, PartialEq, Default)]
pub enum WalkCmd {
    #[default]
    Move,
    Interact(HotspotKind),
    Npc(usize), // 走向某位 NPC 对话（NPCS 下标 1-7）
    Food,       // 自主觅食：到点后由到达系统解析为最近觅食点
    Sleep,
}

#[derive(Resource, Default)]
pub struct WalkState {
    pub target: Option<Vec3>,
    pub path: Vec<Vec2>,           // A* 路径点（世界坐标 XZ，绕开建筑）
    pub path_target: Option<Vec3>, // 路径对应的目标（目标变化时重算）
    pub cmd: WalkCmd,
    pub bob: f32, // 走路起伏相位
}

// 到达后待执行的行为（由 movement 写入、interactions 消费）
#[derive(Debug)]
pub enum PendingKind {
    Hotspot(HotspotKind),
    Npc(usize),
}

#[derive(Resource, Default)]
pub struct PendingAction(pub Option<PendingKind>);

// 自主行为冷却（秒），避免每帧反复触发
#[derive(Resource, Default)]
pub struct AutoCooldown {
    pub food: f32,
    pub sleep: f32,
}

// 求职推进去重标记：记录上次已推进的 (force, week, day)，避免每帧重复推进。
// 用 Resource 而非 Local，是为了能随存档序列化——否则读档后 Local 归零，
// 读档当天会被再推进一次（笔试重弹、等待天数虚增）。
#[derive(Resource, Default, Clone, Copy, Serialize, Deserialize)]
pub struct JobAdvanceStamp(pub Option<(u32, u32, u32)>);

// ==================== 热点 ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HotspotKind {
    // 家
    Bed,
    Desk,
    Books,
    Kitchen,
    Computer,
    Phone,
    Tv,
    Bathroom,
    Fridge,
    // 校园
    Track,
    TechGroup,
    Library,
    Lab,
    CampusShop,
    DormBed,
    DormGame,
    DormSnack,
    // 食堂
    Canteen1,
    Canteen2,
    Microwave,
    InstantNoodle,
    DrinkMachine,
    MilkTea,
    FruitStand,
    // 办公室
    Workstation,
    Lounge,
    Slacking,
    Takeout,
    Coffee,
    Meeting,
    Printer,
    // 公园
    ParkBench,
    ParkFountain,
    // 城市交通
    SubwayStop,
    BusStop,
    BikeSpot,
    // 校园周边探索
    NightMarket,
    Lookout,
    Graffiti,
}

impl HotspotKind {
    pub fn label(self) -> &'static str {
        match self {
            HotspotKind::Bed => "床",
            HotspotKind::Desk => "书桌",
            HotspotKind::Books => "书堆",
            HotspotKind::Kitchen => "厨房",
            HotspotKind::Computer => "电脑",
            HotspotKind::Phone => "手机",
            HotspotKind::Tv => "电视",
            HotspotKind::Bathroom => "浴室",
            HotspotKind::Fridge => "冰箱",
            HotspotKind::Track => "操场",
            HotspotKind::TechGroup => "技术群",
            HotspotKind::Library => "图书馆",
            HotspotKind::Lab => "实验室",
            HotspotKind::CampusShop => "小卖部",
            HotspotKind::DormBed => "宿舍床",
            HotspotKind::DormGame => "室友电脑",
            HotspotKind::DormSnack => "零食柜",
            HotspotKind::Canteen1 => "大众菜",
            HotspotKind::Canteen2 => "小炒",
            HotspotKind::Microwave => "微波炉",
            HotspotKind::InstantNoodle => "泡面",
            HotspotKind::DrinkMachine => "饮料机",
            HotspotKind::MilkTea => "奶茶",
            HotspotKind::FruitStand => "水果摊",
            HotspotKind::Workstation => "工位",
            HotspotKind::Lounge => "茶水间",
            HotspotKind::Slacking => "摸鱼",
            HotspotKind::Takeout => "点外卖",
            HotspotKind::Coffee => "咖啡机",
            HotspotKind::Meeting => "会议室",
            HotspotKind::Printer => "打印机",
            HotspotKind::ParkBench => "长椅",
            HotspotKind::ParkFountain => "喷泉",
            HotspotKind::SubwayStop => "地铁站",
            HotspotKind::BusStop => "公交站",
            HotspotKind::BikeSpot => "共享单车",
            HotspotKind::NightMarket => "夜市",
            HotspotKind::Lookout => "观景台",
            HotspotKind::Graffiti => "涂鸦墙",
        }
    }

    // 注意：新增「觅食类」热点必须同步加入此列表，否则饱食低时自主觅食
    // （movement::nearest_food）会静默漏掉它（matches! 白名单没有编译期检查）。
    pub fn is_food(self) -> bool {
        matches!(
            self,
            HotspotKind::Kitchen
                | HotspotKind::Canteen1
                | HotspotKind::Canteen2
                | HotspotKind::Microwave
                | HotspotKind::InstantNoodle
                | HotspotKind::Takeout
                | HotspotKind::CampusShop
                | HotspotKind::Fridge
                | HotspotKind::DrinkMachine
                | HotspotKind::MilkTea
                | HotspotKind::FruitStand
                | HotspotKind::DormSnack
        )
    }
}

// ==================== 消息（Toast） ====================
#[derive(Resource, Default)]
pub struct ToastLog {
    pub items: VecDeque<(String, f32)>, // (文本, 剩余秒数)
}

impl ToastLog {
    pub fn push(&mut self, s: impl Into<String>) {
        self.items.push_back((s.into(), 4.5));
        // 最多保留 4 条，避免多行为连续触发时房租/offer 等关键提示被过早挤出
        while self.items.len() > 4 {
            self.items.pop_front();
        }
    }
}

// ==================== 弹窗 ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ModalKind {
    Dialogue, // 对话
    Quiz,     // 笔试答题
    Commute,  // 选通勤方式
    Company,  // 投简历面板
    Event,    // 随机事件（选项化）
}

// Modal 可序列化：弹窗打开时存档，读档后恢复弹窗 UI（避免事件/笔试/对话进度丢失）
#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Modal {
    pub kind: Option<ModalKind>,
    pub version: u32, // 内容变化时 +1，触发弹窗 UI 重建
}

impl Modal {
    pub fn open(&mut self, k: ModalKind) {
        self.kind = Some(k);
        self.version += 1;
    }
    pub fn close(&mut self) {
        self.kind = None;
        self.version += 1;
    }
    pub fn refresh(&mut self) {
        self.version += 1;
    }
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct DialogueState {
    pub npc: usize,
    pub node: usize,
}

// ==================== 对话效果结算记录 ====================
// 聊天不消耗行动槽位，若不限制，玩家可反复与同一 NPC 对话无限刷取心态/技能。
// 规则：每个 NPC 的对话效果每天只结算一次；天数变化时自动重置。
// 随存档序列化：读档当天不重置防刷记录（否则读档后可再结算一次）。
#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct DialogSettle {
    pub day: u32, // 生效的天（与 GameClock.day 对比，不同则清空）
    pub npcs: Vec<usize>, // 当天已结算过效果的 NPC 下标
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct QuizState {
    pub app: usize, // 正在答题的投递下标
    pub q: usize,   // 题目下标
}

// 当前随机事件（选项化弹窗用；随存档序列化，读档后恢复事件弹窗/延后标记）
#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct EventState {
    pub idx: usize,    // EVENTS 下标
    pub pending: bool, // 进入晚上时已有弹窗打开，事件延后到弹窗关闭后再触发
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct WorkBonus {
    pub used_today: bool, // 工位/茶水间 白拿每日限一次
}

// ==================== 免费恢复热点每日限次 ====================
// 浴室/冰箱/微波炉/咖啡/长椅/喷泉/观景台/涂鸦/床补觉等免费恢复类热点
// 无成本即可反复触发，若不限制可无限刷满精力/心态/饱食/健康，绕开
// 「低精力做事事倍功半」的制衡。规则：每天每个热点首次全额，之后效果减半。
#[derive(Resource, Default)]
pub struct FreeUse {
    pub day: u32,                // 生效的天（与 GameClock.day 对比，不同则清空）
    pub used: Vec<HotspotKind>,  // 当天已全额结算过的免费热点
}

// ==================== 游戏标记（求职进展等） ====================
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GameFlags {
    pub intern_offer: bool, // 已拿到实习 offer
    pub formal_offer: bool, // 已拿到正式 offer
    pub best_tier: u32,     // 最好 offer 档次（0 最好；MAX 表示还没拿到）
    pub salary: f32,        // 当前 offer 日薪
    pub intern_ok: bool,    // 转正成功
    pub rejected_count: u32,
    pub applied_count: u32,
}

impl Default for GameFlags {
    fn default() -> Self {
        Self {
            intern_offer: false,
            formal_offer: false,
            best_tier: u32::MAX,
            salary: 0.0,
            intern_ok: false,
            rejected_count: 0,
            applied_count: 0,
        }
    }
}

// ==================== 结局 ====================
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OverReason {
    Mentality, // 心态崩溃
    Rent,      // 交不起房租
    Finished,  // 26 周走完
}

#[derive(Resource, Default)]
pub struct OverInfo {
    pub reason: Option<OverReason>,
}

#[derive(Resource, Default)]
pub struct Ending {
    pub title: String,
    pub desc: String,
}

// 场景重建强制信号：进入 Playing 时 +1，保证场景一定会重建
#[derive(Resource, Default)]
pub struct SceneForce(pub u32);

// ==================== 章节剧情演出 ====================
// 章节切换时播放的大字报过场：锁操作数秒后自动消失。
#[derive(Resource)]
pub struct Cinematic {
    pub active: bool,
    pub title: String,
    pub sub: String,
    pub t: f32,
    pub duration: f32,
}

impl Default for Cinematic {
    fn default() -> Self {
        Self {
            active: false,
            title: String::new(),
            sub: String::new(),
            t: 0.0,
            duration: 3.0,
        }
    }
}

impl Cinematic {
    pub fn play(&mut self, title: impl Into<String>, sub: impl Into<String>) {
        self.active = true;
        self.title = title.into();
        self.sub = sub.into();
        self.t = 0.0;
        self.duration = 3.0;
    }
}

// 程序化纸张纹理句柄
#[derive(Resource)]
pub struct PaperTexture(pub Handle<Image>);

// 每日收入
pub fn work_income(flags: &GameFlags, chapter: u32) -> f32 {
    if flags.formal_offer {
        flags.salary
    } else if flags.intern_ok {
        // 转正留用：答辩通过后薪资涨到 250/天（持续到拿到正式 offer）
        250.0
    } else if flags.intern_offer || chapter == 2 || chapter == 3 {
        // 实习期 / 秋招季白天实习
        150.0
    } else if chapter >= 4 {
        // 毕业未入职：自由职业/兼职兜底（低于 1500/周房租，仍形成持续经济压力）
        100.0
    } else {
        0.0
    }
}

// 每周房租
pub fn rent_amount(chapter: u32) -> f32 {
    match chapter {
        1 => 400.0,
        2 => 500.0,
        3 => 1000.0,
        _ => 1500.0,
    }
}
