//! 游戏数据定义：资源 / 建筑 / 施工种类枚举、建筑属性表（尺寸、造价、工期、人口容量）、
//! 网格与世界坐标换算、建筑入口位置计算。整个游戏世界的“规则数据”都集中在这里。

use bevy::prelude::*;

pub const CELL_SIZE: f32 = 1.0;
pub const MAP_HALF_CELLS: i32 = 216;
pub const MAP_GRID_CELLS: u32 = (MAP_HALF_CELLS * 2) as u32;
pub const MAP_BUILD_HALF_EXTENT: f32 = (MAP_HALF_CELLS as f32 + 0.5) * CELL_SIZE;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceKind {
    Wood,
    Food,
    Firewood,
}

impl ResourceKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Wood => "木材",
            Self::Food => "食物",
            Self::Firewood => "柴火",
        }
    }

    pub fn unit_size(self) -> i32 {
        match self {
            Self::Wood => 10,
            Self::Food => 1,
            Self::Firewood => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BuildingKind {
    House,
    Storage,
    Woodcutter,
    Gatherer,
    ChoppingYard,
    Road,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ConstructionKind {
    Building(BuildingKind),
    Farm,
}

pub const CONSTRUCTION_KINDS: [ConstructionKind; 7] = [
    ConstructionKind::Building(BuildingKind::House),
    ConstructionKind::Building(BuildingKind::Storage),
    ConstructionKind::Building(BuildingKind::Woodcutter),
    ConstructionKind::Building(BuildingKind::Gatherer),
    ConstructionKind::Building(BuildingKind::Road),
    ConstructionKind::Farm,
    ConstructionKind::Building(BuildingKind::ChoppingYard),
];

#[derive(Clone, Copy, Debug)]
pub struct BuildingDefinition {
    pub label: &'static str,
    pub size: IVec2,
    pub wood_cost: i32,
    pub build_seconds: f32,
    pub work_seconds: f32,
    pub height: f32,
    pub population_capacity: i32,
}

impl BuildingKind {
    pub fn definition(self) -> BuildingDefinition {
        match self {
            Self::House => BuildingDefinition {
                label: "房屋",
                size: IVec2::new(2, 2),
                wood_cost: 10,
                build_seconds: 5.0,
                work_seconds: 0.0,
                height: 1.1,
                population_capacity: 5,
            },
            Self::Storage => BuildingDefinition {
                label: "仓库",
                size: IVec2::new(3, 2),
                wood_cost: 12,
                build_seconds: 6.0,
                work_seconds: 0.0,
                height: 0.9,
                population_capacity: 0,
            },
            Self::Woodcutter => BuildingDefinition {
                label: "伐木屋",
                size: IVec2::new(2, 2),
                wood_cost: 8,
                build_seconds: 4.0,
                work_seconds: 5.0,
                height: 1.0,
                population_capacity: 0,
            },
            Self::Gatherer => BuildingDefinition {
                label: "采集屋",
                size: IVec2::new(2, 2),
                wood_cost: 8,
                build_seconds: 4.0,
                work_seconds: 5.0,
                height: 1.0,
                population_capacity: 0,
            },
            Self::ChoppingYard => BuildingDefinition {
                label: "劈柴场",
                size: IVec2::new(2, 2),
                wood_cost: 10,
                build_seconds: 4.0,
                work_seconds: 8.0,
                height: 0.8,
                population_capacity: 0,
            },
            Self::Road => BuildingDefinition {
                label: "道路",
                size: IVec2::new(1, 1),
                wood_cost: 1,
                build_seconds: 0.6,
                work_seconds: 0.0,
                height: 0.05,
                population_capacity: 0,
            },
        }
    }

    pub fn hotkey(self) -> KeyCode {
        match self {
            Self::House => KeyCode::Digit1,
            Self::Storage => KeyCode::Digit2,
            Self::Woodcutter => KeyCode::Digit3,
            Self::Gatherer => KeyCode::Digit4,
            Self::ChoppingYard => KeyCode::Digit7,
            Self::Road => KeyCode::Digit5,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::House => "为定居者提供住房。",
            Self::Storage => "接收采集的物资与建造材料。",
            Self::Woodcutter => "提供伐木工岗位，从树木采集木材。",
            Self::Gatherer => "提供采集工岗位，从浆果丛采集食物。",
            Self::ChoppingYard => "有人手时把木材劈成柴火。",
            Self::Road => "标记聚落中的规划路径。",
        }
    }

    pub fn entrance_direction(self) -> Option<IVec2> {
        match self {
            Self::Road => None,
            _ => Some(IVec2::NEG_Y),
        }
    }
}

impl ConstructionKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Building(kind) => kind.definition().label,
            Self::Farm => "农场",
        }
    }

    pub fn hotkey(self) -> KeyCode {
        match self {
            Self::Building(kind) => kind.hotkey(),
            Self::Farm => KeyCode::Digit6,
        }
    }

    pub fn hotkey_label(self) -> &'static str {
        match self {
            Self::Building(BuildingKind::House) => "1",
            Self::Building(BuildingKind::Storage) => "2",
            Self::Building(BuildingKind::Woodcutter) => "3",
            Self::Building(BuildingKind::Gatherer) => "4",
            Self::Building(BuildingKind::Road) => "5",
            Self::Farm => "6",
            Self::Building(BuildingKind::ChoppingYard) => "7",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Building(kind) => kind.description(),
            Self::Farm => "圈定一块已备好的耕地，供日后种植。",
        }
    }

    pub fn as_building(self) -> Option<BuildingKind> {
        match self {
            Self::Building(kind) => Some(kind),
            Self::Farm => None,
        }
    }
}

pub fn building_color(kind: BuildingKind) -> Color {
    match kind {
        BuildingKind::House => Color::srgb(0.74, 0.38, 0.24),
        BuildingKind::Storage => Color::srgb(0.72, 0.58, 0.34),
        BuildingKind::Woodcutter => Color::srgb(0.35, 0.55, 0.23),
        BuildingKind::Gatherer => Color::srgb(0.45, 0.38, 0.68),
        BuildingKind::ChoppingYard => Color::srgb(0.64, 0.42, 0.22),
        BuildingKind::Road => Color::srgb(0.22, 0.2, 0.18),
    }
}

pub fn world_to_cell(position: Vec3) -> IVec2 {
    IVec2::new(
        (position.x / CELL_SIZE).round() as i32,
        (position.z / CELL_SIZE).round() as i32,
    )
}

pub fn cell_to_world(cell: IVec2) -> Vec3 {
    Vec3::new(cell.x as f32 * CELL_SIZE, 0.0, cell.y as f32 * CELL_SIZE)
}

pub fn snap_to_grid(position: Vec3) -> Vec3 {
    cell_to_world(world_to_cell(position))
}

#[cfg(test)]
pub fn rotated_size(size: IVec2, rotation_steps: i32) -> IVec2 {
    if rotation_steps.rem_euclid(2) == 0 {
        size
    } else {
        IVec2::new(size.y, size.x)
    }
}

#[cfg(test)]
pub fn footprint_cells(center: IVec2, size: IVec2) -> Vec<IVec2> {
    let start = center - IVec2::new((size.x - 1) / 2, (size.y - 1) / 2);
    let mut cells = Vec::with_capacity((size.x * size.y) as usize);

    for x in 0..size.x {
        for y in 0..size.y {
            cells.push(start + IVec2::new(x, y));
        }
    }

    cells
}

#[cfg(test)]
pub fn rotated_direction(direction: IVec2, rotation_steps: i32) -> IVec2 {
    match rotation_steps.rem_euclid(4) {
        0 => direction,
        1 => IVec2::new(direction.y, -direction.x),
        2 => -direction,
        _ => IVec2::new(-direction.y, direction.x),
    }
}

#[cfg(test)]
pub fn entrance_cell(center: IVec2, size: IVec2, rotation_steps: i32, direction: IVec2) -> IVec2 {
    let size = rotated_size(size, rotation_steps);
    let direction = rotated_direction(direction, rotation_steps);
    let cells = footprint_cells(center, size);
    let min_x = cells.iter().map(|cell| cell.x).min().unwrap_or(center.x);
    let max_x = cells.iter().map(|cell| cell.x).max().unwrap_or(center.x);
    let min_y = cells.iter().map(|cell| cell.y).min().unwrap_or(center.y);
    let max_y = cells.iter().map(|cell| cell.y).max().unwrap_or(center.y);

    match (direction.x.signum(), direction.y.signum()) {
        (-1, _) => IVec2::new(min_x - 1, center.y),
        (1, _) => IVec2::new(max_x + 1, center.y),
        (_, -1) => IVec2::new(center.x, min_y - 1),
        (_, 1) => IVec2::new(center.x, max_y + 1),
        _ => center,
    }
}

pub fn entrance_world_position(center: Vec3, size: IVec2, angle: f32, direction: IVec2) -> Vec3 {
    center + Quat::from_rotation_y(angle) * entrance_local_offset(size, direction)
}

pub fn entrance_local_offset(size: IVec2, direction: IVec2) -> Vec3 {
    let hx = size.x as f32 * CELL_SIZE * 0.5;
    let hz = size.y as f32 * CELL_SIZE * 0.5;

    match (direction.x.signum(), direction.y.signum()) {
        (-1, _) => Vec3::new(-hx, 0.0, 0.0),
        (1, _) => Vec3::new(hx, 0.0, 0.0),
        (_, -1) => Vec3::new(0.0, 0.0, -hz),
        (_, 1) => Vec3::new(0.0, 0.0, hz),
        _ => Vec3::ZERO,
    }
}

#[cfg(test)]
pub fn within_map(cell: IVec2) -> bool {
    cell.x >= -MAP_HALF_CELLS
        && cell.x <= MAP_HALF_CELLS
        && cell.y >= -MAP_HALF_CELLS
        && cell.y <= MAP_HALF_CELLS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footprint_matches_building_area() {
        let cells = footprint_cells(IVec2::ZERO, IVec2::new(3, 2));
        assert_eq!(cells.len(), 6);
        assert!(cells.contains(&IVec2::new(-1, 0)));
        assert!(cells.contains(&IVec2::new(1, 1)));
    }

    #[test]
    fn rotation_swaps_rectangular_size() {
        assert_eq!(rotated_size(IVec2::new(3, 2), 1), IVec2::new(2, 3));
        assert_eq!(rotated_size(IVec2::new(3, 2), 2), IVec2::new(3, 2));
    }

    #[test]
    fn entrance_cell_tracks_rotation() {
        let center = IVec2::ZERO;
        let size = IVec2::new(3, 2);
        let direction = IVec2::NEG_Y;

        assert_eq!(entrance_cell(center, size, 0, direction), IVec2::new(0, -1));
        assert_eq!(entrance_cell(center, size, 1, direction), IVec2::new(-1, 0));
        assert_eq!(entrance_cell(center, size, 2, direction), IVec2::new(0, 2));
        assert_eq!(entrance_cell(center, size, 3, direction), IVec2::new(2, 0));
    }

    #[test]
    fn entrance_local_offset_uses_unrotated_building_space() {
        let size = IVec2::new(3, 2);

        assert_eq!(
            entrance_local_offset(size, IVec2::NEG_Y),
            Vec3::new(0.0, 0.0, -1.0)
        );
        assert_eq!(
            entrance_local_offset(size, IVec2::X),
            Vec3::new(1.5, 0.0, 0.0)
        );
    }

    #[test]
    fn entrance_world_position_tracks_bevy_rotation() {
        let center = Vec3::new(10.0, 0.0, 20.0);
        let size = IVec2::new(3, 2);
        let direction = IVec2::NEG_Y;

        assert_vec3_approx_eq(
            entrance_world_position(center, size, 0.0, direction),
            Vec3::new(10.0, 0.0, 19.0),
        );
        assert_vec3_approx_eq(
            entrance_world_position(center, size, std::f32::consts::FRAC_PI_2, direction),
            Vec3::new(9.0, 0.0, 20.0),
        );
        assert_vec3_approx_eq(
            entrance_world_position(center, size, std::f32::consts::PI, direction),
            Vec3::new(10.0, 0.0, 21.0),
        );
        assert_vec3_approx_eq(
            entrance_world_position(center, size, std::f32::consts::FRAC_PI_2 * 3.0, direction),
            Vec3::new(11.0, 0.0, 20.0),
        );

        let angle = 0.37;
        assert_vec3_approx_eq(
            entrance_world_position(center, size, angle, direction),
            center + Vec3::new(-angle.sin(), 0.0, -angle.cos()),
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
