//! 碰撞与寻路：墙 / 建筑 / 家具等实体挂上 `Solid` 组件后，
//! 构建模块会汇总成碰撞盒列表 + 阻挡网格。
//! 玩家行走按「网格 A* 找绕行路径 + 每帧圆盒推出」：不会穿墙穿建筑，贴墙还能滑动；
//! NPC 巡逻同样做推出，不穿建筑。
//! 乘车（地铁 / 公交）是车辆模式，不做碰撞，保持直线行驶。

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use bevy::prelude::*;

use super::components::{PlayerRoot, Solid};
use super::resources::{TransitState, WORLD_HALF};

// 玩家半径：碰撞推出与网格膨胀都按它计算
pub const PLAYER_RADIUS: f32 = 0.45;

// ==================== 碰撞地图 ====================
// boxes = 所有 Solid 的 (中心, 半宽高)；grid 是 1×1 格阻挡网格（已膨胀玩家半径）。
#[derive(Resource, Default)]
pub struct CollisionMap {
    pub boxes: Vec<(Vec2, Vec2)>,
    pub cell: f32,
    pub w: usize,
    pub h: usize,
    pub grid: Vec<u8>, // 1 = 阻挡
}

/// 由碰撞盒列表构建碰撞地图（阻挡网格 + boxes）。真实游戏与模拟测试共用，
/// 保证测试里的寻路结果与真实游戏一致。
pub fn make_map(boxes: Vec<(Vec2, Vec2)>) -> CollisionMap {
    let cell = 1.0;
    let w = (WORLD_HALF * 2.0 / cell) as usize;
    let h = w;
    let mut grid = vec![0u8; w * h];
    // AABB 膨胀玩家半径后标记覆盖的格
    for (c, half) in &boxes {
        let min = c - *half - Vec2::splat(PLAYER_RADIUS);
        let max = c + *half + Vec2::splat(PLAYER_RADIUS);
        let x0 = (((min.x + WORLD_HALF) / cell).floor().max(0.0) as usize).min(w - 1);
        let x1 = (((max.x + WORLD_HALF) / cell).floor().max(0.0) as usize).min(w - 1);
        let y0 = (((min.y + WORLD_HALF) / cell).floor().max(0.0) as usize).min(h - 1);
        let y1 = (((max.y + WORLD_HALF) / cell).floor().max(0.0) as usize).min(h - 1);
        for gy in y0..=y1 {
            for gx in x0..=x1 {
                grid[gy * w + gx] = 1;
            }
        }
    }
    CollisionMap {
        boxes,
        cell,
        w,
        h,
        grid,
    }
}

/// 进入游戏、城市搭建完后执行一次：收集所有 Solid 实体，构建碰撞盒列表与阻挡网格。
pub fn build_map(solids: Query<(&Transform, &Solid)>, mut map: ResMut<CollisionMap>) {
    let boxes = solids
        .iter()
        .filter(|(_, s)| s.bottom < 1.0) // 高空装饰（底部 > 1.0）只挡视线不挡脚，跳过
        .map(|(tf, s)| (tf.translation.xz(), s.half))
        .collect();
    *map = make_map(boxes);
    info!(
        "[碰撞] 碰撞盒 {} 个，阻挡网格 {}×{}",
        map.boxes.len(),
        map.w,
        map.h
    );
}

// 世界坐标 → 网格下标
fn grid_idx(map: &CollisionMap, p: Vec2) -> Option<usize> {
    let gx = ((p.x + WORLD_HALF) / map.cell).floor() as isize;
    let gy = ((p.y + WORLD_HALF) / map.cell).floor() as isize;
    if (0..map.w as isize).contains(&gx) && (0..map.h as isize).contains(&gy) {
        Some(gy as usize * map.w + gx as usize)
    } else {
        None
    }
}

// 线段是否一路畅通（用 DDA 采样阻挡网格）
fn line_clear(map: &CollisionMap, a: Vec2, b: Vec2) -> bool {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    // 按半格采样：45° 斜线每步间距约 0.7 格，避免跳过单格阻挡的对角
    let steps = (((dx.abs().max(dy.abs())) / (map.cell * 0.5)).ceil() as usize).max(1) + 1;
    for s in 0..=steps {
        let t = s as f32 / steps as f32;
        let p = a.lerp(b, t);
        if let Some(i) = grid_idx(map, p) {
            if map.grid[i] == 1 {
                return false;
            }
        }
    }
    true
}

// A* 节点：堆里存 (代价, 下标)。BinaryHeap 是大顶堆，代价取负实现小顶堆。
#[derive(PartialEq)]
struct Node(f32, usize);

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

/// 网格 A* 寻路：返回绕开建筑的路径点（世界坐标 XZ）。
/// 找不到完整路径（目标在建筑内等）返回 None，调用方退回直线行走，由碰撞挡住。
pub fn find_path(map: &CollisionMap, from: Vec2, to: Vec2) -> Option<Vec<Vec2>> {
    if map.grid.is_empty() {
        return None;
    }
    let start = grid_idx(map, from)?;
    let goal = grid_idx(map, to)?;
    // 目标在墙里/建筑里 → 不可达
    if map.grid[goal] == 1 {
        return None;
    }
    if start == goal {
        return Some(vec![to]);
    }

    let mut g = vec![f32::MAX; map.grid.len()];
    let mut prev = vec![usize::MAX; map.grid.len()];
    g[start] = 0.0;
    let mut heap = BinaryHeap::new();
    heap.push(Node(0.0, start));

    let goal_cx = (goal % map.w) as f32 + 0.5;
    let goal_cy = (goal / map.w) as f32 + 0.5;
    let goal_x = goal_cx * map.cell - WORLD_HALF;
    let goal_y = goal_cy * map.cell - WORLD_HALF;

    while let Some(Node(_, cur)) = heap.pop() {
        if cur == goal {
            break;
        }
        let cx = cur % map.w;
        let cy = cur / map.w;
        for (dx, dy) in [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)] {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx < 0 || ny < 0 || nx as usize >= map.w || ny as usize >= map.h {
                continue;
            }
            let ni = ny as usize * map.w + nx as usize;
            if map.grid[ni] == 1 {
                continue;
            }
            let cost = g[cur] + 1.0;
            if cost < g[ni] {
                g[ni] = cost;
                prev[ni] = cur;
                let nwx = (nx as f32 + 0.5) * map.cell - WORLD_HALF;
                let nwy = (ny as f32 + 0.5) * map.cell - WORLD_HALF;
                let h = (nwx - goal_x).abs() + (nwy - goal_y).abs();
                heap.push(Node(-(cost + h), ni));
            }
        }
    }
    if g[goal] == f32::MAX {
        return None;
    }

    // 回溯格子中心点（不含起点，含目标格）
    let mut cells = Vec::new();
    let mut cur = goal;
    while cur != usize::MAX && cur != start {
        let wx = ((cur % map.w) as f32 + 0.5) * map.cell - WORLD_HALF;
        let wy = ((cur / map.w) as f32 + 0.5) * map.cell - WORLD_HALF;
        cells.push(Vec2::new(wx, wy));
        cur = prev[cur];
    }
    cells.reverse();

    // 贪心拉直：能直达的中间拐点全部去掉，得到贴近直线的行走路径
    let mut pts = vec![from];
    pts.extend_from_slice(&cells);
    pts.push(to);
    let mut smooth: Vec<Vec2> = Vec::new();
    smooth.push(from);
    let mut i = 0;
    while i < pts.len() - 1 {
        let mut j = i + 1;
        while j < pts.len() && line_clear(map, pts[i], pts[j]) {
            j += 1;
        }
        let reach = j - 1;
        if reach > i && pts[reach] != pts[i] {
            smooth.push(pts[reach]);
            i = reach;
        } else {
            // 紧挨着不可直达（极端情况）：退而求其次，推下一步
            if pts[i + 1] != pts[i] {
                smooth.push(pts[i + 1]);
            }
            i += 1;
        }
    }
    if *smooth.last().unwrap() != to {
        smooth.push(to);
    }
    Some(smooth)
}

/// 圆形玩家 vs 所有碰撞盒：把位置推出碰撞盒（贴墙滑行）。
pub fn resolve(pos: &mut Vec3, boxes: &[(Vec2, Vec2)]) {
    for _ in 0..2 {
        for (c, half) in boxes {
            let p = Vec2::new(pos.x, pos.z);
            let closest = Vec2::new(
                p.x.clamp(c.x - half.x, c.x + half.x),
                p.y.clamp(c.y - half.y, c.y + half.y),
            );
            let d = p - closest;
            let dist = d.length();
            if dist < PLAYER_RADIUS {
                if dist > 1e-5 {
                    let push = d / dist * (PLAYER_RADIUS - dist);
                    pos.x += push.x;
                    pos.z += push.y;
                } else {
                    // 圆心在盒内：沿穿透最小的轴推出
                    let l = p - c;
                    let ox = half.x + PLAYER_RADIUS - l.x.abs();
                    let oz = half.y + PLAYER_RADIUS - l.y.abs();
                    if ox < oz {
                        pos.x += if l.x >= 0.0 { ox } else { -ox };
                    } else {
                        pos.z += if l.y >= 0.0 { oz } else { -oz };
                    }
                }
            }
        }
    }
}

/// 每帧把主角推出碰撞盒（乘车行驶中除外，车辆不碰撞）。
pub fn resolve_player(
    map: Res<CollisionMap>,
    transit: Res<TransitState>,
    mut player: Single<&mut Transform, With<PlayerRoot>>,
) {
    if transit.active {
        return;
    }
    resolve(&mut player.translation, &map.boxes);
}

#[cfg(test)]
mod tests {
    use super::*;

    // 手工构造一张只有一堵横墙的地图（复用 make_map，与真实构建逻辑一致）
    fn wall_map(wall_center: Vec2) -> CollisionMap {
        make_map(vec![(wall_center, Vec2::new(1.0, 0.5))])
    }

    #[test]
    fn path_goes_around_wall_not_through() {
        // 横墙横亘在 (0,0)，从西到东不能直穿
        let map = wall_map(Vec2::new(0.0, 0.0));
        let path =
            find_path(&map, Vec2::new(-5.0, 0.0), Vec2::new(5.0, 0.0)).expect("应找到绕行路径");
        assert!(path.len() >= 3, "绕行至少需要拐点，实际 {}", path.len());
        // 所有路径点都必须落在可行走格上
        for p in &path {
            let i = grid_idx(&map, *p).expect("路径点在地图内");
            assert_eq!(map.grid[i], 0, "路径点 ({p:?}) 不应落在阻挡格");
        }
        // 起点终点都在
        assert_eq!(*path.first().unwrap(), Vec2::new(-5.0, 0.0));
        assert_eq!(*path.last().unwrap(), Vec2::new(5.0, 0.0));
    }

    #[test]
    fn path_to_blocked_target_returns_none() {
        let map = wall_map(Vec2::new(0.0, 0.0));
        // 目标正好点在墙的阻挡格内 → 判定不可达
        assert!(find_path(&map, Vec2::new(-5.0, 0.0), Vec2::new(0.0, 0.0)).is_none());
    }

    #[test]
    fn resolve_pushes_player_out_of_wall() {
        let mut pos = Vec3::new(0.0, 0.0, 0.0); // 圆心在墙内
        let boxes = vec![(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.5))];
        resolve(&mut pos, &boxes);
        let p = Vec2::new(pos.x, pos.z);
        let dist = p.distance(Vec2::ZERO);
        // 被推出后应站在墙外缘（距离 == 半高 + 玩家半径 = 0.95）
        assert!((dist - 0.95).abs() < 0.01, "推出后应在墙外缘，实际 {dist}");
    }

    #[test]
    fn line_of_sight_is_blocked_by_wall() {
        let map = wall_map(Vec2::new(0.0, 0.0));
        assert!(
            !line_clear(&map, Vec2::new(-3.0, 0.0), Vec2::new(3.0, 0.0)),
            "横穿墙应不可直达"
        );
        assert!(
            line_clear(&map, Vec2::new(-3.0, 5.0), Vec2::new(3.0, 5.0)),
            "墙外通行无阻"
        );
    }
}
