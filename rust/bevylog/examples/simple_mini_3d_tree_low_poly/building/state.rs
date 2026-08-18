//! `BuildState`：当前选中的建筑类型、旋转角、网格吸附、预览实体、农场角点等建造状态；
//! `PlacementIssue` 枚举表示「不能放」的原因，UI 会把原因显示给玩家。

use bevy::prelude::*;

use crate::types::ConstructionKind;

#[derive(Resource, Debug)]
pub struct BuildState {
    pub selected: Option<ConstructionKind>,
    pub snap_to_grid: bool,
    pub rotation_angle: f32,
    pub r_hold_timer: f32,
    pub preview_entity: Option<Entity>,
    pub preview_entrance_entity: Option<Entity>,
    pub last_valid: bool,
    pub last_position: Vec3,
    pub last_polygon: Vec<Vec2>,
    pub last_access_point: Option<Vec3>,
    pub farm_points: Vec<Vec2>,
    pub invalid_reason: Option<PlacementIssue>,
    pub status: String,
}

impl Default for BuildState {
    fn default() -> Self {
        Self {
            selected: None,
            snap_to_grid: true,
            rotation_angle: 0.0,
            r_hold_timer: 0.0,
            preview_entity: None,
            preview_entrance_entity: None,
            last_valid: false,
            last_position: Vec3::ZERO,
            last_polygon: Vec::new(),
            last_access_point: None,
            farm_points: Vec::new(),
            invalid_reason: None,
            status: "选择一个建筑开始规划。".to_string(),
        }
    }
}

impl BuildState {
    pub fn select_construction(&mut self, construction: ConstructionKind) {
        self.selected = Some(construction);
        self.last_valid = false;
        self.last_polygon.clear();
        self.last_access_point = None;
        self.invalid_reason = None;
        self.farm_points.clear();
        self.status = match construction {
            ConstructionKind::Building(kind) => {
                format!("正在规划：{}。", kind.definition().label)
            }
            ConstructionKind::Farm => "正在规划农场。点击放置第一个角点。".to_string(),
        };
    }

    pub fn cancel(&mut self) {
        self.selected = None;
        self.last_valid = false;
        self.last_polygon.clear();
        self.last_access_point = None;
        self.invalid_reason = None;
        self.farm_points.clear();
        self.status = "已取消建造。".to_string();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlacementIssue {
    OutOfBounds,
    Occupied,
    EntranceBlocked,
    TooSteep,
    TooFewPoints,
    InvalidShape,
}

impl PlacementIssue {
    pub fn label(self) -> &'static str {
        match self {
            Self::OutOfBounds => "超出可建造区域",
            Self::Occupied => "被其他规划、建筑、资源或入口占用",
            Self::EntranceBlocked => "入口被阻挡",
            Self::TooSteep => "地形过于陡峭",
            Self::TooFewPoints => "至少需要三个角点",
            Self::InvalidShape => "轮廓必须是凸且不自交的多边形",
        }
    }
}
