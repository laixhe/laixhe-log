//! 建造相关组件与数据结构：
//! `Blueprint`（蓝图：所需木料 / 已运木料 / 建造进度 / 状态机）、`Footprint`（占地多边形）、
//! `Workplace`（工作岗位）、`Housing`（住房）、`Profession`（职业）、`BuildingVisual`（视觉体）等。

use bevy::prelude::*;

use crate::types::{BuildingKind, ConstructionKind};

#[derive(Component)]
pub struct BuildPreview;

#[derive(Component)]
pub struct EntrancePreview;

#[derive(Component, Debug)]
pub struct BuildingVisual {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct Footprint {
    pub polygon: Vec<Vec2>,
    pub passable: bool,
}

#[derive(Component, Debug)]
pub struct Blueprint {
    pub kind: ConstructionKind,
    pub required_wood: i32,
    pub delivered_wood: i32,
    pub progress: f32,
    pub build_seconds: f32,
}

impl Blueprint {
    pub fn needs_wood(&self) -> i32 {
        (self.required_wood - self.delivered_wood).max(0)
    }

    pub fn has_materials(&self) -> bool {
        self.needs_wood() == 0
    }

    pub fn is_complete(&self) -> bool {
        self.has_materials() && self.progress >= self.build_seconds
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.build_seconds <= 0.0 {
            1.0
        } else {
            (self.progress / self.build_seconds).clamp(0.0, 1.0)
        }
    }

    pub fn status(&self) -> BlueprintStatus {
        if !self.has_materials() {
            BlueprintStatus::WaitingForMaterials
        } else if self.is_complete() {
            BlueprintStatus::Complete
        } else if self.progress > 0.0 {
            BlueprintStatus::Building
        } else {
            BlueprintStatus::WaitingForBuilder
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlueprintStatus {
    WaitingForMaterials,
    WaitingForBuilder,
    Building,
    Complete,
}

impl BlueprintStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::WaitingForMaterials => "等待材料",
            Self::WaitingForBuilder => "等待建造者",
            Self::Building => "建造中",
            Self::Complete => "已完成",
        }
    }
}

#[derive(Component, Debug)]
pub struct CompletedBuilding {
    pub kind: BuildingKind,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum Profession {
    #[default]
    Unemployed,
    Lumberjack,
    Gatherer,
    WoodSplitter,
}

impl Profession {
    pub fn label(self) -> &'static str {
        match self {
            Self::Unemployed => "无业",
            Self::Lumberjack => "伐木工",
            Self::Gatherer => "采集工",
            Self::WoodSplitter => "劈柴工",
        }
    }

    pub fn for_building(kind: BuildingKind) -> Option<Self> {
        match kind {
            BuildingKind::Woodcutter => Some(Self::Lumberjack),
            BuildingKind::Gatherer => Some(Self::Gatherer),
            BuildingKind::ChoppingYard => Some(Self::WoodSplitter),
            BuildingKind::House | BuildingKind::Storage | BuildingKind::Road => None,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Workplace {
    pub profession: Profession,
    pub desired_slots: u8,
    pub max_slots: u8,
    pub work_seconds: f32,
}

impl Workplace {
    pub const DEFAULT_MAX_SLOTS: u8 = 2;

    pub fn for_building(kind: BuildingKind) -> Option<Self> {
        let profession = Profession::for_building(kind)?;
        let def = kind.definition();
        Some(Self {
            profession,
            desired_slots: Self::DEFAULT_MAX_SLOTS,
            max_slots: Self::DEFAULT_MAX_SLOTS,
            work_seconds: def.work_seconds,
        })
    }

    pub fn clamp_desired_slots(&mut self) {
        self.desired_slots = self.desired_slots.min(self.max_slots);
    }

    pub fn adjust_desired_slots(&mut self, delta: i8) {
        let next = self.desired_slots as i16 + delta as i16;
        self.desired_slots = next.clamp(0, self.max_slots as i16) as u8;
    }
}

#[derive(Component, Debug, Default)]
pub struct Housing {
    pub residents: Vec<Entity>,
}

impl Housing {
    pub const CAPACITY: usize = 5;

    pub fn resident_count(&self) -> usize {
        self.residents.len()
    }

    pub fn has_space(&self) -> bool {
        self.resident_count() < Self::CAPACITY
    }

    pub fn add_resident(&mut self, resident: Entity) -> bool {
        if self.residents.contains(&resident) || !self.has_space() {
            return false;
        }

        self.residents.push(resident);
        true
    }

    pub fn remove_resident(&mut self, resident: Entity) {
        self.residents.retain(|entity| *entity != resident);
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BuildingEntrance {
    pub world_position: Vec3,
    pub local_offset: Vec3,
}

#[derive(Component, Debug)]
pub struct EntranceMarker {
    pub owner: Entity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blueprint_waits_for_materials_before_completion() {
        let mut blueprint = Blueprint {
            kind: ConstructionKind::Building(BuildingKind::House),
            required_wood: 4,
            delivered_wood: 3,
            progress: 99.0,
            build_seconds: 5.0,
        };

        assert!(!blueprint.is_complete());
        blueprint.delivered_wood = 4;
        assert!(blueprint.is_complete());
    }

    #[test]
    fn blueprint_status_tracks_materials_and_work() {
        let mut blueprint = Blueprint {
            kind: ConstructionKind::Building(BuildingKind::House),
            required_wood: 4,
            delivered_wood: 0,
            progress: 0.0,
            build_seconds: 5.0,
        };

        assert_eq!(blueprint.status(), BlueprintStatus::WaitingForMaterials);
        blueprint.delivered_wood = 4;
        assert_eq!(blueprint.status(), BlueprintStatus::WaitingForBuilder);
        blueprint.progress = 2.0;
        assert_eq!(blueprint.status(), BlueprintStatus::Building);
        blueprint.progress = 5.0;
        assert_eq!(blueprint.status(), BlueprintStatus::Complete);
    }

    #[test]
    fn housing_capacity_is_five_residents() {
        let mut housing = Housing::default();

        for index in 0..Housing::CAPACITY {
            assert!(housing.add_resident(Entity::from_raw_u32(index as u32).unwrap()));
        }

        assert!(!housing.has_space());
        assert!(!housing.add_resident(Entity::from_raw_u32(99).unwrap()));
        assert_eq!(housing.resident_count(), 5);
    }
}
