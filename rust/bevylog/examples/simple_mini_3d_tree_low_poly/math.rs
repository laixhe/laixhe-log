//! 数学工具：
//! - 鼠标射线与程序化地形的求交：先按固定步长下探，穿过地面后再二分精化，
//!   得到精确的“鼠标指向的地面点”（用于建造放置、点选）。
//! - xz 平面距离 / 边界判断（游戏逻辑都忽略 y 轴）。

use bevy::prelude::*;

use crate::terrain::terrain_height;
use crate::types::MAP_BUILD_HALF_EXTENT;

pub fn xz(position: Vec3) -> Vec2 {
    Vec2::new(position.x, position.z)
}

pub fn xz_distance(left: Vec3, right: Vec3) -> f32 {
    xz(left).distance(xz(right))
}

pub fn xz_length(position: Vec3) -> f32 {
    xz(position).length()
}

pub fn within_world_bounds(point: Vec2) -> bool {
    point.x >= -MAP_BUILD_HALF_EXTENT
        && point.x <= MAP_BUILD_HALF_EXTENT
        && point.y >= -MAP_BUILD_HALF_EXTENT
        && point.y <= MAP_BUILD_HALF_EXTENT
}

const RAY_STEP: f32 = 0.5;
const BINARY_REFINE_ITERATIONS: usize = 6;

pub fn terrain_pick_max_distance() -> f32 {
    MAP_BUILD_HALF_EXTENT * 4.0
}

// 射线与地形求交：沿射线方向以 RAY_STEP 步进采样，第一次“钻到地面以下”时，
// 说明交点就在 [t-RAY_STEP, t] 之间，再用二分把交点收敛到精度以内。
pub fn ray_terrain_intersection(ray: Ray3d, seed: u64, max_distance: f32) -> Option<Vec3> {
    let mut t = 0.0;
    let max_steps = (max_distance.max(0.0) / RAY_STEP).ceil() as usize;

    for _ in 0..=max_steps {
        if t > max_distance {
            break;
        }
        let point = ray.origin + ray.direction * t;
        let terrain_y = terrain_height(seed, point.x, point.z);
        if point.y <= terrain_y {
            if t <= RAY_STEP {
                return Some(point_at_terrain_height(ray, seed, t));
            }
            return Some(binary_refine_terrain(ray, seed, t - RAY_STEP, t));
        }
        t += RAY_STEP;
    }

    None
}

// 二分精化：地面高度函数是连续的，把“在地上”和“在地下”两个端点不断对半分，
// 收敛到交点 t。最后把 y 修正为精确地形高度，避免网格采样误差。
fn binary_refine_terrain(ray: Ray3d, seed: u64, t_low: f32, t_high: f32) -> Vec3 {
    let mut lo = t_low;
    let mut hi = t_high;

    for _ in 0..BINARY_REFINE_ITERATIONS {
        let mid = (lo + hi) * 0.5;
        let point = ray.origin + ray.direction * mid;
        let terrain_y = terrain_height(seed, point.x, point.z);
        if point.y <= terrain_y {
            hi = mid;
        } else {
            lo = mid;
        }
    }

    let t = (lo + hi) * 0.5;
    let result = ray.origin + ray.direction * t;
    Vec3::new(result.x, terrain_height(seed, result.x, result.z), result.z)
}

fn point_at_terrain_height(ray: Ray3d, seed: u64, t: f32) -> Vec3 {
    let point = ray.origin + ray.direction * t;
    Vec3::new(point.x, terrain_height(seed, point.x, point.z), point.z)
}
