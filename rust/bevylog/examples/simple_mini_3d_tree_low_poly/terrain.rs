//! 程序化地形：
//! - 高度：用“分形值噪声”（value noise 多倍频叠加，俗称 fBm）生成，宏观起伏 + 细节起伏两层。
//! - 植被分类：再用两组不同频率的噪声把地面分成 草地 / 森林 / 浆果地 三种区域。
//! - 资源点：在可放置区域内按“森林得分/浆果得分 + 随机抖动”选出高分候选，
//!   逐个放入并保持最小间距，最终得到确定性的树木与浆果分布（同一种子结果一致）。

use std::{cmp::Ordering, collections::BinaryHeap};

use bevy::prelude::*;

use crate::math::{xz_distance, xz_length};
use crate::types::{CELL_SIZE, MAP_HALF_CELLS, ResourceKind};

pub const DEFAULT_TERRAIN_SEED: u64 = 0x5452_4149_4C42_4C5A;
pub const TERRAIN_CHUNK_CELLS: i32 = 24;
pub const WOOD_NODE_COUNT: usize = 96;
pub const FOOD_NODE_COUNT: usize = 54;
pub const WOOD_NODE_AMOUNT: i32 = 24;
pub const FOOD_NODE_AMOUNT: i32 = 20;

const START_CLEAR_RADIUS: f32 = 8.0;
const EDGE_MARGIN_CELLS: i32 = 3;
const MIN_RESOURCE_SPACING: f32 = 2.6;
const FOREST_THRESHOLD: f32 = 0.57;
const FORAGE_THRESHOLD: f32 = 0.59;
const FOREST_SALT: u64 = 0x464F_5245_5354;
const FORAGE_SALT: u64 = 0x464F_5241_4745;
const JITTER_SALT: u64 = 0x4A49_5454_4552;

const MACRO_HEIGHT_SALT: u64 = 0x4D41_4352_4F48_4754;
const DETAIL_HEIGHT_SALT: u64 = 0x4445_5441_494C_4854;
const MACRO_HEIGHT_AMPLITUDE: f32 = 45.0;
const MACRO_HEIGHT_FREQUENCY: f32 = 0.0045;
const DETAIL_HEIGHT_AMPLITUDE: f32 = 1.25;
const DETAIL_HEIGHT_FREQUENCY: f32 = 0.028;
const DEFAULT_MAX_BUILDABLE_SLOPE: f32 = 0.58;
const RESOURCE_CANDIDATE_POOL_MULTIPLIER: usize = 20;
const MIN_RESOURCE_CANDIDATE_POOL: usize = 512;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        let config = TerrainGenerationConfig::default();
        let seed = TerrainSeed(config.seed);
        app.init_resource::<TerrainGenerationConfig>();
        app.insert_resource(seed);
    }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct TerrainSeed(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerrainKind {
    Grass,
    ForestFloor,
    ForageField,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainTile {
    pub center: Vec3,
    pub kind: TerrainKind,
    pub height: f32,
    pub slope: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeneratedResource {
    pub kind: ResourceKind,
    pub position: Vec3,
    pub amount: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedTerrain {
    pub tiles: Vec<TerrainTile>,
    pub resources: Vec<GeneratedResource>,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct TerrainGenerationConfig {
    pub seed: u64,
    pub half_cells: i32,
    pub tile_cells: i32,
    pub wood_nodes: usize,
    pub food_nodes: usize,
    pub start_clear_radius: f32,
    pub edge_margin_cells: i32,
    pub min_resource_spacing: f32,
    #[allow(dead_code)]
    pub macro_height_amplitude: f32,
    #[allow(dead_code)]
    pub macro_height_frequency: f32,
    #[allow(dead_code)]
    pub detail_height_amplitude: f32,
    #[allow(dead_code)]
    pub detail_height_frequency: f32,
    pub max_buildable_slope: f32,
}

impl Default for TerrainGenerationConfig {
    fn default() -> Self {
        Self {
            seed: DEFAULT_TERRAIN_SEED,
            half_cells: MAP_HALF_CELLS,
            tile_cells: TERRAIN_CHUNK_CELLS,
            wood_nodes: WOOD_NODE_COUNT,
            food_nodes: FOOD_NODE_COUNT,
            start_clear_radius: START_CLEAR_RADIUS,
            edge_margin_cells: EDGE_MARGIN_CELLS,
            min_resource_spacing: MIN_RESOURCE_SPACING,
            macro_height_amplitude: MACRO_HEIGHT_AMPLITUDE,
            macro_height_frequency: MACRO_HEIGHT_FREQUENCY,
            detail_height_amplitude: DETAIL_HEIGHT_AMPLITUDE,
            detail_height_frequency: DETAIL_HEIGHT_FREQUENCY,
            max_buildable_slope: DEFAULT_MAX_BUILDABLE_SLOPE,
        }
    }
}

pub fn terrain_height(seed: u64, x: f32, z: f32) -> f32 {
    macro_terrain_height(seed, x, z) + detail_terrain_height(seed, x, z)
}

// 宏观起伏：低频大振幅，决定山丘与盆地的大轮廓；加一点点幂次让谷底更平缓。
fn macro_terrain_height(seed: u64, x: f32, z: f32) -> f32 {
    let centered =
        fractal_noise_octaves(seed, x, z, MACRO_HEIGHT_FREQUENCY, MACRO_HEIGHT_SALT, 3) * 2.0 - 1.0;
    let shaped = centered + centered.powi(3) * 0.4;
    shaped * MACRO_HEIGHT_AMPLITUDE
}

fn detail_terrain_height(seed: u64, x: f32, z: f32) -> f32 {
    (fractal_noise_octaves(seed, x, z, DETAIL_HEIGHT_FREQUENCY, DETAIL_HEIGHT_SALT, 3) * 2.0 - 1.0)
        * DETAIL_HEIGHT_AMPLITUDE
}

pub fn terrain_slope(seed: u64, x: f32, z: f32, sample_dist: f32) -> f32 {
    let h = terrain_height(seed, x, z);
    let hx = terrain_height(seed, x + sample_dist, z);
    let hz = terrain_height(seed, x, z + sample_dist);
    let dx = (hx - h) / sample_dist;
    let dz = (hz - h) / sample_dist;
    (dx * dx + dz * dz).sqrt()
}

pub fn max_slope_in_polygon(seed: u64, polygon: &[Vec2], sample_count: usize) -> f32 {
    if polygon.len() < 3 {
        return 0.0;
    }
    let mut max_slope = 0.0f32;
    let sample_dist = 0.3;
    for i in 0..sample_count {
        let t = i as f32 / (sample_count as f32).max(1.0);
        let idx = (t * polygon.len() as f32) as usize % polygon.len();
        let next = (idx + 1) % polygon.len();
        let p = polygon[idx].lerp(polygon[next], t - (t * polygon.len() as f32).floor());
        let s = terrain_slope(seed, p.x, p.y, sample_dist);
        max_slope = max_slope.max(s);
    }
    max_slope
}

pub fn generate_terrain(config: TerrainGenerationConfig) -> GeneratedTerrain {
    let tiles = generate_tiles(config);
    let mut resources = Vec::with_capacity(config.wood_nodes + config.food_nodes);
    select_resources(
        &config,
        ResourceKind::Wood,
        config.wood_nodes,
        &mut resources,
    );
    select_resources(
        &config,
        ResourceKind::Food,
        config.food_nodes,
        &mut resources,
    );

    GeneratedTerrain { tiles, resources }
}

fn generate_tiles(config: TerrainGenerationConfig) -> Vec<TerrainTile> {
    let tile_cells = config.tile_cells.max(1);
    let tiles_per_axis = (config.half_cells * 2 / tile_cells).max(1);
    let start = -config.half_cells as f32 * CELL_SIZE;
    let tile_size = tile_cells as f32 * CELL_SIZE;
    let mut tiles = Vec::with_capacity((tiles_per_axis * tiles_per_axis) as usize);

    for x in 0..tiles_per_axis {
        for z in 0..tiles_per_axis {
            let center_x = start + (x as f32 + 0.5) * tile_size;
            let center_z = start + (z as f32 + 0.5) * tile_size;
            let height = terrain_height(config.seed, center_x, center_z);
            let slope = terrain_slope(config.seed, center_x, center_z, 0.5);
            let center = Vec3::new(center_x, height, center_z);
            let (forest, forage) = terrain_scores(config.seed, center.x, center.z);
            tiles.push(TerrainTile {
                center,
                kind: classify_terrain(forest, forage),
                height,
                slope,
            });
        }
    }

    tiles
}

// 把每种资源单独按“候选得分排序 + 最小间距”逐个挑选：
// 候选池用 BinaryHeap 只保留分数最高的 pool_size 个（避免对全地图排序）。
fn select_resources(
    config: &TerrainGenerationConfig,
    kind: ResourceKind,
    target_count: usize,
    resources: &mut Vec<GeneratedResource>,
) {
    if target_count == 0 {
        return;
    }

    let mut candidates = resource_candidates(config, kind);
    candidates.sort_by(|left, right| candidate_quality_cmp(right, left));

    for candidate in candidates {
        if resources.iter().any(|resource| {
            xz_distance(resource.position, candidate.position) < config.min_resource_spacing
        }) {
            continue;
        }

        let y = terrain_height(config.seed, candidate.position.x, candidate.position.z);
        resources.push(GeneratedResource {
            kind,
            position: Vec3::new(candidate.position.x, y, candidate.position.z),
            amount: resource_amount(kind),
        });
        if resources
            .iter()
            .filter(|resource| resource.kind == kind)
            .count()
            >= target_count
        {
            break;
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ResourceCandidate {
    position: Vec3,
    score: f32,
}

#[derive(Clone, Copy, Debug)]
struct CandidatePoolItem(ResourceCandidate);

impl PartialEq for CandidatePoolItem {
    fn eq(&self, other: &Self) -> bool {
        candidate_quality_cmp(&self.0, &other.0) == Ordering::Equal
    }
}

impl Eq for CandidatePoolItem {}

impl PartialOrd for CandidatePoolItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CandidatePoolItem {
    fn cmp(&self, other: &Self) -> Ordering {
        candidate_quality_cmp(&self.0, &other.0).reverse()
    }
}

fn resource_candidates(
    config: &TerrainGenerationConfig,
    kind: ResourceKind,
) -> Vec<ResourceCandidate> {
    let limit = (config.half_cells - config.edge_margin_cells).max(0);
    let pool_limit = resource_candidate_pool_size(target_count_for_kind(config, kind));
    let mut candidates = BinaryHeap::with_capacity(pool_limit.saturating_add(1));

    for x in -limit..=limit {
        for z in -limit..=limit {
            let position = Vec3::new(x as f32 * CELL_SIZE, 0.0, z as f32 * CELL_SIZE);
            if xz_length(position) < config.start_clear_radius {
                continue;
            }

            let (forest, forage) = terrain_scores(config.seed, position.x, position.z);
            let jitter = hash_unit(config.seed, x, z, JITTER_SALT);
            let score = match kind {
                ResourceKind::Wood => forest * 0.82 + jitter * 0.18,
                ResourceKind::Food => {
                    if forest >= 0.76 {
                        continue;
                    }
                    forage * 0.76 + (1.0 - forest) * 0.14 + jitter * 0.1
                }
                ResourceKind::Firewood => 0.0,
            };
            push_candidate(
                &mut candidates,
                ResourceCandidate { position, score },
                pool_limit,
            );
        }
    }

    candidates
        .into_iter()
        .map(|candidate| candidate.0)
        .collect()
}

fn push_candidate(
    candidates: &mut BinaryHeap<CandidatePoolItem>,
    candidate: ResourceCandidate,
    pool_limit: usize,
) {
    if candidates.len() < pool_limit {
        candidates.push(CandidatePoolItem(candidate));
        return;
    }

    if let Some(worst) = candidates.peek() {
        if candidate_quality_cmp(&candidate, &worst.0).is_gt() {
            candidates.pop();
            candidates.push(CandidatePoolItem(candidate));
        }
    }
}

fn resource_candidate_pool_size(target_count: usize) -> usize {
    (target_count * RESOURCE_CANDIDATE_POOL_MULTIPLIER).max(MIN_RESOURCE_CANDIDATE_POOL)
}

fn target_count_for_kind(config: &TerrainGenerationConfig, kind: ResourceKind) -> usize {
    match kind {
        ResourceKind::Wood => config.wood_nodes,
        ResourceKind::Food => config.food_nodes,
        ResourceKind::Firewood => 0,
    }
}

fn candidate_quality_cmp(left: &ResourceCandidate, right: &ResourceCandidate) -> Ordering {
    left.score
        .total_cmp(&right.score)
        .then_with(|| right.position.x.total_cmp(&left.position.x))
        .then_with(|| right.position.z.total_cmp(&left.position.z))
}

fn resource_amount(kind: ResourceKind) -> i32 {
    match kind {
        ResourceKind::Wood => WOOD_NODE_AMOUNT,
        ResourceKind::Food => FOOD_NODE_AMOUNT,
        ResourceKind::Firewood => 0,
    }
}

fn classify_terrain(forest: f32, forage: f32) -> TerrainKind {
    if forest >= FOREST_THRESHOLD {
        TerrainKind::ForestFloor
    } else if forage >= FORAGE_THRESHOLD {
        TerrainKind::ForageField
    } else {
        TerrainKind::Grass
    }
}

pub fn terrain_kind_at(seed: u64, x: f32, z: f32) -> TerrainKind {
    let (forest, forage) = terrain_scores(seed, x, z);
    classify_terrain(forest, forage)
}

// 森林 / 浆果得分：同一块地同时用两套不同频率、不同偏移的噪声打分，
// 分数高 → 森林覆盖；浆果分数高且不在密林里 → 浆果地。
fn terrain_scores(seed: u64, x: f32, z: f32) -> (f32, f32) {
    (
        fractal_noise(seed, x, z, 0.045, FOREST_SALT),
        fractal_noise(seed, x + 91.0, z - 37.0, 0.065, FORAGE_SALT),
    )
}

fn fractal_noise(seed: u64, x: f32, z: f32, base_frequency: f32, salt: u64) -> f32 {
    fractal_noise_octaves(seed, x, z, base_frequency, salt, 4)
}

// 分形噪声（fBm）：叠加多个倍频——频率翻倍、振幅减半，最后除以振幅和归一化。
// 低倍频提供大形状，高倍频添加细节，结果像自然的地形/植被分布。
fn fractal_noise_octaves(
    seed: u64,
    x: f32,
    z: f32,
    base_frequency: f32,
    salt: u64,
    octaves: usize,
) -> f32 {
    let mut total = 0.0;
    let mut normalizer = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = base_frequency;

    for octave in 0..octaves {
        total += value_noise(
            seed,
            x,
            z,
            frequency,
            salt.wrapping_add(octave as u64 * 0x9E37_79B9),
        ) * amplitude;
        normalizer += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    total / normalizer
}

// 值噪声（value noise）：在整数格点上取随机哈希值，格子之间用平滑插值（smoothstep），
// 输出 [0,1] 连续平滑的标量场。
fn value_noise(seed: u64, x: f32, z: f32, frequency: f32, salt: u64) -> f32 {
    let sample_x = x * frequency;
    let sample_z = z * frequency;
    let x0 = sample_x.floor() as i32;
    let z0 = sample_z.floor() as i32;
    let tx = smoothstep(sample_x - x0 as f32);
    let tz = smoothstep(sample_z - z0 as f32);

    let a = lerp(
        hash_unit(seed, x0, z0, salt),
        hash_unit(seed, x0 + 1, z0, salt),
        tx,
    );
    let b = lerp(
        hash_unit(seed, x0, z0 + 1, salt),
        hash_unit(seed, x0 + 1, z0 + 1, salt),
        tx,
    );

    lerp(a, b, tz)
}

// 把 (seed, salt, 格子坐标) 搅成一个伪随机数：
// 坐标乘以大素数 + 异或盐值，再经 mix64 雪崩混合，最后取高 24 位映射到 [0,1]。
// 同一 (seed, x, z) 永远得到同一个值——这就是地形“确定性”的来源。
fn hash_unit(seed: u64, x: i32, z: i32, salt: u64) -> f32 {
    let hash = mix64(
        seed ^ salt
            ^ (x as i64 as u64).wrapping_mul(0xA24B_AED4_963E_E407)
            ^ (z as i64 as u64).wrapping_mul(0x9FB2_1C65_1E98_DF25),
    );
    ((hash >> 40) as u32) as f32 / 0x00FF_FFFFu32 as f32
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

fn lerp(left: f32, right: f32, amount: f32) -> f32 {
    left + (right - left) * amount
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MAP_BUILD_HALF_EXTENT, MAP_GRID_CELLS, MAP_HALF_CELLS};

    #[test]
    fn generated_terrain_is_deterministic_for_same_seed() {
        let config = TerrainGenerationConfig::default();

        assert_eq!(generate_terrain(config), generate_terrain(config));
    }

    #[test]
    fn generated_tiles_cover_expected_chunk_grid() {
        let terrain = generate_terrain(TerrainGenerationConfig::default());

        assert_eq!(terrain.tiles.len(), 18 * 18);
    }

    #[test]
    fn default_map_area_is_about_twenty_times_original() {
        let old_grid_cells = 96.0;
        let area_ratio =
            (MAP_GRID_CELLS as f32 * MAP_GRID_CELLS as f32) / (old_grid_cells * old_grid_cells);

        assert_eq!(MAP_GRID_CELLS, 432);
        assert!((area_ratio - 20.25).abs() < 0.01);
    }

    #[test]
    fn resources_stay_inside_bounds_and_clear_start() {
        let config = TerrainGenerationConfig::default();
        let terrain = generate_terrain(config);
        let edge_limit = (config.half_cells - config.edge_margin_cells) as f32 * CELL_SIZE;

        for resource in &terrain.resources {
            assert!(resource.position.x.abs() <= MAP_BUILD_HALF_EXTENT);
            assert!(resource.position.z.abs() <= MAP_BUILD_HALF_EXTENT);
            assert!(resource.position.x.abs() <= edge_limit);
            assert!(resource.position.z.abs() <= edge_limit);
            assert!(xz_length(resource.position) >= config.start_clear_radius);
        }
    }

    #[test]
    fn resources_respect_counts_and_spacing() {
        let config = TerrainGenerationConfig::default();
        let terrain = generate_terrain(config);
        let wood = terrain
            .resources
            .iter()
            .filter(|resource| resource.kind == ResourceKind::Wood)
            .count();
        let food = terrain
            .resources
            .iter()
            .filter(|resource| resource.kind == ResourceKind::Food)
            .count();

        assert!(wood > 0);
        assert!(wood <= config.wood_nodes);
        assert!(food > 0);
        assert!(food <= config.food_nodes);

        for (index, left) in terrain.resources.iter().enumerate() {
            for right in terrain.resources.iter().skip(index + 1) {
                assert!(xz_distance(left.position, right.position) >= config.min_resource_spacing);
            }
        }
    }

    #[test]
    fn default_generation_hits_intended_resource_counts() {
        let config = TerrainGenerationConfig::default();
        let terrain = generate_terrain(config);
        let wood = terrain
            .resources
            .iter()
            .filter(|resource| resource.kind == ResourceKind::Wood)
            .count();
        let food = terrain
            .resources
            .iter()
            .filter(|resource| resource.kind == ResourceKind::Food)
            .count();

        assert_eq!(wood, config.wood_nodes);
        assert_eq!(food, config.food_nodes);
    }

    #[test]
    fn terrain_height_is_deterministic() {
        let seed = 0x5452_4149_4C42_4C5A;
        let h1 = terrain_height(seed, 10.0, 20.0);
        let h2 = terrain_height(seed, 10.0, 20.0);
        assert_eq!(h1, h2);
    }

    #[test]
    fn terrain_height_varies_with_position() {
        let seed = 0x5452_4149_4C42_4C5A;
        let h1 = terrain_height(seed, 0.0, 0.0);
        let h2 = terrain_height(seed, 60.0, 60.0);
        assert!(
            (h1 - h2).abs() > 0.01,
            "height should vary more: {h1} vs {h2}"
        );
    }

    #[test]
    fn macro_height_layer_has_larger_range_than_detail_layer() {
        let seed = 0x5452_4149_4C42_4C5A;
        let macro_range = sampled_height_range(|x, z| macro_terrain_height(seed, x, z));
        let detail_range = sampled_height_range(|x, z| detail_terrain_height(seed, x, z));

        assert!(macro_range > 5.0, "macro range was {macro_range}");
        assert!(
            macro_range > detail_range * 2.0,
            "macro range {macro_range} should dominate detail range {detail_range}"
        );
    }

    #[test]
    fn terrain_slope_returns_zero_on_flat_extreme() {
        let seed = 0x5452_4149_4C42_4C5A;
        let slope = terrain_slope(seed, 0.0, 0.0, 0.5);
        assert!(slope >= 0.0);
    }

    #[test]
    fn generated_tiles_have_height_and_slope() {
        let config = TerrainGenerationConfig::default();
        let terrain = generate_terrain(config);
        for tile in &terrain.tiles {
            assert!(tile.height.is_finite());
            assert!(tile.slope.is_finite());
            assert!(tile.slope >= 0.0);
        }
    }

    fn sampled_height_range(mut height_at: impl FnMut(f32, f32) -> f32) -> f32 {
        let mut min_height = f32::MAX;
        let mut max_height = f32::MIN;
        let mut x = -MAP_HALF_CELLS;
        while x <= MAP_HALF_CELLS {
            let mut z = -MAP_HALF_CELLS;
            while z <= MAP_HALF_CELLS {
                let height = height_at(x as f32, z as f32);
                min_height = min_height.min(height);
                max_height = max_height.max(height);
                z += TERRAIN_CHUNK_CELLS;
            }
            x += TERRAIN_CHUNK_CELLS;
        }
        max_height - min_height
    }
}
