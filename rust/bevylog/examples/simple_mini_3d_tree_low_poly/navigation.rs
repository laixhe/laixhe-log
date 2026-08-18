//! 寻路系统：把世界划分为 `NAV_CELL_SIZE`（0.5 单位）的网格，再按 32×32 分块（chunk）管理。
//! - 增量重建：建筑/道路变化只在“脏区”对应的 chunk 内重算，避免整图重建。
//! - 双向 A*：先求两端 chunk 的走廊，再在走廊网格里跑 A*，控制搜索规模。
//! - 异步规划：路径通过 `AsyncComputeTaskPool` 在后台线程计算，`poll_once` 轮询结果，
//!   并缓存最近 4096 条路径；世界变化后自动判定重规划。
//! - 连通分量（component）：预计算后用来快速判断两点是否可能连通（剪枝）。

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    sync::Arc,
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, poll_once},
};

use crate::{
    building::{NavigationDirtyArea, WorldGeometry, expanded_polygon, point_in_polygon},
    math::{within_world_bounds, xz},
    terrain::{TerrainSeed, terrain_height},
    types::MAP_BUILD_HALF_EXTENT,
};

pub const NAV_CELL_SIZE: f32 = 0.5;
pub const NAV_CHUNK_CELLS: i32 = 32;

const AGENT_RADIUS: f32 = 0.08;
const MAX_DIRTY_CHUNKS_PER_TICK: usize = 8;
const MAX_PATH_SUBMISSIONS_PER_TICK: usize = 12;
const MAX_IN_FLIGHT_PATHS: usize = 48;
const PATH_CACHE_LIMIT: usize = 4096;
const CHUNK_CORRIDOR_THRESHOLD: i32 = 4;
const CHUNK_CORRIDOR_PADDING: i32 = 1;
const NEAREST_WALKABLE_RADIUS: i32 = 6;
const SLOPE_COST_FACTOR: f32 = 1.5;
const DIRTY_AREA_PADDING: f32 = AGENT_RADIUS + NAV_CELL_SIZE;
const UNVISITED_COMPONENT: u32 = u32::MAX;

pub type PathRequestId = u64;

#[derive(Resource, Debug)]
pub struct NavGrid {
    width: i32,
    height: i32,
    chunks_x: i32,
    chunks_y: i32,
    half_extent: f32,
    cells: Arc<Vec<u8>>,
    components: Arc<Vec<u32>>,
    chunk_walkable: Arc<Vec<u8>>,
    dirty_chunks: Vec<bool>,
    dirty_queue: VecDeque<usize>,
    dirty_set: HashSet<usize>,
    revision: u64,
    chunk_revision: Vec<u64>,
    geometry_revision: Option<u64>,
    initialized: bool,
}

impl Default for NavGrid {
    fn default() -> Self {
        let width = ((MAP_BUILD_HALF_EXTENT * 2.0) / NAV_CELL_SIZE).ceil() as i32;
        let height = width;
        let chunks_x = div_ceil_i32(width, NAV_CHUNK_CELLS);
        let chunks_y = div_ceil_i32(height, NAV_CHUNK_CELLS);
        let cell_count = (width * height) as usize;
        let chunk_count = (chunks_x * chunks_y) as usize;

        Self {
            width,
            height,
            chunks_x,
            chunks_y,
            half_extent: MAP_BUILD_HALF_EXTENT,
            cells: Arc::new(vec![1; cell_count]),
            components: Arc::new(vec![UNVISITED_COMPONENT; cell_count]),
            chunk_walkable: Arc::new(vec![1; chunk_count]),
            dirty_chunks: vec![false; chunk_count],
            dirty_queue: VecDeque::new(),
            dirty_set: HashSet::new(),
            revision: 0,
            chunk_revision: vec![0; chunk_count],
            geometry_revision: None,
            initialized: false,
        }
    }
}

impl NavGrid {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn path_needs_replan(&self, path: &[Vec3], stored_revision: u64) -> bool {
        if stored_revision == self.revision {
            return false;
        }
        for point in path {
            if let Some(cell) = self.cell_at_world(Vec2::new(point.x, point.z)) {
                if let Some(chunk) = self.chunk_index_for_cell(cell) {
                    if self.chunk_revision[chunk] > stored_revision {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[cfg(test)]
    pub fn sync_for_test(&mut self, geometry: &mut WorldGeometry, seed: u64) {
        self.sync_from_geometry(geometry, seed);
    }

    #[cfg(test)]
    pub fn pending_dirty_chunks(&self) -> usize {
        self.dirty_set.len()
    }

    pub fn snapshot(&self) -> NavGridSnapshot {
        NavGridSnapshot {
            width: self.width,
            height: self.height,
            chunks_x: self.chunks_x,
            chunks_y: self.chunks_y,
            half_extent: self.half_extent,
            cells: self.cells.clone(),
            components: self.components.clone(),
            chunk_walkable: self.chunk_walkable.clone(),
            dirty_chunks: Arc::new(self.dirty_chunks.iter().map(|dirty| *dirty as u8).collect()),
            revision: self.revision,
        }
    }

    pub fn endpoint_chunks_clean(&self, start: Vec3, target: Vec3) -> bool {
        self.cell_at_world(xz(start))
            .and_then(|cell| self.chunk_index_for_cell(cell))
            .is_some_and(|chunk| !self.dirty_chunks[chunk])
            && self
                .cell_at_world(xz(target))
                .and_then(|cell| self.chunk_index_for_cell(cell))
                .is_some_and(|chunk| !self.dirty_chunks[chunk])
    }

    pub fn maybe_reachable(&self, start: Vec3, target: Vec3) -> bool {
        let snapshot = self.snapshot();
        let Some(start_cell) = snapshot.nearest_walkable_cell(xz(start)) else {
            return false;
        };
        let Some(target_cell) = snapshot.nearest_walkable_cell(xz(target)) else {
            return false;
        };
        let start_component = snapshot.component_at(start_cell);
        let target_component = snapshot.component_at(target_cell);
        start_component != UNVISITED_COMPONENT && start_component == target_component
    }

    fn sync_from_geometry(&mut self, geometry: &mut WorldGeometry, seed: u64) {
        if !self.initialized {
            self.rebuild_all(geometry, seed);
            geometry.take_navigation_dirty_areas();
            self.geometry_revision = Some(geometry.revision());
            self.initialized = true;
            self.revision = self.revision.wrapping_add(1);
            return;
        }

        let dirty_areas = geometry.take_navigation_dirty_areas();
        if self.geometry_revision != Some(geometry.revision()) {
            for area in dirty_areas {
                self.mark_dirty_area(area);
            }
            self.geometry_revision = Some(geometry.revision());
        }

        let mut rebuilt = 0usize;
        while rebuilt < MAX_DIRTY_CHUNKS_PER_TICK {
            let Some(chunk) = self.dirty_queue.pop_front() else {
                break;
            };
            if !self.dirty_set.remove(&chunk) {
                continue;
            }
            self.rebuild_chunk(chunk, geometry, seed);
            self.dirty_chunks[chunk] = false;
            // Record the revision this chunk will have after the batch increment below
            self.chunk_revision[chunk] = self.revision.wrapping_add(1);
            rebuilt += 1;
        }

        if rebuilt > 0 {
            self.rebuild_components();
            self.revision = self.revision.wrapping_add(1);
        }
    }

    fn rebuild_all(&mut self, geometry: &WorldGeometry, seed: u64) {
        for chunk in 0..(self.chunks_x * self.chunks_y) as usize {
            self.rebuild_chunk(chunk, geometry, seed);
        }
        self.dirty_chunks.fill(false);
        self.dirty_queue.clear();
        self.dirty_set.clear();
        self.rebuild_components();
    }

    // 重建单个 chunk：把该块内的不可走障碍多边形（膨胀了角色半径）落到网格上，
    // 一个格子在所有障碍外且在世界范围内才算可走。
    fn rebuild_chunk(&mut self, chunk: usize, geometry: &WorldGeometry, _seed: u64) {
        let chunk_x = chunk as i32 % self.chunks_x;
        let chunk_y = chunk as i32 / self.chunks_x;
        let min_cell = IVec2::new(chunk_x * NAV_CHUNK_CELLS, chunk_y * NAV_CHUNK_CELLS);
        let max_cell = IVec2::new(
            ((chunk_x + 1) * NAV_CHUNK_CELLS).min(self.width),
            ((chunk_y + 1) * NAV_CHUNK_CELLS).min(self.height),
        );
        let chunk_bounds = self.chunk_world_bounds(min_cell, max_cell);
        let blockers: Vec<_> = geometry
            .obstacles()
            .iter()
            .filter(|obstacle| !obstacle.passable)
            .filter_map(|obstacle| {
                let expanded = expanded_polygon(&obstacle.polygon, AGENT_RADIUS);
                let bounds = polygon_bounds(&expanded);
                bounds_intersect(bounds, chunk_bounds).then_some(expanded)
            })
            .collect();

        let width = self.width;
        let half_extent = self.half_extent;
        let cells = Arc::make_mut(&mut self.cells);
        let mut chunk_has_walkable = false;
        for y in min_cell.y..max_cell.y {
            for x in min_cell.x..max_cell.x {
                let index = (y * width + x) as usize;
                let point = Vec2::new(
                    -half_extent + (x as f32 + 0.5) * NAV_CELL_SIZE,
                    -half_extent + (y as f32 + 0.5) * NAV_CELL_SIZE,
                );
                let walkable = within_world_bounds(point)
                    && blockers
                        .iter()
                        .all(|polygon| !point_in_polygon(point, polygon));
                cells[index] = walkable as u8;
                chunk_has_walkable |= walkable;
            }
        }
        Arc::make_mut(&mut self.chunk_walkable)[chunk] = chunk_has_walkable as u8;
    }

    fn rebuild_components(&mut self) {
        let mut components = vec![UNVISITED_COMPONENT; (self.width * self.height) as usize];
        let mut current_component = 0u32;
        let mut queue = VecDeque::new();

        for index in 0..components.len() {
            if self.cells[index] == 0 || components[index] != UNVISITED_COMPONENT {
                continue;
            }

            components[index] = current_component;
            queue.push_back(index);
            while let Some(current) = queue.pop_front() {
                let cell = self.cell_from_index(current);
                for neighbor in cardinal_neighbors(cell) {
                    if !self.in_bounds_cell(neighbor) {
                        continue;
                    }
                    let neighbor_index = self.cell_index(neighbor);
                    if self.cells[neighbor_index] == 0
                        || components[neighbor_index] != UNVISITED_COMPONENT
                    {
                        continue;
                    }
                    components[neighbor_index] = current_component;
                    queue.push_back(neighbor_index);
                }
            }
            current_component = current_component.wrapping_add(1);
        }

        self.components = Arc::new(components);
    }

    fn mark_dirty_area(&mut self, area: NavigationDirtyArea) {
        let min = area.min - Vec2::splat(DIRTY_AREA_PADDING);
        let max = area.max + Vec2::splat(DIRTY_AREA_PADDING);
        let Some(min_cell) = self.cell_at_world_clamped(min) else {
            return;
        };
        let Some(max_cell) = self.cell_at_world_clamped(max) else {
            return;
        };

        let min_chunk = self.cell_to_chunk(min_cell);
        let max_chunk = self.cell_to_chunk(max_cell);
        for y in min_chunk.y..=max_chunk.y {
            for x in min_chunk.x..=max_chunk.x {
                if let Some(chunk) = self.chunk_index(IVec2::new(x, y)) {
                    if self.dirty_set.insert(chunk) {
                        self.dirty_queue.push_back(chunk);
                        self.dirty_chunks[chunk] = true;
                    }
                }
            }
        }
    }

    fn chunk_world_bounds(&self, min_cell: IVec2, max_cell: IVec2) -> NavigationDirtyArea {
        let min = Vec2::new(
            -self.half_extent + min_cell.x as f32 * NAV_CELL_SIZE - DIRTY_AREA_PADDING,
            -self.half_extent + min_cell.y as f32 * NAV_CELL_SIZE - DIRTY_AREA_PADDING,
        );
        let max = Vec2::new(
            -self.half_extent + max_cell.x as f32 * NAV_CELL_SIZE + DIRTY_AREA_PADDING,
            -self.half_extent + max_cell.y as f32 * NAV_CELL_SIZE + DIRTY_AREA_PADDING,
        );
        NavigationDirtyArea { min, max }
    }

    fn cell_at_world(&self, point: Vec2) -> Option<IVec2> {
        if !within_world_bounds(point) {
            return None;
        }
        let x = ((point.x + self.half_extent) / NAV_CELL_SIZE).floor() as i32;
        let y = ((point.y + self.half_extent) / NAV_CELL_SIZE).floor() as i32;
        let cell = IVec2::new(x.clamp(0, self.width - 1), y.clamp(0, self.height - 1));
        self.in_bounds_cell(cell).then_some(cell)
    }

    fn cell_at_world_clamped(&self, point: Vec2) -> Option<IVec2> {
        let clamped = Vec2::new(
            point.x.clamp(-self.half_extent, self.half_extent),
            point.y.clamp(-self.half_extent, self.half_extent),
        );
        self.cell_at_world(clamped)
    }

    fn cell_index(&self, cell: IVec2) -> usize {
        (cell.y * self.width + cell.x) as usize
    }

    fn cell_from_index(&self, index: usize) -> IVec2 {
        IVec2::new(index as i32 % self.width, index as i32 / self.width)
    }

    fn in_bounds_cell(&self, cell: IVec2) -> bool {
        cell.x >= 0 && cell.y >= 0 && cell.x < self.width && cell.y < self.height
    }

    fn cell_to_chunk(&self, cell: IVec2) -> IVec2 {
        IVec2::new(cell.x / NAV_CHUNK_CELLS, cell.y / NAV_CHUNK_CELLS)
    }

    fn chunk_index_for_cell(&self, cell: IVec2) -> Option<usize> {
        self.chunk_index(self.cell_to_chunk(cell))
    }

    fn chunk_index(&self, chunk: IVec2) -> Option<usize> {
        (chunk.x >= 0 && chunk.y >= 0 && chunk.x < self.chunks_x && chunk.y < self.chunks_y)
            .then_some((chunk.y * self.chunks_x + chunk.x) as usize)
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct PathCache;

#[cfg(test)]
pub fn path_to_waypoints(
    geometry: &WorldGeometry,
    _cache: &mut PathCache,
    start: Vec3,
    target: Vec3,
    seed: u64,
) -> Option<Vec<Vec3>> {
    let mut nav_grid = NavGrid::default();
    nav_grid.rebuild_all(geometry, seed);
    nav_grid.revision = 1;
    nav_grid.snapshot().path_to_waypoints(start, target, seed)
}

#[derive(Clone, Debug)]
pub struct NavGridSnapshot {
    width: i32,
    height: i32,
    chunks_x: i32,
    chunks_y: i32,
    half_extent: f32,
    cells: Arc<Vec<u8>>,
    components: Arc<Vec<u32>>,
    chunk_walkable: Arc<Vec<u8>>,
    dirty_chunks: Arc<Vec<u8>>,
    revision: u64,
}

impl NavGridSnapshot {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    // 单次寻路流程：取最近可走格 → 连通分量剪枝（不连通直接放弃）→ A* → 平滑。
    fn path_to_waypoints(&self, start: Vec3, target: Vec3, seed: u64) -> Option<Vec<Vec3>> {
        let start_cell = self.nearest_walkable_cell(xz(start))?;
        let target_cell = self.nearest_walkable_cell(xz(target))?;
        if start_cell == target_cell {
            return Some(Vec::new());
        }
        if self.component_at(start_cell) != self.component_at(target_cell) {
            return None;
        }

        let path = grid_a_star(self, start_cell, target_cell, seed)?;
        let smoothed = smooth_path(self, &path);
        Some(
            smoothed
                .into_iter()
                .skip(1)
                .map(|cell| {
                    if cell == target_cell {
                        Vec3::new(target.x, terrain_height(seed, target.x, target.z), target.z)
                    } else {
                        let point = self.cell_center(cell);
                        Vec3::new(point.x, terrain_height(seed, point.x, point.y), point.y)
                    }
                })
                .collect(),
        )
    }

    // 起终点若恰好落在不可走的格子（比如贴墙站），就向外逐圈搜索最近的可走格。
    fn nearest_walkable_cell(&self, point: Vec2) -> Option<IVec2> {
        let center = self.cell_at_world(point)?;
        if self.is_walkable_cell(center) {
            return Some(center);
        }

        let mut best: Option<(f32, IVec2)> = None;
        for radius in 1..=NEAREST_WALKABLE_RADIUS {
            for y in -radius..=radius {
                for x in -radius..=radius {
                    if x.abs() != radius && y.abs() != radius {
                        continue;
                    }
                    let cell = center + IVec2::new(x, y);
                    if !self.is_walkable_cell(cell) {
                        continue;
                    }
                    let distance = self.cell_center(cell).distance(point);
                    if best.is_none_or(|(best_distance, _)| distance < best_distance) {
                        best = Some((distance, cell));
                    }
                }
            }
            if let Some((_, cell)) = best {
                return Some(cell);
            }
        }

        None
    }

    fn is_walkable_cell(&self, cell: IVec2) -> bool {
        self.in_bounds_cell(cell)
            && self.cells[self.cell_index(cell)] != 0
            && self
                .chunk_index_for_cell(cell)
                .is_some_and(|chunk| self.dirty_chunks[chunk] == 0)
    }

    fn component_at(&self, cell: IVec2) -> u32 {
        if !self.in_bounds_cell(cell) {
            return UNVISITED_COMPONENT;
        }
        self.components[self.cell_index(cell)]
    }

    fn cell_at_world(&self, point: Vec2) -> Option<IVec2> {
        if !within_world_bounds(point) {
            return None;
        }
        let x = ((point.x + self.half_extent) / NAV_CELL_SIZE).floor() as i32;
        let y = ((point.y + self.half_extent) / NAV_CELL_SIZE).floor() as i32;
        let cell = IVec2::new(x.clamp(0, self.width - 1), y.clamp(0, self.height - 1));
        self.in_bounds_cell(cell).then_some(cell)
    }

    fn cell_center(&self, cell: IVec2) -> Vec2 {
        Vec2::new(
            -self.half_extent + (cell.x as f32 + 0.5) * NAV_CELL_SIZE,
            -self.half_extent + (cell.y as f32 + 0.5) * NAV_CELL_SIZE,
        )
    }

    fn cell_index(&self, cell: IVec2) -> usize {
        (cell.y * self.width + cell.x) as usize
    }

    fn in_bounds_cell(&self, cell: IVec2) -> bool {
        cell.x >= 0 && cell.y >= 0 && cell.x < self.width && cell.y < self.height
    }

    fn cell_to_chunk(&self, cell: IVec2) -> IVec2 {
        IVec2::new(cell.x / NAV_CHUNK_CELLS, cell.y / NAV_CHUNK_CELLS)
    }

    fn chunk_index_for_cell(&self, cell: IVec2) -> Option<usize> {
        self.chunk_index(self.cell_to_chunk(cell))
    }

    fn chunk_index(&self, chunk: IVec2) -> Option<usize> {
        (chunk.x >= 0 && chunk.y >= 0 && chunk.x < self.chunks_x && chunk.y < self.chunks_y)
            .then_some((chunk.y * self.chunks_x + chunk.x) as usize)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PathCacheKey {
    revision: u64,
    sx: i32,
    sz: i32,
    tx: i32,
    tz: i32,
}

#[derive(Clone, Debug)]
struct PathRequest {
    id: PathRequestId,
    start: Vec3,
    target: Vec3,
    key: PathCacheKey,
}

#[derive(Clone, Debug)]
pub struct PathResult {
    pub id: PathRequestId,
    pub revision: u64,
    pub path: Option<Vec<Vec3>>,
}

struct PathJob {
    task: Task<PathJobResult>,
}

#[derive(Clone, Debug)]
struct PathJobResult {
    key: PathCacheKey,
    result: PathResult,
}

// 异步路径规划器：路径在后台线程（AsyncComputeTaskPool）计算，避免卡主线程。
// 流程：request_path 入队 → submit_queued 每帧把队列任务投递到线程池（限流）→
// poll_completed 每帧轮询已完成任务并回收结果；结果按 (版本, 起终点) 缓存，
// 世界变化后旧缓存自动失效（版本号不同即重新算）。
#[derive(Resource, Default)]
pub struct PathPlanner {
    next_request_id: PathRequestId,
    queue: VecDeque<PathRequest>,
    in_flight: HashMap<PathRequestId, PathJob>,
    results: HashMap<PathRequestId, PathResult>,
    cache: HashMap<PathCacheKey, Option<Vec<Vec3>>>,
    cache_order: VecDeque<PathCacheKey>,
}

impl PathPlanner {
    // 发起寻路：先查缓存（同版本同起终点直接复用），否则排队等后台线程计算。
    pub fn request_path(&mut self, nav_grid: &NavGrid, start: Vec3, target: Vec3) -> PathRequestId {
        self.next_request_id = self.next_request_id.wrapping_add(1).max(1);
        let id = self.next_request_id;
        let key = path_cache_key(nav_grid.revision(), start, target);

        if let Some(path) = self.cache.get(&key).cloned() {
            self.results.insert(
                id,
                PathResult {
                    id,
                    revision: key.revision,
                    path,
                },
            );
            return id;
        }

        self.queue.push_back(PathRequest {
            id,
            start,
            target,
            key,
        });
        id
    }

    pub fn take_result(&mut self, id: PathRequestId) -> Option<PathResult> {
        self.results.remove(&id)
    }

    #[cfg(test)]
    pub fn insert_result_for_test(&mut self, result: PathResult) {
        self.results.insert(result.id, result);
    }

    // 回收后台任务：poll_once 非阻塞检查；结果与当前世界版本一致才进缓存，
    // 否则丢弃（旧版本算出来的路径不能用）。
    fn poll_completed(&mut self, current_revision: u64) {
        let mut finished = Vec::new();
        for (id, job) in &mut self.in_flight {
            if let Some(result) = block_on(poll_once(&mut job.task)) {
                finished.push((*id, result));
            }
        }

        for (id, job_result) in finished {
            self.in_flight.remove(&id);
            if job_result.result.revision == current_revision {
                self.cache_result(job_result.key, job_result.result.path.clone());
            }
            self.results.insert(job_result.result.id, job_result.result);
        }
    }

    // 投递排队任务：每帧最多提交 MAX_PATH_SUBMISSIONS_PER_TICK 个、同时最多
    // MAX_IN_FLIGHT_PATHS 个在途；起终点所在 chunk 还没重建好的任务先放回队尾。
    fn submit_queued(&mut self, nav_grid: &NavGrid, seed: u64) {
        let pool = AsyncComputeTaskPool::get();
        let mut submitted = 0usize;
        let mut attempts = self.queue.len();

        while submitted < MAX_PATH_SUBMISSIONS_PER_TICK
            && self.in_flight.len() < MAX_IN_FLIGHT_PATHS
            && attempts > 0
        {
            attempts -= 1;
            let Some(mut request) = self.queue.pop_front() else {
                break;
            };

            if !nav_grid.endpoint_chunks_clean(request.start, request.target) {
                self.queue.push_back(request);
                continue;
            }

            request.key = path_cache_key(nav_grid.revision(), request.start, request.target);
            if let Some(path) = self.cache.get(&request.key).cloned() {
                self.results.insert(
                    request.id,
                    PathResult {
                        id: request.id,
                        revision: request.key.revision,
                        path,
                    },
                );
                continue;
            }

            let snapshot = nav_grid.snapshot();
            let key = request.key;
            let task = pool.spawn(async move {
                let path = snapshot.path_to_waypoints(request.start, request.target, seed);
                PathJobResult {
                    key,
                    result: PathResult {
                        id: request.id,
                        revision: snapshot.revision(),
                        path,
                    },
                }
            });
            self.in_flight.insert(request.id, PathJob { task });
            submitted += 1;
        }
    }

    fn cache_result(&mut self, key: PathCacheKey, result: Option<Vec<Vec3>>) {
        if !self.cache.contains_key(&key) {
            self.cache_order.push_back(key);
        }
        self.cache.insert(key, result);
        while self.cache_order.len() > PATH_CACHE_LIMIT {
            if let Some(oldest) = self.cache_order.pop_front() {
                self.cache.remove(&oldest);
            }
        }
    }
}

pub fn sync_nav_grid(
    mut geometry: ResMut<WorldGeometry>,
    terrain_seed: Res<TerrainSeed>,
    mut nav_grid: ResMut<NavGrid>,
) {
    nav_grid.sync_from_geometry(&mut geometry, terrain_seed.0);
}

pub fn poll_path_planner(mut planner: ResMut<PathPlanner>, nav_grid: Res<NavGrid>) {
    planner.poll_completed(nav_grid.revision());
}

pub fn submit_path_planner(
    mut planner: ResMut<PathPlanner>,
    nav_grid: Res<NavGrid>,
    terrain_seed: Res<TerrainSeed>,
) {
    planner.submit_queued(&nav_grid, terrain_seed.0);
}

// A* 主搜索：f(n) = g(n) + h(n)，h 用欧氏距离启发。
// 两个优化：
// 1. chunk 走廊裁剪——只有走廊内的网格才会被展开（见 chunk_corridor），
//    长距离寻路时搜索规模被限制在起终点附近的一小块区域；
// 2. 代价里加入“海拔差 × SLOPE_COST_FACTOR”，让路径倾向走平缓的地形。
fn grid_a_star(
    snapshot: &NavGridSnapshot,
    start: IVec2,
    target: IVec2,
    seed: u64,
) -> Option<Vec<IVec2>> {
    let allowed_chunks = chunk_corridor(snapshot, start, target);
    let mut frontier = BinaryHeap::new();
    let cell_count = (snapshot.width * snapshot.height) as usize;
    let mut came_from = vec![usize::MAX; cell_count];
    let mut costs = vec![f32::MAX; cell_count];
    let start_index = snapshot.cell_index(start);
    let target_index = snapshot.cell_index(target);

    costs[start_index] = 0.0;
    frontier.push(GridNode {
        index: start_index,
        cost: 0.0,
        estimated_total: start.as_vec2().distance(target.as_vec2()),
    });

    while let Some(current) = frontier.pop() {
        if current.index == target_index {
            return Some(reconstruct_cell_path(
                snapshot,
                &came_from,
                start_index,
                target_index,
            ));
        }
        if current.cost > costs[current.index] {
            continue;
        }

        let current_cell = IVec2::new(
            current.index as i32 % snapshot.width,
            current.index as i32 / snapshot.width,
        );
        for (neighbor, step_cost) in grid_neighbors(snapshot, current_cell) {
            if let Some(allowed) = &allowed_chunks
                && !snapshot
                    .chunk_index_for_cell(neighbor)
                    .is_some_and(|chunk| allowed.contains(&chunk))
            {
                continue;
            }

            let neighbor_index = snapshot.cell_index(neighbor);
            let current_point = snapshot.cell_center(current_cell);
            let neighbor_point = snapshot.cell_center(neighbor);
            let h_current = terrain_height(seed, current_point.x, current_point.y);
            let h_next = terrain_height(seed, neighbor_point.x, neighbor_point.y);
            let next_cost =
                current.cost + step_cost + (h_next - h_current).abs() * SLOPE_COST_FACTOR;

            if next_cost < costs[neighbor_index] {
                costs[neighbor_index] = next_cost;
                came_from[neighbor_index] = current.index;
                frontier.push(GridNode {
                    index: neighbor_index,
                    cost: next_cost,
                    estimated_total: next_cost + neighbor.as_vec2().distance(target.as_vec2()),
                });
            }
        }
    }

    None
}

// 8 方向邻居：正交步长 1，斜向步长 √2，天然形成“走斜线比绕直角更短”的代价差异。
fn grid_neighbors(
    snapshot: &NavGridSnapshot,
    cell: IVec2,
) -> impl Iterator<Item = (IVec2, f32)> + '_ {
    const NEIGHBORS: [(i32, i32, f32); 8] = [
        (-1, 0, 1.0),
        (1, 0, 1.0),
        (0, -1, 1.0),
        (0, 1, 1.0),
        (-1, -1, std::f32::consts::SQRT_2),
        (1, -1, std::f32::consts::SQRT_2),
        (-1, 1, std::f32::consts::SQRT_2),
        (1, 1, std::f32::consts::SQRT_2),
    ];

    NEIGHBORS.into_iter().filter_map(move |(dx, dy, cost)| {
        let next = cell + IVec2::new(dx, dy);
        if !snapshot.is_walkable_cell(next) {
            return None;
        }
        if dx != 0
            && dy != 0
            && (!snapshot.is_walkable_cell(cell + IVec2::new(dx, 0))
                || !snapshot.is_walkable_cell(cell + IVec2::new(0, dy)))
        {
            return None;
        }
        Some((next, cost))
    })
}

// 起终点距离较远时，先在 chunk 粒度上跑一次 A* 得到一串 chunk 路径，
// 再把路径上每个 chunk（含 1 格膨胀）圈出来作为“走廊”，供网格级 A* 限界。
fn chunk_corridor(
    snapshot: &NavGridSnapshot,
    start: IVec2,
    target: IVec2,
) -> Option<HashSet<usize>> {
    let start_chunk = snapshot.cell_to_chunk(start);
    let target_chunk = snapshot.cell_to_chunk(target);
    if (start_chunk.x - target_chunk.x).abs() + (start_chunk.y - target_chunk.y).abs()
        <= CHUNK_CORRIDOR_THRESHOLD
    {
        return None;
    }

    let chunk_path = chunk_a_star(snapshot, start_chunk, target_chunk)?;
    let mut allowed = HashSet::new();
    for chunk in chunk_path {
        for y in -CHUNK_CORRIDOR_PADDING..=CHUNK_CORRIDOR_PADDING {
            for x in -CHUNK_CORRIDOR_PADDING..=CHUNK_CORRIDOR_PADDING {
                if let Some(index) = snapshot.chunk_index(chunk + IVec2::new(x, y)) {
                    allowed.insert(index);
                }
            }
        }
    }
    Some(allowed)
}

// chunk 粒度上的 A*：把每个 32×32 网格块当作一个节点，只走可通行且已重建的块。
fn chunk_a_star(snapshot: &NavGridSnapshot, start: IVec2, target: IVec2) -> Option<Vec<IVec2>> {
    let start_index = snapshot.chunk_index(start)?;
    let target_index = snapshot.chunk_index(target)?;
    let chunk_count = (snapshot.chunks_x * snapshot.chunks_y) as usize;
    let mut frontier = BinaryHeap::new();
    let mut came_from = vec![usize::MAX; chunk_count];
    let mut costs = vec![f32::MAX; chunk_count];
    costs[start_index] = 0.0;
    frontier.push(GridNode {
        index: start_index,
        cost: 0.0,
        estimated_total: start.as_vec2().distance(target.as_vec2()),
    });

    while let Some(current) = frontier.pop() {
        if current.index == target_index {
            return Some(reconstruct_chunk_path(
                snapshot,
                &came_from,
                start_index,
                target_index,
            ));
        }
        if current.cost > costs[current.index] {
            continue;
        }

        let chunk = IVec2::new(
            current.index as i32 % snapshot.chunks_x,
            current.index as i32 / snapshot.chunks_x,
        );
        for neighbor in cardinal_neighbors(chunk) {
            let Some(neighbor_index) = snapshot.chunk_index(neighbor) else {
                continue;
            };
            if snapshot.chunk_walkable[neighbor_index] == 0
                || snapshot.dirty_chunks[neighbor_index] != 0
            {
                continue;
            }

            let next_cost = current.cost + 1.0;
            if next_cost < costs[neighbor_index] {
                costs[neighbor_index] = next_cost;
                came_from[neighbor_index] = current.index;
                frontier.push(GridNode {
                    index: neighbor_index,
                    cost: next_cost,
                    estimated_total: next_cost + neighbor.as_vec2().distance(target.as_vec2()),
                });
            }
        }
    }

    None
}

fn reconstruct_cell_path(
    snapshot: &NavGridSnapshot,
    came_from: &[usize],
    start_index: usize,
    target_index: usize,
) -> Vec<IVec2> {
    let mut current = target_index;
    let mut path = vec![snapshot_cell_from_index(snapshot, current)];
    while current != start_index {
        current = came_from[current];
        path.push(snapshot_cell_from_index(snapshot, current));
    }
    path.reverse();
    path
}

fn reconstruct_chunk_path(
    snapshot: &NavGridSnapshot,
    came_from: &[usize],
    start_index: usize,
    target_index: usize,
) -> Vec<IVec2> {
    let mut current = target_index;
    let mut path = vec![snapshot_chunk_from_index(snapshot, current)];
    while current != start_index {
        current = came_from[current];
        path.push(snapshot_chunk_from_index(snapshot, current));
    }
    path.reverse();
    path
}

// 路径平滑（string pulling）：从锚点出发，跳过“整段无障碍直连”的中间节点，
// 让折线路径变直，减少殖民者的“之”字形走位。
fn smooth_path(snapshot: &NavGridSnapshot, path: &[IVec2]) -> Vec<IVec2> {
    if path.len() <= 2 {
        return path.to_vec();
    }

    let mut result = vec![path[0]];
    let mut anchor = 0usize;
    while anchor < path.len() - 1 {
        let mut next = path.len() - 1;
        while next > anchor + 1 && !grid_line_clear(snapshot, path[anchor], path[next]) {
            next -= 1;
        }
        result.push(path[next]);
        anchor = next;
    }
    result
}

fn grid_line_clear(snapshot: &NavGridSnapshot, from: IVec2, to: IVec2) -> bool {
    let delta = to - from;
    let steps = delta.x.abs().max(delta.y.abs()).max(1);
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let x = (from.x as f32 + delta.x as f32 * t).round() as i32;
        let y = (from.y as f32 + delta.y as f32 * t).round() as i32;
        if !snapshot.is_walkable_cell(IVec2::new(x, y)) {
            return false;
        }
    }
    true
}

#[derive(Clone, Copy, Debug)]
struct GridNode {
    index: usize,
    cost: f32,
    estimated_total: f32,
}

impl PartialEq for GridNode {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.cost == other.cost
            && self.estimated_total == other.estimated_total
    }
}

impl Eq for GridNode {}

impl Ord for GridNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_total
            .partial_cmp(&self.estimated_total)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                other
                    .cost
                    .partial_cmp(&self.cost)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for GridNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn snapshot_cell_from_index(snapshot: &NavGridSnapshot, index: usize) -> IVec2 {
    IVec2::new(index as i32 % snapshot.width, index as i32 / snapshot.width)
}

fn snapshot_chunk_from_index(snapshot: &NavGridSnapshot, index: usize) -> IVec2 {
    IVec2::new(
        index as i32 % snapshot.chunks_x,
        index as i32 / snapshot.chunks_x,
    )
}

fn cardinal_neighbors(cell: IVec2) -> [IVec2; 4] {
    [
        cell + IVec2::NEG_X,
        cell + IVec2::X,
        cell + IVec2::NEG_Y,
        cell + IVec2::Y,
    ]
}

fn path_cache_key(revision: u64, start: Vec3, target: Vec3) -> PathCacheKey {
    PathCacheKey {
        revision,
        sx: snap_path_coord(start.x),
        sz: snap_path_coord(start.z),
        tx: snap_path_coord(target.x),
        tz: snap_path_coord(target.z),
    }
}

fn snap_path_coord(value: f32) -> i32 {
    (value / NAV_CELL_SIZE).round() as i32
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

fn bounds_intersect(left: NavigationDirtyArea, right: NavigationDirtyArea) -> bool {
    left.min.x <= right.max.x
        && left.max.x >= right.min.x
        && left.min.y <= right.max.y
        && left.max.y >= right.min.y
}

fn div_ceil_i32(value: i32, divisor: i32) -> i32 {
    (value + divisor - 1) / divisor
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        building::{WorldGeometry, rectangle_polygon},
        terrain::DEFAULT_TERRAIN_SEED,
    };

    const SEED: u64 = DEFAULT_TERRAIN_SEED;

    fn test_entity(index: u32) -> Entity {
        Entity::from_raw_u32(index).unwrap()
    }

    fn synced_grid(geometry: &mut WorldGeometry) -> NavGrid {
        let mut grid = NavGrid::default();
        grid.sync_from_geometry(geometry, SEED);
        grid
    }

    #[test]
    fn grid_rasterizes_blockers_and_ignores_passable_roads() {
        let mut geometry = WorldGeometry::default();
        geometry.occupy_polygon(
            rectangle_polygon(Vec3::new(0.0, 0.0, 0.0), Vec2::splat(1.0), 0.0),
            test_entity(1),
            false,
        );
        geometry.occupy_polygon(
            rectangle_polygon(Vec3::new(2.0, 0.0, 0.0), Vec2::splat(1.0), 0.0),
            test_entity(2),
            true,
        );

        let grid = synced_grid(&mut geometry);
        let snapshot = grid.snapshot();

        assert!(
            snapshot
                .nearest_walkable_cell(Vec2::new(0.0, 0.0))
                .is_some()
        );
        assert!(!snapshot.is_walkable_cell(snapshot.cell_at_world(Vec2::new(0.0, 0.0)).unwrap()));
        assert!(snapshot.is_walkable_cell(snapshot.cell_at_world(Vec2::new(2.0, 0.0)).unwrap()));
    }

    #[test]
    fn dirty_chunks_are_rebuilt_incrementally() {
        let mut geometry = WorldGeometry::default();
        let mut grid = synced_grid(&mut geometry);
        let blocker = test_entity(1);
        geometry.occupy_polygon(
            rectangle_polygon(Vec3::new(1.0, 0.0, 1.0), Vec2::splat(1.0), 0.0),
            blocker,
            false,
        );

        grid.sync_from_geometry(&mut geometry, SEED);
        assert!(grid.pending_dirty_chunks() <= 1);
        assert!(grid.pending_dirty_chunks() == 0 || grid.revision() > 1);

        for _ in 0..4 {
            grid.sync_from_geometry(&mut geometry, SEED);
        }
        assert_eq!(grid.pending_dirty_chunks(), 0);
    }

    #[test]
    fn connected_components_reject_unreachable_targets() {
        let mut geometry = WorldGeometry::default();
        geometry.occupy_polygon(
            rectangle_polygon(
                Vec3::new(0.0, 0.0, 0.0),
                Vec2::new(1.0, MAP_BUILD_HALF_EXTENT * 2.0),
                0.0,
            ),
            test_entity(1),
            false,
        );

        let grid = synced_grid(&mut geometry);

        assert!(!grid.maybe_reachable(Vec3::new(-2.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn smoothed_path_does_not_cross_blocked_cells() {
        let mut geometry = WorldGeometry::default();
        geometry.occupy_polygon(
            rectangle_polygon(Vec3::new(1.5, 0.0, 0.0), Vec2::new(1.0, 1.6), 0.0),
            test_entity(1),
            false,
        );
        let grid = synced_grid(&mut geometry);
        let snapshot = grid.snapshot();
        let start = Vec3::new(0.0, 0.0, 0.0);
        let target = Vec3::new(3.0, 0.0, 0.0);

        let path = snapshot.path_to_waypoints(start, target, SEED).unwrap();
        let mut previous = snapshot.cell_at_world(xz(start)).unwrap();
        for waypoint in path {
            let next = snapshot.cell_at_world(xz(waypoint)).unwrap();
            assert!(grid_line_clear(&snapshot, previous, next));
            previous = next;
        }
    }

    #[test]
    fn stale_async_results_are_distinguishable() {
        let mut geometry = WorldGeometry::default();
        let grid = synced_grid(&mut geometry);
        let mut planner = PathPlanner::default();
        let request = planner.request_path(&grid, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
        planner.insert_result_for_test(PathResult {
            id: request,
            revision: grid.revision().saturating_sub(1),
            path: Some(vec![Vec3::new(1.0, 0.0, 0.0)]),
        });

        let result = planner.take_result(request).unwrap();

        assert_ne!(result.revision, grid.revision());
    }

    #[test]
    #[ignore = "400-request async path planning pressure check"]
    fn pressure_queues_four_hundred_async_path_requests() {
        use bevy::tasks::TaskPoolBuilder;

        let _ = AsyncComputeTaskPool::get_or_init(|| {
            TaskPoolBuilder::new()
                .num_threads(4)
                .thread_name("nav-pressure".to_string())
                .build()
        });

        let mut geometry = WorldGeometry::default();
        let grid = synced_grid(&mut geometry);
        let mut planner = PathPlanner::default();

        for index in 0..400 {
            let lane = index as f32 * 0.05;
            planner.request_path(&grid, Vec3::new(-8.0, 0.0, lane), Vec3::new(8.0, 0.0, lane));
        }

        for _ in 0..2_000 {
            planner.submit_queued(&grid, SEED);
            planner.poll_completed(grid.revision());
            if planner.results.len() == 400 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        assert_eq!(planner.queue.len(), 0);
        assert_eq!(planner.in_flight.len(), 0);
        assert_eq!(planner.results.len(), 400);
    }
}
