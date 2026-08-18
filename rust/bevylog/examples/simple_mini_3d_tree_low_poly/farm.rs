//! 农场系统：玩家用鼠标依次点出角点（≥3、凸、无自交的多边形），
//! 校验合法后生成半透明覆盖层网格（沿多边形边缘羽化），完工后按网格排布作物。
//! 与普通建筑不同，农场地块是任意凸多边形，几何判定集中在 `building::polygon`。

use bevy::prelude::*;

use crate::{
    building::{
        PlacementIssue, WorldGeometry, is_convex_polygon, point_in_polygon, polygon_area,
        polygon_has_self_intersection, signed_polygon_area,
    },
    math::within_world_bounds,
    terrain::terrain_height,
    types::CELL_SIZE,
};

pub const FARM_CLOSE_DISTANCE: f32 = CELL_SIZE * 0.55;
pub const FARM_ACCESS_OFFSET: f32 = CELL_SIZE * 0.75;
pub const FARM_OVERLAY_SAMPLE_STEP: f32 = CELL_SIZE * 0.35;
pub const FARM_OVERLAY_EDGE_FEATHER: f32 = CELL_SIZE * 0.18;
pub const FARM_OVERLAY_Y_OFFSET: f32 = 0.025;
pub const CROP_HEIGHT: f32 = 0.62;
pub const CROP_RADIUS: f32 = 0.055;
pub const CROP_GRID_SPACING: f32 = CELL_SIZE * 0.55;
pub const CROP_EDGE_PADDING: f32 = CELL_SIZE * 0.18;
pub const CROP_Y_OFFSET: f32 = 0.03;

#[derive(Component, Debug)]
pub struct FarmPlot {
    pub area_cells: f32,
}

#[derive(Component, Debug)]
pub struct CompletedFarmPlot {
    pub area_cells: f32,
}

#[derive(Component, Debug)]
pub struct FarmVisual {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct FarmCrop;

pub fn validate_farm_plan(
    geometry: &WorldGeometry,
    polygon: &[Vec2],
    access_point: Option<Vec3>,
    seed: u64,
    max_slope: f32,
) -> Option<PlacementIssue> {
    if polygon.len() < 3 {
        return Some(PlacementIssue::TooFewPoints);
    }
    if polygon_has_self_intersection(polygon) || !is_convex_polygon(polygon) {
        return Some(PlacementIssue::InvalidShape);
    }

    if let Some(issue) = geometry.placement_issue_for_polygon(polygon, None, true, seed, max_slope)
    {
        return Some(issue);
    }

    let Some(access_point) = access_point else {
        return Some(PlacementIssue::EntranceBlocked);
    };
    geometry.placement_issue_for_polygon(polygon, Some(access_point), true, seed, max_slope)
}

pub fn farm_area_cells(polygon: &[Vec2]) -> f32 {
    polygon_area(polygon) / (CELL_SIZE * CELL_SIZE)
}

pub fn farm_build_seconds(polygon: &[Vec2]) -> f32 {
    (farm_area_cells(polygon) * 0.25).max(1.0)
}

pub fn farm_origin(seed: u64, polygon: &[Vec2]) -> Vec3 {
    let center = polygon_centroid(polygon);
    Vec3::new(center.x, terrain_height(seed, center.x, center.y), center.y)
}

// 为农场找一个人口点：遍历每条边，取中点向外偏移 FARM_ACCESS_OFFSET，
// 边越长越优先（入口更宽敞），并确保落在世界边界内。
pub fn farm_access_point(seed: u64, polygon: &[Vec2]) -> Option<Vec3> {
    if polygon.len() < 3 {
        return None;
    }

    let signed_area = signed_polygon_area(polygon);
    if signed_area.abs() <= 0.0001 {
        return None;
    }

    let mut candidates: Vec<(f32, Vec2)> = Vec::new();
    for index in 0..polygon.len() {
        let start = polygon[index];
        let end = polygon[(index + 1) % polygon.len()];
        let edge = end - start;
        let length = edge.length();
        if length <= 0.0001 {
            continue;
        }

        let outward = if signed_area > 0.0 {
            Vec2::new(edge.y, -edge.x)
        } else {
            Vec2::new(-edge.y, edge.x)
        }
        .normalize();
        let candidate = (start + end) * 0.5 + outward * FARM_ACCESS_OFFSET;
        candidates.push((length, candidate));
    }

    candidates.sort_by(|(left, _), (right, _)| right.total_cmp(left));
    candidates
        .into_iter()
        .map(|(_, point)| point)
        .find(|point| within_world_bounds(*point))
        .map(|point| Vec3::new(point.x, terrain_height(seed, point.x, point.y), point.y))
}

// 覆盖层网格：把多边形按“质心 + 每条边”拆成三角形扇，每个三角形按
// FARM_OVERLAY_SAMPLE_STEP 细分出密集顶点，顶点颜色用“离边距离”做羽化渐变，
// 得到一块紧贴地形的半透明土层（视觉上示意“这是可耕作的农地”）。
pub fn farm_overlay_mesh(seed: u64, polygon: &[Vec2], origin: Vec3) -> Mesh {
    let signed_area = signed_polygon_area(polygon);
    if polygon.len() < 3 || signed_area.abs() <= 0.0001 {
        return empty_overlay_mesh();
    }

    let center = polygon_centroid(polygon);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    for index in 0..polygon.len() {
        let current = polygon[index];
        let next = polygon[(index + 1) % polygon.len()];
        let (edge_a, edge_b) = if signed_area > 0.0 {
            (next, current)
        } else {
            (current, next)
        };
        append_overlay_triangle(
            seed,
            polygon,
            origin,
            center,
            edge_a,
            edge_b,
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut colors,
            &mut indices,
        );
    }

    Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(bevy::mesh::Indices::U32(indices))
}

pub fn farm_crop_positions(seed: u64, polygon: &[Vec2], origin: Vec3) -> Vec<Vec3> {
    if polygon.len() < 3 {
        return Vec::new();
    }

    let Some((min, max)) = polygon_bounds(polygon) else {
        return Vec::new();
    };
    let mut positions = Vec::new();
    let mut z = aligned_grid_start(min.y, CROP_GRID_SPACING);

    while z <= max.y {
        let mut x = aligned_grid_start(min.x, CROP_GRID_SPACING);
        while x <= max.x {
            let point = Vec2::new(x, z);
            if point_in_polygon(point, polygon)
                && distance_to_polygon_edge(point, polygon) >= CROP_EDGE_PADDING
            {
                let height = terrain_height(seed, x, z);
                positions.push(Vec3::new(
                    x - origin.x,
                    height - origin.y + CROP_Y_OFFSET + CROP_HEIGHT * 0.5,
                    z - origin.z,
                ));
            }
            x += CROP_GRID_SPACING;
        }
        z += CROP_GRID_SPACING;
    }

    positions
}

pub fn polygon_centroid(polygon: &[Vec2]) -> Vec2 {
    if polygon.is_empty() {
        return Vec2::ZERO;
    }

    polygon
        .iter()
        .copied()
        .fold(Vec2::ZERO, |sum, point| sum + point)
        / polygon.len() as f32
}

fn polygon_bounds(polygon: &[Vec2]) -> Option<(Vec2, Vec2)> {
    let first = *polygon.first()?;
    let mut min = first;
    let mut max = first;

    for point in polygon.iter().skip(1) {
        min = min.min(*point);
        max = max.max(*point);
    }

    Some((min, max))
}

fn aligned_grid_start(value: f32, spacing: f32) -> f32 {
    (value / spacing).ceil() * spacing
}

// 单个三角形扇的细分：按行生成顶点（第 i 行有 subdivisions-i+1 个），
// 顶点坐标 = 质心×中心权重 + 两条边×边权重 的凸组合，保证全部落在多边形内。
fn append_overlay_triangle(
    seed: u64,
    polygon: &[Vec2],
    origin: Vec3,
    center: Vec2,
    edge_a: Vec2,
    edge_b: Vec2,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
) {
    let subdivisions = overlay_triangle_subdivisions(center, edge_a, edge_b);
    let mut row_starts = Vec::with_capacity(subdivisions as usize + 1);
    let n = subdivisions as f32;

    for i in 0..=subdivisions {
        row_starts.push(positions.len() as u32);
        for j in 0..=(subdivisions - i) {
            let edge_a_weight = i as f32 / n;
            let edge_b_weight = j as f32 / n;
            let center_weight = 1.0 - edge_a_weight - edge_b_weight;
            let point = center * center_weight + edge_a * edge_a_weight + edge_b * edge_b_weight;
            push_overlay_vertex(
                seed, polygon, origin, point, positions, normals, uvs, colors,
            );
        }
    }

    for i in 0..subdivisions {
        for j in 0..(subdivisions - i) {
            let a = overlay_vertex_index(&row_starts, i, j);
            let b = overlay_vertex_index(&row_starts, i + 1, j);
            let c = overlay_vertex_index(&row_starts, i, j + 1);
            indices.extend([a, b, c]);

            if j < subdivisions - i - 1 {
                let d = overlay_vertex_index(&row_starts, i + 1, j + 1);
                indices.extend([b, d, c]);
            }
        }
    }
}

fn overlay_triangle_subdivisions(center: Vec2, edge_a: Vec2, edge_b: Vec2) -> u32 {
    let max_edge = center
        .distance(edge_a)
        .max(center.distance(edge_b))
        .max(edge_a.distance(edge_b));
    (max_edge / FARM_OVERLAY_SAMPLE_STEP).ceil().max(1.0) as u32
}

fn overlay_vertex_index(row_starts: &[u32], row: u32, column: u32) -> u32 {
    row_starts[row as usize] + column
}

fn push_overlay_vertex(
    seed: u64,
    polygon: &[Vec2],
    origin: Vec3,
    point: Vec2,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
) {
    let height = terrain_height(seed, point.x, point.y);
    positions.push([
        point.x - origin.x,
        height - origin.y + FARM_OVERLAY_Y_OFFSET,
        point.y - origin.z,
    ]);
    normals.push(farm_overlay_normal(seed, point.x, point.y));
    uvs.push([point.x * 0.25, point.y * 0.25]);
    colors.push([1.0, 1.0, 1.0, farm_overlay_alpha(point, polygon)]);
}

fn farm_overlay_normal(seed: u64, x: f32, z: f32) -> [f32; 3] {
    let sample_dist = FARM_OVERLAY_SAMPLE_STEP;
    let left = terrain_height(seed, x - sample_dist, z);
    let right = terrain_height(seed, x + sample_dist, z);
    let down = terrain_height(seed, x, z - sample_dist);
    let up = terrain_height(seed, x, z + sample_dist);
    Vec3::new(left - right, sample_dist * 2.0, down - up)
        .normalize_or_zero()
        .to_array()
}

fn farm_overlay_alpha(point: Vec2, polygon: &[Vec2]) -> f32 {
    (distance_to_polygon_edge(point, polygon) / FARM_OVERLAY_EDGE_FEATHER).clamp(0.0, 1.0)
}

fn distance_to_polygon_edge(point: Vec2, polygon: &[Vec2]) -> f32 {
    polygon
        .iter()
        .enumerate()
        .map(|(index, start)| {
            let end = polygon[(index + 1) % polygon.len()];
            distance_to_segment(point, *start, end)
        })
        .fold(f32::MAX, f32::min)
}

fn distance_to_segment(point: Vec2, start: Vec2, end: Vec2) -> f32 {
    let segment = end - start;
    let length_squared = segment.length_squared();
    if length_squared <= 0.0001 {
        return point.distance(start);
    }
    let t = ((point - start).dot(segment) / length_squared).clamp(0.0, 1.0);
    point.distance(start + segment * t)
}

fn empty_overlay_mesh() -> Mesh {
    Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, Vec::<[f32; 3]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new())
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<[f32; 4]>::new())
    .with_inserted_indices(bevy::mesh::Indices::U32(Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{building::footprint_polygon, terrain::DEFAULT_TERRAIN_SEED, types::BuildingKind};
    use bevy::mesh::VertexAttributeValues;

    const SEED: u64 = DEFAULT_TERRAIN_SEED;

    fn square() -> Vec<Vec2> {
        vec![
            Vec2::new(2.0, 2.0),
            Vec2::new(4.0, 2.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(2.0, 4.0),
        ]
    }

    #[test]
    fn farm_area_uses_cell_area() {
        assert_eq!(farm_area_cells(&square()), 4.0);
    }

    #[test]
    fn farm_validation_accepts_convex_polygon() {
        let geometry = WorldGeometry::default();
        let polygon = square();

        assert_eq!(
            validate_farm_plan(
                &geometry,
                &polygon,
                farm_access_point(SEED, &polygon),
                SEED,
                10.0
            ),
            None
        );
    }

    #[test]
    fn farm_validation_rejects_bad_shapes_and_too_few_points() {
        let geometry = WorldGeometry::default();
        let concave = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(1.0, 0.5),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];

        assert_eq!(
            validate_farm_plan(&geometry, &square()[..2], None, SEED, 10.0),
            Some(PlacementIssue::TooFewPoints)
        );
        assert_eq!(
            validate_farm_plan(
                &geometry,
                &concave,
                farm_access_point(SEED, &concave),
                SEED,
                10.0
            ),
            Some(PlacementIssue::InvalidShape)
        );
    }

    #[test]
    fn farm_validation_rejects_overlap_bounds_and_slope() {
        let mut world = World::new();
        let blocker = world.spawn_empty().id();
        let mut geometry = WorldGeometry::default();
        let building = footprint_polygon(
            BuildingKind::Storage,
            Vec3::new(3.0, 0.0, 3.0),
            BuildingKind::Storage.definition().size,
            0.0,
        );
        geometry.occupy_polygon(building, blocker, false);

        let overlap = square();
        assert_eq!(
            validate_farm_plan(
                &geometry,
                &overlap,
                farm_access_point(SEED, &overlap),
                SEED,
                10.0
            ),
            Some(PlacementIssue::Occupied)
        );

        let out_of_bounds = vec![
            Vec2::new(1000.0, 1000.0),
            Vec2::new(1001.0, 1000.0),
            Vec2::new(1001.0, 1001.0),
            Vec2::new(1000.0, 1001.0),
        ];
        assert_eq!(
            validate_farm_plan(
                &WorldGeometry::default(),
                &out_of_bounds,
                farm_access_point(SEED, &out_of_bounds),
                SEED,
                10.0
            ),
            Some(PlacementIssue::OutOfBounds)
        );

        let open = vec![
            Vec2::new(20.0, 20.0),
            Vec2::new(22.0, 20.0),
            Vec2::new(22.0, 22.0),
            Vec2::new(20.0, 22.0),
        ];
        assert_eq!(
            validate_farm_plan(
                &WorldGeometry::default(),
                &open,
                farm_access_point(SEED, &open),
                SEED,
                0.0
            ),
            Some(PlacementIssue::TooSteep)
        );
    }

    #[test]
    fn occupied_farm_polygon_blocks_navigation_and_overlap() {
        let mut world = World::new();
        let farm = world.spawn_empty().id();
        let mut geometry = WorldGeometry::default();
        let polygon = square();

        geometry.occupy_polygon(polygon.clone(), farm, false);

        assert!(!geometry.is_walkable_point(Vec3::new(3.0, 0.0, 3.0)));
        assert_eq!(
            geometry.placement_issue_for_polygon(&polygon, None, true, SEED, 10.0),
            Some(PlacementIssue::Occupied)
        );
    }

    #[test]
    fn farm_blueprint_needs_no_materials() {
        let blueprint = crate::building::Blueprint {
            kind: crate::types::ConstructionKind::Farm,
            required_wood: 0,
            delivered_wood: 0,
            progress: 1.0,
            build_seconds: 1.0,
        };

        assert!(blueprint.has_materials());
        assert!(blueprint.is_complete());
    }

    #[test]
    fn farm_overlay_mesh_samples_finite_terrain_positions_and_normals() {
        let polygon = vec![
            Vec2::new(20.0, 20.0),
            Vec2::new(23.0, 20.5),
            Vec2::new(22.5, 23.0),
            Vec2::new(20.0, 22.0),
        ];
        let origin = farm_origin(SEED, &polygon);
        let mesh = farm_overlay_mesh(SEED, &polygon, origin);

        let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
            VertexAttributeValues::Float32x3(values) => values,
            values => panic!("unexpected position attribute: {values:?}"),
        };
        let normals = match mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap() {
            VertexAttributeValues::Float32x3(values) => values,
            values => panic!("unexpected normal attribute: {values:?}"),
        };

        assert!(!positions.is_empty());
        assert_eq!(positions.len(), normals.len());
        assert!(positions.iter().flatten().all(|value| value.is_finite()));
        assert!(normals.iter().flatten().all(|value| value.is_finite()));
        assert!(normals.iter().any(|normal| normal[1] > 0.5));
    }

    #[test]
    fn farm_overlay_mesh_feathers_alpha_at_polygon_edge() {
        let polygon = square();
        let origin = farm_origin(SEED, &polygon);
        let mesh = farm_overlay_mesh(SEED, &polygon, origin);

        let colors = match mesh.attribute(Mesh::ATTRIBUTE_COLOR).unwrap() {
            VertexAttributeValues::Float32x4(values) => values,
            values => panic!("unexpected color attribute: {values:?}"),
        };

        let min_alpha = colors.iter().map(|color| color[3]).fold(f32::MAX, f32::min);
        let max_alpha = colors.iter().map(|color| color[3]).fold(f32::MIN, f32::max);

        assert!(min_alpha <= 0.01, "edge alpha was {min_alpha}");
        assert!(max_alpha >= 0.99, "interior alpha was {max_alpha}");
    }

    #[test]
    fn farm_crop_positions_stay_inside_polygon_with_edge_padding() {
        let polygon = vec![
            Vec2::new(20.0, 20.0),
            Vec2::new(23.0, 20.0),
            Vec2::new(23.0, 23.0),
            Vec2::new(20.0, 23.0),
        ];
        let origin = farm_origin(SEED, &polygon);
        let positions = farm_crop_positions(SEED, &polygon, origin);

        assert!(!positions.is_empty());
        for position in positions {
            assert!(position.is_finite());

            let point = Vec2::new(position.x + origin.x, position.z + origin.z);
            assert!(point_in_polygon(point, &polygon));
            assert!(distance_to_polygon_edge(point, &polygon) >= CROP_EDGE_PADDING - 0.0001);

            let expected_y = terrain_height(SEED, point.x, point.y) - origin.y
                + CROP_Y_OFFSET
                + CROP_HEIGHT * 0.5;
            assert!(
                (position.y - expected_y).abs() < 0.0001,
                "expected crop y {expected_y}, got {}",
                position.y
            );
        }
    }
}
