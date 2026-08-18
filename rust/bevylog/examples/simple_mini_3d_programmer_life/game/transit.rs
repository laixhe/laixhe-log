//! 城市交通：地铁 / 公交在站点上车后，沿道路网自动行驶到目的区域站点
//! （支路 → 主路 → 市中心 → 主路 → 支路），全程沿路、不穿建筑；
//! 共享单车是骑行状态（BikeMode 资源，步行速度加倍），不在此模块处理。

use bevy::prelude::*;

use super::components::{CommuteChoice, PlayerRoot};
use super::resources::*;

// 行驶速度（米/秒）：地铁快、公交慢
const SUBWAY_SPEED: f32 = 30.0;
const BUS_SPEED: f32 = 15.0;

/// 交通方式的行驶速度（米/秒）：地铁快、公交慢
pub fn ride_speed(mode: CommuteChoice) -> f32 {
    match mode {
        CommuteChoice::Subway => SUBWAY_SPEED,
        CommuteChoice::Bus => BUS_SPEED,
    }
}

// 站点坐标 → 所在道路与主路的入路口。
// 地铁站在五条支路上（各区域门口），公交站在环路四角。
const STOPS: &[(Vec2, Vec2)] = &[
    // (站点, 到主路的入路口)
    (Vec2::new(-42.0, 15.0), Vec2::new(-42.0, 0.0)), // 家地铁站：沿 x=-42 支路到主路
    (Vec2::new(15.0, -42.0), Vec2::new(0.0, -42.0)), // 校园地铁站：沿 z=-42 支路到主路
    (Vec2::new(42.0, -15.0), Vec2::new(42.0, 0.0)),  // 食堂地铁站：沿 x=42 支路到主路
    (Vec2::new(-15.0, 42.0), Vec2::new(0.0, 42.0)),  // 公司地铁站：沿 z=42 支路到主路
    (Vec2::new(42.0, 15.0), Vec2::new(42.0, 0.0)),   // 公园地铁站：沿 x=42 支路到主路
    (Vec2::new(24.0, 24.0), Vec2::new(24.0, 0.0)),   // 公交站（环路四角 → 主路）
    (Vec2::new(24.0, -24.0), Vec2::new(24.0, 0.0)),
    (Vec2::new(-24.0, 24.0), Vec2::new(-24.0, 0.0)),
    (Vec2::new(-24.0, -24.0), Vec2::new(-24.0, 0.0)),
];

// 找到离位置最近的站点 → 返回它所在道路的主路入路口
fn stop_entry(pos: Vec2) -> Vec2 {
    let mut best = Vec2::new(pos.x, 0.0);
    let mut best_d = f32::MAX;
    for (s, e) in STOPS {
        let d = s.distance(pos);
        if d < best_d {
            best_d = d;
            best = *e;
        }
    }
    best
}

// 沿道路网的路点：起点 → 起点入主路口 → 市中心 (0,0) → 目标入主路口 → 目标站点。
// pub(crate)：sim 模块（路线仿真）复用同一套规划。
pub(crate) fn road_waypoints(from: Vec2, target: Location) -> Vec<Vec2> {
    let to = station_pos(target).xz();
    let from_entry = stop_entry(from);
    let to_entry = stop_entry(to);
    let mut pts: Vec<Vec2> = Vec::new();
    for p in [from, from_entry, Vec2::ZERO, to_entry, to] {
        if pts.last().is_none_or(|q| q.distance(p) > 0.5) {
            pts.push(p);
        }
    }
    pts
}

/// 开始乘车：设定方式与目的地。路点在 transit_tick 里按当前站点位置计算。
pub fn start_transit(transit: &mut TransitState, mode: CommuteChoice, target: Location) {
    transit.active = true;
    transit.mode = mode;
    transit.target = target;
    transit.waypoints.clear();
    transit.wp_idx = 0;
    info!(
        "[通勤] 乘坐{}前往{}",
        mode.label(),
        super::scenes::location_name(target)
    );
}

/// 乘车自动行驶：沿道路网路点行驶，全程不穿建筑。
/// 行驶期间接管玩家位置（清空寻路目标，防两套移动逻辑冲突）。
pub fn transit_tick(
    time: Res<Time>,
    mut transit: ResMut<TransitState>,
    mut walk: ResMut<WalkState>,
    mut player: Single<&mut Transform, With<PlayerRoot>>,
    mut toast: ResMut<ToastLog>,
) {
    if !transit.active {
        return;
    }
    // 行驶中锁定寻路，只允许车辆系统移动玩家
    walk.target = None;
    walk.cmd = WalkCmd::Move;
    walk.bob = 0.0;
    walk.path.clear();
    walk.path_target = None;

    // 首次行驶：根据当前站点坐标计算沿道路网的路点
    if transit.waypoints.is_empty() {
        let wps = road_waypoints(player.translation.xz(), transit.target);
        transit.waypoints = wps.iter().map(|p| [p.x, p.y]).collect();
        transit.wp_idx = 0;
        // 完整路径规划日志：起点站 → 入主路 → 市中心 → 转入支路 → 终点站
        let seq = wps
            .iter()
            .map(|p| format!("({:.0},{:.0})", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" → ");
        info!(
            "[通勤] 路径规划（{} → {}）：{}",
            transit.mode.label(),
            super::scenes::location_name(transit.target),
            seq
        );
    }

    let speed = ride_speed(transit.mode);
    let step = speed * time.delta_secs();
    let start_idx = transit.wp_idx;
    let (new_pos, new_idx, done) =
        super::sim::drive_step(&transit.waypoints, transit.wp_idx, player.translation, step);
    player.translation = new_pos;
    transit.wp_idx = new_idx;

    // 途经日志：本次跨过的中间节点（起点站跳过，终点站由到站日志覆盖）
    let len = transit.waypoints.len();
    for i in start_idx..new_idx.min(len) {
        if i == 0 || i >= len - 1 {
            continue;
        }
        let a = Vec2::new(transit.waypoints[i][0], transit.waypoints[i][1]);
        info!(
            "[通勤] {} 途经：{} ({:.0},{:.0}) [{}/{}]",
            transit.mode.label(),
            super::sim::wp_label(&transit.waypoints, i),
            a.x,
            a.y,
            i + 1,
            len
        );
    }

    if done {
        transit.active = false;
        transit.waypoints.clear();
        transit.wp_idx = 0;
        // 到站提示按交通方式区分图标
        let icon = if transit.mode == CommuteChoice::Subway {
            "🚇"
        } else {
            "🚌"
        };
        toast.push(format!("{icon} 到站了，下车吧"));
        info!(
            "[通勤] 到站：{}",
            super::scenes::location_name(transit.target)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 沿道路网的路点必须依次是：起点 → 起点入主路口 → 市中心 → 目标入主路口 → 目标站点
    #[test]
    fn subway_route_goes_through_center() {
        let wps = road_waypoints(Vec2::new(-42.0, 15.0), Location::Office);
        assert_eq!(
            wps,
            vec![
                Vec2::new(-42.0, 15.0),
                Vec2::new(-42.0, 0.0),
                Vec2::ZERO,
                Vec2::new(0.0, 42.0),
                Vec2::new(-15.0, 42.0),
            ]
        );
    }

    #[test]
    fn bus_route_from_ring_corner_reaches_campus() {
        let wps = road_waypoints(Vec2::new(24.0, 24.0), Location::Campus);
        // 公交从环路角 (24,24) 沿 x=24 环路段到主路 (24,0)，经市中心到校园
        assert!(wps.first() == Some(&Vec2::new(24.0, 24.0)));
        assert!(wps.contains(&Vec2::ZERO), "必须经过市中心枢纽");
        assert!(
            wps.last() == Some(&Vec2::new(15.0, -42.0)),
            "终点应为校园站"
        );
    }

    #[test]
    fn same_region_ride_round_trips_safely() {
        // 家 → 家（调试场景）：路点不空、起点终点一致，中间经过市中心
        let wps = road_waypoints(Vec2::new(-42.0, 15.0), Location::Home);
        assert_eq!(*wps.first().unwrap(), Vec2::new(-42.0, 15.0));
        assert_eq!(*wps.last().unwrap(), Vec2::new(-42.0, 15.0));
        assert!(wps.contains(&Vec2::ZERO));
    }
}
