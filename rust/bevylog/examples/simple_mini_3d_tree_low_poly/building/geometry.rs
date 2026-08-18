//! `WorldGeometry`：记录所有已占用的多边形与保留的入口点，
//! 提供「能否放置 / 能否行走」查询；占用变化时按多边形包围盒标记导航脏区，
//! 供 `navigation` 增量重建。这是放置合法性与寻路之间的桥梁。

use bevy::prelude::*;

use crate::math::{within_world_bounds, xz};
use crate::terrain::max_slope_in_polygon;

use super::PlacementIssue;
#[cfg(test)]
use super::polygon::{cell_center_2d, cell_polygon};
use super::polygon::{distance_to_polygon, point_in_polygon, polygons_intersect};
#[cfg(test)]
use crate::types::within_map;

const ENTRANCE_RESERVATION_RADIUS: f32 = 0.35;

#[derive(Clone, Debug)]
pub struct Obstacle {
    pub entity: Entity,
    pub polygon: Vec<Vec2>,
    pub passable: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ReservedEntrance {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavigationDirtyArea {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Resource, Debug, Default)]
pub struct WorldGeometry {
    obstacles: Vec<Obstacle>,
    reserved_entrances: Vec<ReservedEntrance>,
    navigation_dirty_areas: Vec<NavigationDirtyArea>,
    revision: u64,
}

#[cfg(test)]
pub type MapGrid = WorldGeometry;

impl WorldGeometry {
    #[cfg(test)]
    pub fn is_area_free(&self, cells: &[IVec2]) -> bool {
        self.placement_issue(cells).is_none()
    }

    #[cfg(test)]
    pub fn placement_issue(&self, cells: &[IVec2]) -> Option<PlacementIssue> {
        self.placement_issue_for(cells, None, true)
    }

    #[cfg(test)]
    pub fn placement_issue_for(
        &self,
        cells: &[IVec2],
        entrance: Option<IVec2>,
        block_reserved_entrances: bool,
    ) -> Option<PlacementIssue> {
        self.placement_issue_for_with_slope(cells, entrance, block_reserved_entrances, 0, 0.58)
    }

    #[cfg(test)]
    pub fn placement_issue_for_with_slope(
        &self,
        cells: &[IVec2],
        entrance: Option<IVec2>,
        block_reserved_entrances: bool,
        _seed: u64,
        _max_slope: f32,
    ) -> Option<PlacementIssue> {
        if cells.iter().any(|cell| !within_map(*cell)) {
            return Some(PlacementIssue::OutOfBounds);
        }
        if cells.iter().any(|cell| {
            let polygon = cell_polygon(*cell);
            self.obstacles
                .iter()
                .any(|obstacle| polygons_intersect(&polygon, &obstacle.polygon))
        }) {
            return Some(PlacementIssue::Occupied);
        }
        if block_reserved_entrances
            && cells.iter().any(|cell| {
                self.reserved_entrances
                    .iter()
                    .any(|reserved| reserved.position == cell_center_2d(*cell))
            })
        {
            return Some(PlacementIssue::Occupied);
        }
        if let Some(entrance) = entrance {
            if !within_map(entrance) {
                return Some(PlacementIssue::OutOfBounds);
            }
            let entrance_position = cell_center_2d(entrance);
            if !self.is_walkable(entrance)
                || self.reserved_entrances.iter().any(|reserved| {
                    reserved.position.distance(entrance_position) <= ENTRANCE_RESERVATION_RADIUS
                })
            {
                return Some(PlacementIssue::EntranceBlocked);
            }
        }

        None
    }

    // 放置合法性检查，按优先级依次判定：越界 → 与已有占用重叠 → 占用保留的入口
    // → 入口位置可走且未被占 → 多边形内地势过陡。返回第一个不满足的原因。
    pub fn placement_issue_for_polygon(
        &self,
        polygon: &[Vec2],
        entrance: Option<Vec3>,
        block_reserved_entrances: bool,
        seed: u64,
        max_slope: f32,
    ) -> Option<PlacementIssue> {
        if polygon.iter().any(|point| !within_world_bounds(*point)) {
            return Some(PlacementIssue::OutOfBounds);
        }
        if self
            .obstacles
            .iter()
            .any(|obstacle| polygons_intersect(polygon, &obstacle.polygon))
        {
            return Some(PlacementIssue::Occupied);
        }
        if block_reserved_entrances
            && self.reserved_entrances.iter().any(|reserved| {
                point_in_polygon(reserved.position, polygon)
                    || distance_to_polygon(reserved.position, polygon)
                        <= ENTRANCE_RESERVATION_RADIUS
            })
        {
            return Some(PlacementIssue::Occupied);
        }

        if let Some(entrance) = entrance {
            let entrance = xz(entrance);
            if !within_world_bounds(entrance) {
                return Some(PlacementIssue::OutOfBounds);
            }
            if !self.is_walkable_point_2d(entrance)
                || self.reserved_entrances.iter().any(|reserved| {
                    reserved.position.distance(entrance) <= ENTRANCE_RESERVATION_RADIUS
                })
            {
                return Some(PlacementIssue::EntranceBlocked);
            }
        }

        let slope = max_slope_in_polygon(seed, polygon, 8);
        if slope > max_slope {
            return Some(PlacementIssue::TooSteep);
        }

        None
    }

    #[cfg(test)]
    pub fn occupy(&mut self, cells: &[IVec2], entity: Entity, passable: bool) {
        for cell in cells {
            self.occupy_polygon(cell_polygon(*cell), entity, passable);
        }
    }

    pub fn occupy_polygon(&mut self, polygon: Vec<Vec2>, entity: Entity, passable: bool) {
        if !passable {
            self.mark_navigation_dirty(polygon_bounds(&polygon));
        }
        self.obstacles.push(Obstacle {
            entity,
            polygon,
            passable,
        });
        self.bump_revision();
    }

    #[cfg(test)]
    pub fn reserve_entrance(&mut self, cell: IVec2, entity: Entity) {
        self.reserve_entrance_point_2d(cell_center_2d(cell), entity);
    }

    pub fn reserve_entrance_point(&mut self, position: Vec3, entity: Entity) {
        self.reserve_entrance_point_2d(xz(position), entity);
    }

    pub fn reserve_entrance_point_2d(&mut self, position: Vec2, entity: Entity) {
        self.reserved_entrances
            .push(ReservedEntrance { entity, position });
        self.bump_revision();
    }

    pub fn release_entity(&mut self, entity: Entity) {
        let obstacle_count = self.obstacles.len();
        let reserved_count = self.reserved_entrances.len();
        let released_areas: Vec<_> = self
            .obstacles
            .iter()
            .filter(|obstacle| obstacle.entity == entity && !obstacle.passable)
            .map(|obstacle| polygon_bounds(&obstacle.polygon))
            .collect();
        self.obstacles.retain(|obstacle| obstacle.entity != entity);
        self.reserved_entrances
            .retain(|reserved| reserved.entity != entity);
        for area in released_areas {
            self.mark_navigation_dirty(area);
        }
        if self.obstacles.len() != obstacle_count || self.reserved_entrances.len() != reserved_count
        {
            self.bump_revision();
        }
    }

    #[cfg(test)]
    pub fn is_walkable(&self, cell: IVec2) -> bool {
        within_map(cell) && self.is_walkable_point_2d(cell_center_2d(cell))
    }

    #[cfg(test)]
    pub fn is_walkable_point(&self, point: Vec3) -> bool {
        self.is_walkable_point_2d(xz(point))
    }

    pub fn is_walkable_point_2d(&self, point: Vec2) -> bool {
        within_world_bounds(point)
            && self
                .obstacles
                .iter()
                .filter(|obstacle| !obstacle.passable)
                .all(|obstacle| !point_in_polygon(point, &obstacle.polygon))
    }

    pub fn obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn take_navigation_dirty_areas(&mut self) -> Vec<NavigationDirtyArea> {
        std::mem::take(&mut self.navigation_dirty_areas)
    }

    pub fn summary(&self) -> (usize, usize, usize) {
        let road_cells = self.obstacles.iter().filter(|cell| cell.passable).count();
        let mut entities = Vec::new();
        for obstacle in &self.obstacles {
            if !entities.contains(&obstacle.entity) {
                entities.push(obstacle.entity);
            }
        }

        (self.obstacles.len(), road_cells, entities.len())
    }

    fn bump_revision(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    fn mark_navigation_dirty(&mut self, area: NavigationDirtyArea) {
        self.navigation_dirty_areas.push(area);
    }
}

fn polygon_bounds(polygon: &[Vec2]) -> NavigationDirtyArea {
    let mut min = Vec2::splat(f32::MAX);
    let mut max = Vec2::splat(f32::MIN);
    for point in polygon {
        min = min.min(*point);
        max = max.max(*point);
    }
    NavigationDirtyArea { min, max }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::building::footprint_polygon;
    use crate::types::{BuildingKind, MAP_BUILD_HALF_EXTENT, MAP_HALF_CELLS};

    #[test]
    fn grid_rejects_occupied_area() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut grid = MapGrid::default();
        let cells = vec![IVec2::new(0, 0), IVec2::new(1, 0)];

        assert!(grid.is_area_free(&cells));
        grid.occupy(&cells, entity, false);
        assert!(!grid.is_area_free(&cells));
        assert_eq!(grid.summary().0, 2);
    }

    #[test]
    fn world_bounds_use_expanded_map_extent() {
        let geometry = WorldGeometry::default();

        assert!(within_map(IVec2::new(MAP_HALF_CELLS, MAP_HALF_CELLS)));
        assert!(!within_map(IVec2::new(MAP_HALF_CELLS + 1, 0)));
        assert!(geometry.is_walkable_point(Vec3::new(MAP_BUILD_HALF_EXTENT, 0.0, 0.0)));
        assert!(!geometry.is_walkable_point(Vec3::new(MAP_BUILD_HALF_EXTENT + 0.01, 0.0, 0.0)));
    }

    #[test]
    fn placement_issue_prefers_map_bounds_before_occupancy() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut grid = MapGrid::default();
        grid.occupy(&[IVec2::new(0, 0)], entity, false);

        assert_eq!(
            grid.placement_issue(&[IVec2::new(0, 0)]),
            Some(PlacementIssue::Occupied)
        );
        assert_eq!(
            grid.placement_issue(&[IVec2::new(0, 0), IVec2::new(999, 999)]),
            Some(PlacementIssue::OutOfBounds)
        );
    }

    #[test]
    fn grid_releases_entity_occupancy() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut grid = MapGrid::default();
        let cell = IVec2::new(2, 2);

        grid.occupy(&[cell], entity, false);
        assert!(!grid.is_walkable(cell));

        grid.release_entity(entity);
        assert!(grid.is_walkable(cell));
    }

    #[test]
    fn road_cells_are_walkable() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut grid = MapGrid::default();
        let cell = IVec2::new(2, 2);

        grid.occupy(&[cell], entity, true);

        assert!(grid.is_walkable(cell));
    }

    #[test]
    fn reserved_entrance_blocks_non_road_footprint() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut grid = MapGrid::default();
        let cell = IVec2::new(2, 2);

        grid.reserve_entrance(cell, entity);

        assert_eq!(
            grid.placement_issue_for(&[cell], None, true),
            Some(PlacementIssue::Occupied)
        );
        assert_eq!(grid.placement_issue_for(&[cell], None, false), None);
    }

    #[test]
    fn placement_rejects_blocked_entrance() {
        let mut world = World::new();
        let building = world.spawn_empty().id();
        let blocker = world.spawn_empty().id();
        let mut grid = MapGrid::default();

        grid.occupy(&[IVec2::new(0, -1)], blocker, false);

        assert_eq!(
            grid.placement_issue_for(&[IVec2::ZERO], Some(IVec2::new(0, -1)), true),
            Some(PlacementIssue::EntranceBlocked)
        );

        grid.release_entity(blocker);
        grid.reserve_entrance(IVec2::new(0, -1), building);
        assert_eq!(
            grid.placement_issue_for(&[IVec2::ZERO], Some(IVec2::new(0, -1)), true),
            Some(PlacementIssue::EntranceBlocked)
        );
    }

    #[test]
    fn polygon_placement_rejects_overlapping_rotated_footprint() {
        let mut world = World::new();
        let entity = world.spawn_empty().id();
        let mut geometry = WorldGeometry::default();
        let definition = BuildingKind::Storage.definition();
        let first = footprint_polygon(BuildingKind::Storage, Vec3::ZERO, definition.size, 0.37);
        let overlapping = footprint_polygon(
            BuildingKind::House,
            Vec3::new(0.4, 0.0, 0.2),
            BuildingKind::House.definition().size,
            -0.2,
        );

        geometry.occupy_polygon(first, entity, false);

        assert_eq!(
            geometry.placement_issue_for_polygon(&overlapping, None, true, 0, 0.58),
            Some(PlacementIssue::Occupied)
        );
    }
}
