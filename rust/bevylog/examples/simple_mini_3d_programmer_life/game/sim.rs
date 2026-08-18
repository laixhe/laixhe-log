//! 交通 / 移动仿真模块：把「沿路点行驶（地铁 / 公交）」和「沿 A* 路径步行」
//! 统一成可复用的逐帧仿真（drive_step / simulate_waypoints / simulate_walk），
//! 并附带四个场景的完整流程模拟测试（地铁 / 公交 / 步行 / 共享单车）。
//!
//! 后续扩展新交通方式（驾车、共享单车远程骑行等）时：
//! - 沿固定路线行驶 → 复用 `road_waypoints` 规划 + `simulate_waypoints` 驱动；
//! - 自由寻路移动 → 复用 `find_path` + `simulate_walk` 驱动。
//!
//! 日志格式与真实游戏（transit_tick / move_player）保持一致。

use bevy::prelude::*;

use super::collision::{CollisionMap, find_path, make_map};
use super::resources::{BIKE_SPEED_MULT, PLAYER_SPEED};

/// 沿路点行驶一步：返回 (新位置, 新 wp_idx, 是否到站)。
/// wp_idx 指向当前目标路点；到达后递增，>= len 表示已到终点站。
/// 真实游戏 transit_tick 与仿真共用，保证日志与行驶一致。
pub fn drive_step(
    waypoints: &[[f32; 2]],
    mut wp_idx: usize,
    mut pos: Vec3,
    mut step: f32,
) -> (Vec3, usize, bool) {
    while let Some(&[wx, wz]) = waypoints.get(wp_idx) {
        let wpos = Vec3::new(wx, 0.0, wz);
        let delta = wpos - pos;
        let dist = Vec2::new(delta.x, delta.z).length();
        if dist <= 0.8 {
            // 已在当前路点：前进到下一段
            wp_idx += 1;
            if wp_idx >= waypoints.len() {
                return (wpos, wp_idx, true);
            }
            continue;
        }
        if step >= dist {
            pos = wpos;
            step -= dist;
            continue;
        }
        let dir = Vec3::new(delta.x, 0.0, delta.z).normalize();
        pos += dir * step;
        return (pos, wp_idx, false);
    }
    (pos, wp_idx, false)
}

/// 路点在序列中的类型：进入主路 / 市中心 / 转入支路（终点站由到站日志覆盖）
pub fn wp_label(waypoints: &[[f32; 2]], idx: usize) -> &'static str {
    let arrived = Vec2::new(waypoints[idx][0], waypoints[idx][1]);
    let passed_center = waypoints[..idx]
        .iter()
        .any(|p| Vec2::new(p[0], p[1]).distance(Vec2::ZERO) < 0.5);
    if arrived.distance(Vec2::ZERO) < 0.5 {
        "市中心"
    } else if passed_center {
        "转入支路"
    } else {
        "进入主路"
    }
}

/// 一次仿真的结果
#[derive(Debug)]
#[allow(dead_code)] // 预留：供后续扩展其他交通方式复用的仿真 API
pub struct SimResult {
    pub frames: u32,
    pub total_dist: f32,
    pub end_pos: Vec2,
    pub arrived: bool,
}

/// 沿路点逐帧行驶并打印途经日志（tag 如 "地铁" / "公交"）。
/// 路径规划日志与到站日志由调用方打印，便于带上起终点名称。
#[allow(dead_code)] // 预留：供后续扩展其他交通方式复用的仿真 API
pub fn simulate_waypoints(tag: &str, waypoints: &[[f32; 2]], from: Vec2, speed: f32) -> SimResult {
    let dt = 1.0 / 60.0;
    let step = speed * dt;
    let mut pos = Vec3::new(from.x, 0.0, from.y);
    let mut wp_idx = 0usize;
    let mut done = false;
    let mut frames = 0u32;
    let mut total = 0.0f32;
    let len = waypoints.len();
    while !done && frames < 60 * 60 {
        let start_idx = wp_idx;
        let before = pos;
        let (p, i, d) = drive_step(waypoints, wp_idx, pos, step);
        pos = p;
        wp_idx = i;
        done = d;
        total += pos.distance(before);
        // 途经日志：本次跨过的中间节点（起点站跳过，终点站由到站日志覆盖）
        for k in start_idx..wp_idx.min(len) {
            if k == 0 || k >= len - 1 {
                continue;
            }
            let a = Vec2::new(waypoints[k][0], waypoints[k][1]);
            println!(
                "[{tag}] 途经：{} ({:.0},{:.0}) [{}/{}]",
                wp_label(waypoints, k),
                a.x,
                a.y,
                k + 1,
                len
            );
        }
        frames += 1;
    }
    SimResult {
        frames,
        total_dist: total,
        end_pos: pos.xz(),
        arrived: done,
    }
}

/// 沿 A* 路径逐段直线移动（与 move_player 一致：60fps，速度可参数化）。
/// 返回 (路径点列表, 仿真结果)；路径规划日志在此打印。
#[allow(dead_code)] // 预留：供后续扩展其他交通方式复用的仿真 API
pub fn simulate_path(
    map: &CollisionMap,
    from: Vec2,
    to: Vec2,
    speed: f32,
    tag: &str,
) -> (Vec<Vec2>, SimResult) {
    let path = find_path(map, from, to).unwrap_or_default();
    let seq = path
        .iter()
        .map(|p| format!("({:.0},{:.0})", p.x, p.y))
        .collect::<Vec<_>>()
        .join(" → ");
    println!("[{tag}] 路径规划：{seq}");

    let dt = 1.0 / 60.0;
    let mut frames = 0u32;
    let mut total = 0.0f32;
    let mut pos = from;
    for w in path.iter().skip(1) {
        let d = pos.distance(*w);
        total += d;
        frames += (d / (speed * dt)).round() as u32;
        pos = *w;
    }
    println!(
        "[{tag}] 到达（全程 {total:.0}m，{:.1}s）",
        frames as f32 * dt
    );
    (
        path,
        SimResult {
            frames,
            total_dist: total,
            end_pos: pos,
            arrived: true,
        },
    )
}

/// 步行仿真（PLAYER_SPEED = 7.5 m/s）
#[allow(dead_code)] // 预留：供后续扩展其他交通方式复用的仿真 API
pub fn simulate_walk(map: &CollisionMap, from: Vec2, to: Vec2) -> (Vec<Vec2>, SimResult) {
    simulate_path(map, from, to, PLAYER_SPEED, "步行")
}

/// 共享单车骑行仿真（速度 = 步行 × BIKE_SPEED_MULT，与 BikeMode 一致）
#[allow(dead_code)] // 预留：供后续扩展其他交通方式复用的仿真 API
pub fn simulate_bike(map: &CollisionMap, from: Vec2, to: Vec2) -> (Vec<Vec2>, SimResult) {
    simulate_path(map, from, to, PLAYER_SPEED * BIKE_SPEED_MULT, "骑行")
}

/// 仿真用的城市局部地图：家 U 形围墙（南面留门洞）+ 公司南墙（与 scenes.rs 布局一致）。
/// 供 sim / dispatch 等仿真测试共用。
#[allow(dead_code)] // 仿真测试辅助
pub(crate) fn city_walk_map() -> CollisionMap {
    make_map(vec![
        (Vec2::new(-42.0, 35.4), Vec2::new(10.0, 0.125)), // 家北墙
        (Vec2::new(-52.0, 28.0), Vec2::new(0.125, 7.5)),  // 家西墙
        (Vec2::new(-32.0, 28.0), Vec2::new(0.125, 7.5)),  // 家东墙
        (Vec2::new(-48.25, 20.6), Vec2::new(3.75, 0.125)), // 家南墙左段
        (Vec2::new(-35.75, 20.6), Vec2::new(3.75, 0.125)), // 家南墙右段
        (Vec2::new(-28.0, 32.5), Vec2::new(12.0, 0.15)),  // 公司南墙
    ])
}

#[cfg(test)]
mod tests {
    use super::super::components::CommuteChoice;
    use super::super::resources::{Location, station_pos};
    use super::super::transit::{ride_speed, road_waypoints};
    use super::*;

    // 完整模拟：公交从环路角公交站 (24,24) 出发 → 家。
    #[test]
    fn simulate_bus_from_ring_corner_to_home() {
        let from = Vec2::new(24.0, 24.0);
        let wps = road_waypoints(from, Location::Home);
        let waypoints: Vec<[f32; 2]> = wps.iter().map(|p| [p.x, p.y]).collect();
        let seq = wps
            .iter()
            .map(|p| format!("({:.0},{:.0})", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" → ");
        println!("[通勤] 路径规划（公交 → 家）：{seq}");

        let speed = ride_speed(CommuteChoice::Bus);
        let r = simulate_waypoints("公交", &waypoints, from, speed);
        println!(
            "[通勤] 到站：家（{} 帧，{:.1}s）",
            r.frames,
            r.frames as f32 / 60.0
        );

        assert!(r.arrived, "公交应正常到站");
        assert_eq!(r.end_pos, station_pos(Location::Home).xz());
        assert!(
            waypoints
                .iter()
                .any(|p| Vec2::new(p[0], p[1]).distance(Vec2::ZERO) < 0.5),
            "路径必须经过市中心"
        );
        // 全程约 105 米（24+24+42+15）/ 公交 15 m/s ≈ 7 秒；路点提前 snap 允许 ±0.2s
        let expect = (105.0 / ride_speed(CommuteChoice::Bus) * 60.0).round() as u32;
        assert!(
            r.frames.abs_diff(expect) <= 12,
            "行驶帧数应约为 {expect}，实际 {}",
            r.frames
        );
    }

    // 完整模拟：地铁从家站 (-42,15) 出发 → 公司站。
    #[test]
    fn simulate_subway_from_home_to_office() {
        let from = Vec2::new(-42.0, 15.0);
        let wps = road_waypoints(from, Location::Office);
        let waypoints: Vec<[f32; 2]> = wps.iter().map(|p| [p.x, p.y]).collect();
        let seq = wps
            .iter()
            .map(|p| format!("({:.0},{:.0})", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" → ");
        println!("[通勤] 路径规划（地铁 → 公司）：{seq}");

        let speed = ride_speed(CommuteChoice::Subway);
        let r = simulate_waypoints("地铁", &waypoints, from, speed);
        println!(
            "[通勤] 到站：公司（{} 帧，{:.1}s）",
            r.frames,
            r.frames as f32 / 60.0
        );

        assert!(r.arrived, "地铁应正常到站");
        assert_eq!(r.end_pos, station_pos(Location::Office).xz());
        assert!(
            waypoints
                .iter()
                .any(|p| Vec2::new(p[0], p[1]).distance(Vec2::ZERO) < 0.5),
            "路径必须经过市中心"
        );
        // 全程约 114 米（15+42+42+15）/ 地铁 30 m/s ≈ 3.8 秒
        let expect = (114.0 / ride_speed(CommuteChoice::Subway) * 60.0).round() as u32;
        assert!(
            r.frames.abs_diff(expect) <= 12,
            "行驶帧数应约为 {expect}，实际 {}",
            r.frames
        );
    }

    // 完整模拟：步行从家出生点 → 公司门口（A* 寻路 + 逐段行走）。
    #[test]
    fn simulate_walk_home_to_office() {
        let map = city_walk_map();
        let from = Vec2::new(-42.0, 33.0); // 家出生点（床边，与 scenes::spawn_pos 一致）
        let to = Vec2::new(-15.0, 42.0); // 公司站（门口）
        let (path, r) = simulate_walk(&map, from, to);

        // 关键节点：必须出家门洞（南墙门洞 x -44.5..-39.5, z≈20.6）
        let door = path
            .iter()
            .find(|p| p.x > -45.0 && p.x < -39.0 && p.y > 19.0 && p.y < 22.0);
        match door {
            Some(p) => println!("[步行] 途经：出家门洞 ({:.0},{:.0}) ✓", p.x, p.y),
            None => println!("[步行] 警告：路径未经过家门洞！"),
        }
        // 是否上主路（z≈0）——A* 可能抄近路穿过空地，仅提示
        if let Some(p) = path.iter().find(|p| p.y.abs() < 2.0) {
            println!("[步行] 途经：进入主路 ({:.0},{:.0}) ✓", p.x, p.y);
        } else {
            println!("[步行] 途经：抄近路穿过空地（未上主路）");
        }

        assert!(door.is_some(), "步行路径必须经过家门洞");
        assert_eq!(*path.last().unwrap(), to, "终点应为公司门口");
        assert!(r.arrived);
        // 距离合理：直线约 28.5m，绕行需出家门洞明显更长，但不应过远
        let straight = from.distance(to);
        assert!(
            r.total_dist > straight * 1.6,
            "绕行距离应明显大于直线（{straight:.0}m），实际 {:.0}m",
            r.total_dist
        );
        assert!(
            r.total_dist < 140.0,
            "步行距离不应过远，实际 {:.0}m",
            r.total_dist
        );
    }

    // 完整模拟：共享单车从家出生点 → 公司门口（A* 寻路 + 骑行速度）。
    // 路线与步行一致，仅速度更快，验证时间约为步行的 1/BIKE_SPEED_MULT。
    #[test]
    fn simulate_bike_home_to_office() {
        let map = city_walk_map();
        let from = Vec2::new(-42.0, 33.0); // 家出生点（床边，与 scenes::spawn_pos 一致）
        let to = Vec2::new(-15.0, 42.0); // 公司站（门口）
        let (path, r) = simulate_bike(&map, from, to);

        let door = path
            .iter()
            .find(|p| p.x > -45.0 && p.x < -39.0 && p.y > 19.0 && p.y < 22.0);
        match door {
            Some(p) => println!("[骑行] 途经：出家门洞 ({:.0},{:.0}) ✓", p.x, p.y),
            None => println!("[骑行] 警告：路径未经过家门洞！"),
        }

        assert!(door.is_some(), "骑行路径必须经过家门洞");
        assert_eq!(*path.last().unwrap(), to, "终点应为公司门口");
        assert!(r.arrived);
        // 骑行速度是步行的 BIKE_SPEED_MULT 倍 → 用时约为步行的 1/BIKE_SPEED_MULT
        let walk_frames = (r.total_dist / (PLAYER_SPEED / 60.0)) as u32;
        let ratio = walk_frames as f32 / r.frames as f32;
        assert!(
            (ratio - BIKE_SPEED_MULT).abs() < 0.15,
            "骑行应比步行快 {BIKE_SPEED_MULT} 倍，实际 {ratio:.2}"
        );
    }
}
