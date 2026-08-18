//! 多边形几何工具（全部在 xz 平面，y 被忽略）：
//! 矩形占地生成、鞋带公式面积、凸性判定、线段相交、点在多边形内、
//! 基于分离轴定理（SAT）的多边形相交检测——被 `geometry` 与 `farm` 复用。

use bevy::prelude::*;

use crate::types::{BuildingKind, CELL_SIZE};

pub(super) const FOOTPRINT_SCALE: f32 = 0.9;
pub(super) const ROAD_FOOTPRINT_SCALE: f32 = 0.95;
pub(super) const GEOMETRY_EPSILON: f32 = 0.0001;

pub fn footprint_polygon(
    kind: BuildingKind,
    center: Vec3,
    size: IVec2,
    rotation_angle: f32,
) -> Vec<Vec2> {
    rectangle_polygon(center, footprint_dimensions(kind, size), rotation_angle)
}

pub fn resource_obstacle_polygon(position: Vec3) -> Vec<Vec2> {
    rectangle_polygon(
        Vec3::new(position.x, 0.0, position.z),
        Vec2::splat(0.8),
        0.0,
    )
}

pub fn rectangle_polygon(center: Vec3, size: Vec2, rotation_angle: f32) -> Vec<Vec2> {
    let half = size * 0.5;
    let cos = rotation_angle.cos();
    let sin = rotation_angle.sin();
    [
        (-half.x, -half.y),
        (half.x, -half.y),
        (half.x, half.y),
        (-half.x, half.y),
    ]
    .into_iter()
    .map(|(local_x, local_z)| {
        Vec2::new(
            center.x + local_x * cos + local_z * sin,
            center.z - local_x * sin + local_z * cos,
        )
    })
    .collect()
}

pub fn expanded_polygon(polygon: &[Vec2], padding: f32) -> Vec<Vec2> {
    if padding <= 0.0 || polygon.is_empty() {
        return polygon.to_vec();
    }

    let center = polygon
        .iter()
        .copied()
        .fold(Vec2::ZERO, |sum, point| sum + point)
        / polygon.len() as f32;
    polygon
        .iter()
        .map(|point| {
            let from_center = *point - center;
            if from_center.length_squared() < GEOMETRY_EPSILON {
                *point
            } else {
                *point + from_center.normalize() * padding
            }
        })
        .collect()
}

pub fn polygons_intersect(left: &[Vec2], right: &[Vec2]) -> bool {
    if left.len() < 3 || right.len() < 3 {
        return false;
    }

    !has_separating_axis(left, right) && !has_separating_axis(right, left)
}

pub fn polygon_area(polygon: &[Vec2]) -> f32 {
    signed_polygon_area(polygon).abs()
}

// 鞋带公式（shoelace）：沿多边形边求和叉积的一半。
// 正负号表示顶点绕向（顺时针 / 逆时针），取绝对值就是面积。
pub fn signed_polygon_area(polygon: &[Vec2]) -> f32 {
    if polygon.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for index in 0..polygon.len() {
        let a = polygon[index];
        let b = polygon[(index + 1) % polygon.len()];
        area += a.x * b.y - b.x * a.y;
    }
    area * 0.5
}

// 凸性判定：检查每个顶点的拐向（相邻边叉积符号）是否一致——
// 符号翻转说明出现了凹陷；再排除共线、面积为零、自交的情况。
pub fn is_convex_polygon(polygon: &[Vec2]) -> bool {
    if polygon.len() < 3 || polygon_area(polygon) <= GEOMETRY_EPSILON {
        return false;
    }

    let mut sign = 0.0f32;
    for index in 0..polygon.len() {
        let a = polygon[index];
        let b = polygon[(index + 1) % polygon.len()];
        let c = polygon[(index + 2) % polygon.len()];
        let left = b - a;
        let right = c - b;
        if left.length_squared() <= GEOMETRY_EPSILON || right.length_squared() <= GEOMETRY_EPSILON {
            return false;
        }

        let cross = cross_2d(left, right);
        if cross.abs() <= GEOMETRY_EPSILON {
            continue;
        }
        if sign == 0.0 {
            sign = cross.signum();
        } else if sign * cross < -GEOMETRY_EPSILON {
            return false;
        }
    }

    sign != 0.0 && !polygon_has_self_intersection(polygon)
}

pub fn polygon_has_self_intersection(polygon: &[Vec2]) -> bool {
    if polygon.len() < 4 {
        return false;
    }

    for left_index in 0..polygon.len() {
        let left_next = (left_index + 1) % polygon.len();
        for right_index in (left_index + 1)..polygon.len() {
            let right_next = (right_index + 1) % polygon.len();
            if left_index == right_next || left_next == right_index {
                continue;
            }
            if segments_intersect(
                polygon[left_index],
                polygon[left_next],
                polygon[right_index],
                polygon[right_next],
            ) {
                return true;
            }
        }
    }

    false
}

pub fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut sign = 0.0f32;
    for index in 0..polygon.len() {
        let a = polygon[index];
        let b = polygon[(index + 1) % polygon.len()];
        let cross = cross_2d(b - a, point - a);
        if cross.abs() <= GEOMETRY_EPSILON {
            continue;
        }
        if sign == 0.0 {
            sign = cross.signum();
        } else if sign * cross < -GEOMETRY_EPSILON {
            return false;
        }
    }

    true
}

pub fn distance_to_polygon(point: Vec2, polygon: &[Vec2]) -> f32 {
    if point_in_polygon(point, polygon) {
        return 0.0;
    }

    polygon
        .iter()
        .enumerate()
        .map(|(index, start)| {
            let end = polygon[(index + 1) % polygon.len()];
            distance_to_segment(point, *start, end)
        })
        .fold(f32::MAX, f32::min)
}

fn footprint_dimensions(kind: BuildingKind, size: IVec2) -> Vec2 {
    if kind == BuildingKind::Road {
        Vec2::splat(CELL_SIZE * ROAD_FOOTPRINT_SCALE)
    } else {
        Vec2::new(
            size.x as f32 * CELL_SIZE * FOOTPRINT_SCALE,
            size.y as f32 * CELL_SIZE * FOOTPRINT_SCALE,
        )
    }
}

// 分离轴定理（SAT）相交检测：把两个多边形分别投影到对方每条边的法线轴上，
// 只要存在一条轴上两者的投影不重叠，就说明它们不相交。
fn has_separating_axis(left: &[Vec2], right: &[Vec2]) -> bool {
    for index in 0..left.len() {
        let a = left[index];
        let b = left[(index + 1) % left.len()];
        let edge = b - a;
        if edge.length_squared() <= GEOMETRY_EPSILON {
            continue;
        }
        let axis = Vec2::new(-edge.y, edge.x).normalize();
        let (left_min, left_max) = project_polygon(left, axis);
        let (right_min, right_max) = project_polygon(right, axis);
        if left_max <= right_min + GEOMETRY_EPSILON || right_max <= left_min + GEOMETRY_EPSILON {
            return true;
        }
    }
    false
}

fn project_polygon(polygon: &[Vec2], axis: Vec2) -> (f32, f32) {
    polygon
        .iter()
        .map(|point| point.dot(axis))
        .fold((f32::MAX, f32::MIN), |(min, max), value| {
            (min.min(value), max.max(value))
        })
}

fn distance_to_segment(point: Vec2, start: Vec2, end: Vec2) -> f32 {
    let segment = end - start;
    let length_squared = segment.length_squared();
    if length_squared <= GEOMETRY_EPSILON {
        return point.distance(start);
    }
    let t = ((point - start).dot(segment) / length_squared).clamp(0.0, 1.0);
    point.distance(start + segment * t)
}

// 线段相交：先用叉积符号判断两线段是否跨立对方，再单独处理“端点落在线段上”的边界情况。
fn segments_intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let ab_c = cross_2d(b - a, c - a);
    let ab_d = cross_2d(b - a, d - a);
    let cd_a = cross_2d(d - c, a - c);
    let cd_b = cross_2d(d - c, b - c);

    if ab_c.abs() <= GEOMETRY_EPSILON && point_on_segment(c, a, b) {
        return true;
    }
    if ab_d.abs() <= GEOMETRY_EPSILON && point_on_segment(d, a, b) {
        return true;
    }
    if cd_a.abs() <= GEOMETRY_EPSILON && point_on_segment(a, c, d) {
        return true;
    }
    if cd_b.abs() <= GEOMETRY_EPSILON && point_on_segment(b, c, d) {
        return true;
    }

    ab_c.signum() != ab_d.signum() && cd_a.signum() != cd_b.signum()
}

fn point_on_segment(point: Vec2, start: Vec2, end: Vec2) -> bool {
    point.x >= start.x.min(end.x) - GEOMETRY_EPSILON
        && point.x <= start.x.max(end.x) + GEOMETRY_EPSILON
        && point.y >= start.y.min(end.y) - GEOMETRY_EPSILON
        && point.y <= start.y.max(end.y) + GEOMETRY_EPSILON
}

fn cross_2d(left: Vec2, right: Vec2) -> f32 {
    left.x * right.y - left.y * right.x
}

#[cfg(test)]
pub(super) fn cell_polygon(cell: IVec2) -> Vec<Vec2> {
    rectangle_polygon(
        Vec3::new(cell.x as f32 * CELL_SIZE, 0.0, cell.y as f32 * CELL_SIZE),
        Vec2::splat(CELL_SIZE),
        0.0,
    )
}

#[cfg(test)]
pub(super) fn cell_center_2d(cell: IVec2) -> Vec2 {
    Vec2::new(cell.x as f32 * CELL_SIZE, cell.y as f32 * CELL_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polygon_area_uses_shoelace_area() {
        let polygon = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];

        assert_eq!(polygon_area(&polygon), 2.0);
    }

    #[test]
    fn convex_polygon_rejects_concave_shape() {
        let concave = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(1.0, 0.5),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];

        assert!(!is_convex_polygon(&concave));
    }

    #[test]
    fn self_intersection_detects_bow_tie() {
        let bow_tie = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
            Vec2::new(2.0, 0.0),
        ];

        assert!(polygon_has_self_intersection(&bow_tie));
        assert!(!is_convex_polygon(&bow_tie));
    }
}
