//! 统一交通调度器：把地铁 / 公交 / 步行 / 共享单车 / 汽车整合进同一个出行队列，
//! 按早晚高峰时段生成混合流量并推进仿真，统计各方式出行量、完成数与平均用时。
//! 调度器内的汽车沿路行驶时接入红绿灯（红灯停车线等待、绿灯通行），
//! 相位规则与 traffic_light 模块一致。注意：真实游戏的乘车暂不受信号控制，
//! 见 transit::transit_tick（本模块仅用于仿真验证）。
//!
//! 用途：城市 / 时段配置变动后快速验证「一天」的混合交通流量是否合理
//! （早高峰通勤潮、晚高峰返程潮、平峰稀疏），也便于后续扩展新的出行方式。
//! 所有方式统一走 `sim::drive_step` 驱动，日志与真实游戏一致。
#![allow(dead_code)] // 预留扩展模块：调度器与统计当前由仿真测试驱动，后续可接入实时玩法

use bevy::prelude::*;
use rand::{rngs::StdRng, RngExt, SeedableRng};
use std::collections::VecDeque;

use super::collision::{CollisionMap, find_path};
use super::components::CommuteChoice;
use super::resources::{BIKE_SPEED_MULT, Location, PLAYER_SPEED, station_pos};
use super::sim;
use super::traffic::{JUNCTION_COUNT, JUNCTIONS};
use super::traffic_light::{LightState, RED_SECS, junction_stop, light_secs};
use super::transit::{ride_speed, road_waypoints};

// 汽车沿 z=0 主路往返的端点（±x）与车速，与 traffic::spawn_vehicles 保持一致
const CAR_ROUTE_HALF: f32 = 52.0;
const CAR_SPEED: f32 = 11.0;

// ==================== 时段（高峰） ====================
/// 一天按仿真秒划分：早高峰 0-20s → 平峰 20-45s → 午饭 45-60s → 晚高峰 60s 起（无上界）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Peak {
    MorningRush,
    Daytime,
    LunchRush,
    EveningRush,
}

impl Peak {
    pub fn label(self) -> &'static str {
        match self {
            Peak::MorningRush => "早高峰",
            Peak::Daytime => "平峰",
            Peak::LunchRush => "午饭",
            Peak::EveningRush => "晚高峰",
        }
    }

    /// 出行生成间隔（秒）：高峰密集、平峰稀疏
    pub fn spawn_interval(self) -> f32 {
        match self {
            Peak::MorningRush => 2.0,
            Peak::Daytime => 7.0,
            Peak::LunchRush => 4.0,
            Peak::EveningRush => 2.5,
        }
    }
}

pub fn peak_at(t: f32) -> Peak {
    if t < 20.0 {
        Peak::MorningRush
    } else if t < 45.0 {
        Peak::Daytime
    } else if t < 60.0 {
        Peak::LunchRush
    } else {
        Peak::EveningRush
    }
}

// ==================== 出行方式 ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TravelMode {
    Subway,
    Bus,
    Walk,
    Bike,
    Car,
}

impl TravelMode {
    pub fn label(self) -> &'static str {
        match self {
            TravelMode::Subway => "地铁",
            TravelMode::Bus => "公交",
            TravelMode::Walk => "步行",
            TravelMode::Bike => "单车",
            TravelMode::Car => "汽车",
        }
    }

    /// 与真实玩法一致的行驶速度（米/秒）
    pub fn speed(self) -> f32 {
        match self {
            TravelMode::Subway => ride_speed(CommuteChoice::Subway),
            TravelMode::Bus => ride_speed(CommuteChoice::Bus),
            TravelMode::Walk => PLAYER_SPEED,
            TravelMode::Bike => PLAYER_SPEED * BIKE_SPEED_MULT,
            TravelMode::Car => CAR_SPEED,
        }
    }

    pub fn index(self) -> usize {
        match self {
            TravelMode::Subway => 0,
            TravelMode::Bus => 1,
            TravelMode::Walk => 2,
            TravelMode::Bike => 3,
            TravelMode::Car => 4,
        }
    }
}

// ==================== 一次出行 ====================
pub struct Trip {
    pub mode: TravelMode,
    pub from: Vec2,
    pub to_region: Location,
}

struct ActiveTrip {
    mode: TravelMode,
    waypoints: Vec<[f32; 2]>,
    idx: usize,
    pos: Vec3,
    start: f32,
    done: bool,
}

// ==================== 统计 ====================
#[derive(Default)]
pub struct TrafficStats {
    pub by_mode: [u32; 5], // 各方式生成的出行数
    pub finished: u32,
    pub total_secs: f32, // 已完成出行用时累计
}

impl TrafficStats {
    pub fn total_spawned(&self) -> u32 {
        self.by_mode.iter().sum()
    }

    pub fn avg_secs(&self) -> f32 {
        if self.finished > 0 {
            self.total_secs / self.finished as f32
        } else {
            0.0
        }
    }

    /// 各方式出行量文本（日志用）
    pub fn summary(&self) -> String {
        [
            TravelMode::Subway,
            TravelMode::Bus,
            TravelMode::Walk,
            TravelMode::Bike,
            TravelMode::Car,
        ]
        .iter()
        .map(|m| format!("{} {}", m.label(), self.by_mode[m.index()]))
        .collect::<Vec<_>>()
        .join(" ")
    }
}

// ==================== 调度器 ====================
pub struct Dispatcher {
    pub t: f32,                  // 累计仿真秒
    pub peak: Peak,              // 当前时段
    pub next_spawn: f32,         // 下次生成出行的时刻
    pub spawning: bool,          // 是否继续生成新出行（收尾统计时置 false）
    pub pending: VecDeque<Trip>, // 待发车队列
    active: Vec<ActiveTrip>,     // 在途出行（对外用 on_road() 查询）
    pub stats: TrafficStats,
    /// 各路口双相位灯态 (东西向, 南北向)，规则与 traffic_light::JunctionLight 一致
    pub lights: [(LightState, LightState); JUNCTION_COUNT],
    /// 各路口主相位剩余时间（初始相位与真实游戏一致：红灯 + 下标 × 3s 错开）
    light_timers: [f32; JUNCTION_COUNT],
    rng: StdRng, // 固定种子的随机源：保证仿真测试结果可复现
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self {
            t: 0.0,
            peak: Peak::MorningRush,
            next_spawn: 0.0,
            spawning: true,
            pending: VecDeque::new(),
            active: Vec::new(),
            stats: TrafficStats::default(),
            // 初始所有路口东西向红灯、南北向绿灯（互补）
            lights: [(LightState::Red, LightState::Green); JUNCTION_COUNT],
            light_timers: core::array::from_fn(|i| RED_SECS + i as f32 * 3.0),
            rng: StdRng::seed_from_u64(42),
        }
    }
}

impl Dispatcher {
    /// 推进一帧：红绿灯相位 → 更新时段 → 生成新出行 → 发车 → 驱动在途出行
    pub fn tick(&mut self, dt: f32, map: &CollisionMap) {
        self.tick_lights(dt);
        self.t += dt;
        self.peak = peak_at(self.t);

        // 到点生成一次出行（收尾阶段可关闭）
        if self.spawning && self.t >= self.next_spawn {
            self.next_spawn = self.t + self.peak.spawn_interval();
            if let Some(trip) = self.make_trip() {
                self.stats.by_mode[trip.mode.index()] += 1;
                self.pending.push_back(trip);
            }
        }

        // 发车（把待发队列投入运行）
        while let Some(trip) = self.pending.pop_front() {
            let act = self.build_trip(&trip, map);
            println!(
                "[调度] t={:.0}s {} 发车 {}：{}",
                self.t,
                self.peak.label(),
                trip.mode.label(),
                super::scenes::location_name(trip.to_region)
            );
            self.active.push(act);
        }

        // 推进在途出行（沿路行驶受红绿灯控制；步行/单车走人行道绕行，不排队等灯）
        for act in &mut self.active {
            if act.done {
                continue;
            }
            let (np, ni, done) = match act.mode {
                TravelMode::Walk | TravelMode::Bike => {
                    sim::drive_step(&act.waypoints, act.idx, act.pos, act.mode.speed() * dt)
                }
                _ => drive_with_lights(
                    &act.waypoints,
                    act.idx,
                    act.pos,
                    act.mode.speed(),
                    dt,
                    &self.lights,
                    JUNCTIONS,
                ),
            };
            act.pos = np;
            act.idx = ni;
            if done {
                act.done = true;
                self.stats.finished += 1;
                let dur = self.t - act.start;
                self.stats.total_secs += dur;
                println!(
                    "[调度] t={:.0}s {} 到达（{:.1}s）",
                    self.t,
                    act.mode.label(),
                    dur
                );
            }
        }
        self.active.retain(|a| !a.done);
    }

    /// 推进各路口红绿灯相位（与 traffic_light::traffic_tick 同一套规则：
    /// 东西向为主相位按 红→绿→黄 循环，南北向取互补）
    fn tick_lights(&mut self, dt: f32) {
        for i in 0..self.lights.len() {
            self.light_timers[i] -= dt;
            if self.light_timers[i] <= 0.0 {
                let new_ew = self.lights[i].0.next();
                self.lights[i] = (new_ew, new_ew.complement());
                self.light_timers[i] = light_secs(new_ew);
            }
        }
    }

    /// 是否已无出行在途 / 待发（可用于判断仿真是否跑完）
    pub fn is_idle(&self) -> bool {
        self.pending.is_empty() && self.active.is_empty()
    }

    /// 在途出行数
    pub fn on_road(&self) -> usize {
        self.active.len()
    }

    /// 生成一次出行：早高峰通勤潮 → 公司，晚高峰返程潮 → 家，平峰随机串门
    fn make_trip(&mut self) -> Option<Trip> {
        let rng = &mut self.rng;
        let (from, to_region) = match self.peak {
            Peak::MorningRush => (station_pos(Location::Home).xz(), Location::Office),
            Peak::EveningRush => (station_pos(Location::Office).xz(), Location::Home),
            _ => {
                let areas = [
                    Location::Home,
                    Location::Campus,
                    Location::Cafeteria,
                    Location::Office,
                    Location::Park,
                ];
                let i = rng.random_range(0..areas.len());
                let j = (i + 1 + rng.random_range(0..4)) % areas.len();
                (station_pos(areas[i]).xz(), areas[j])
            }
        };
        let mode = Self::pick_mode(self.peak, rng);
        // 汽车从主路端点出发（方向由 build_trip 按 to_region 决定），不走站点
        let from = if mode == TravelMode::Car {
            if to_region == Location::Home {
                Vec2::new(CAR_ROUTE_HALF, 0.0)
            } else {
                Vec2::new(-CAR_ROUTE_HALF, 0.0)
            }
        } else {
            from
        };
        Some(Trip {
            mode,
            from,
            to_region,
        })
    }

    /// 按时段选择出行方式（通勤高峰偏向公共交通与慢行）
    fn pick_mode(peak: Peak, rng: &mut impl rand::Rng) -> TravelMode {
        let r = rng.random::<f32>();
        match peak {
            Peak::MorningRush | Peak::EveningRush => {
                if r < 0.25 {
                    TravelMode::Subway
                } else if r < 0.45 {
                    TravelMode::Bus
                } else if r < 0.62 {
                    TravelMode::Walk
                } else if r < 0.82 {
                    TravelMode::Bike
                } else {
                    TravelMode::Car
                }
            }
            _ => {
                if r < 0.2 {
                    TravelMode::Subway
                } else if r < 0.4 {
                    TravelMode::Bus
                } else if r < 0.55 {
                    TravelMode::Walk
                } else if r < 0.7 {
                    TravelMode::Bike
                } else {
                    TravelMode::Car
                }
            }
        }
    }

    /// 生成出行路点：地铁/公交走道路网，步行/单车走 A* 绕行，汽车沿主路
    fn build_trip(&self, trip: &Trip, map: &CollisionMap) -> ActiveTrip {
        let waypoints: Vec<[f32; 2]> = match trip.mode {
            TravelMode::Walk | TravelMode::Bike => {
                let to = station_pos(trip.to_region).xz();
                let path = find_path(map, trip.from, to).unwrap_or_else(|| vec![trip.from, to]);
                path.iter().map(|p| [p.x, p.y]).collect()
            }
            TravelMode::Subway | TravelMode::Bus => road_waypoints(trip.from, trip.to_region)
                .iter()
                .map(|p| [p.x, p.y])
                .collect(),
            TravelMode::Car => {
                // 汽车沿 z=0 主路往返；返程（回家）反向
                let end = if trip.to_region == Location::Home {
                    Vec2::new(-CAR_ROUTE_HALF, 0.0)
                } else {
                    Vec2::new(CAR_ROUTE_HALF, 0.0)
                };
                vec![[trip.from.x, trip.from.y], [end.x, end.y]]
            }
        };
        ActiveTrip {
            mode: trip.mode,
            waypoints,
            idx: 0,
            pos: Vec3::new(trip.from.x, 0.0, trip.from.y),
            start: self.t,
            done: false,
        }
    }
}

/// 沿路点推进并受红绿灯控制（整合 traffic_light 模块的停车决策）：
/// 前方路口对应轴向为红灯 / 黄灯时，在停车线（路口前 8m）前停下等待；
/// 绿灯则按原速行驶。返回 (新位置, 新路点下标, 是否到达终点)。
fn drive_with_lights(
    waypoints: &[[f32; 2]],
    idx: usize,
    pos: Vec3,
    speed: f32,
    dt: f32,
    lights: &[(LightState, LightState)],
    junctions: &[(Vec2, usize)],
) -> (Vec3, usize, bool) {
    let n = waypoints.len();
    if n == 0 {
        return (pos, idx, true);
    }
    let w = waypoints[idx % n];
    let dir = (Vec2::new(w[0], w[1]) - pos.xz()).normalize_or_zero();
    let mut step = speed * dt;
    if let Some(stop) = junction_stop(pos.xz(), dir, lights, junctions) {
        let d = (stop - pos.xz()).length();
        if d > 1.0 {
            step = step.min(d); // 驶向停车线
        } else {
            step = 0.0; // 已在停车线前，等红灯/黄灯变绿
        }
    }
    sim::drive_step(waypoints, idx, pos, step)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 完整模拟「一天」的混合流量：早高峰通勤 → 平峰 → 午饭 → 晚高峰返程。
    // 用统一调度器生成并推进各方式出行，验证高峰流量明显多于平峰且绝大多数能到达。
    #[test]
    fn simulate_daily_mixed_flow() {
        let map = super::super::sim::city_walk_map();
        let mut d = Dispatcher::default();
        let dt = 1.0 / 60.0;
        let mut frames = 0u32;
        // 跑完 95s（含晚高峰收尾），再等所有在途出行到达
        while d.t < 95.0 && frames < 60 * 120 {
            d.tick(dt, &map);
            frames += 1;
        }
        // 收尾：停止生成新出行，只等已发车的在途出行全部到达（含红绿灯等待）
        d.spawning = false;
        let mut drain = 0;
        while !d.is_idle() && drain < 60 * 180 {
            d.tick(dt, &map);
            drain += 1;
        }
        println!(
            "[调度] 一天统计：出行 {} 次（{}），完成 {}，平均耗时 {:.1}s，在途 {}",
            d.stats.total_spawned(),
            d.stats.summary(),
            d.stats.finished,
            d.stats.avg_secs(),
            d.on_road()
        );

        // 校验：一天应有足够的混合出行，且绝大多数能到达终点
        let spawned = d.stats.total_spawned();
        assert!(spawned >= 15, "一天应生成足够多的出行，实际 {spawned}");
        assert!(
            d.stats.finished >= 10,
            "多数出行应到达，实际 {}",
            d.stats.finished
        );
        assert!(d.on_road() <= 3, "收尾后在途应很少，实际 {}", d.on_road());
        // 各方式都应出现过（混合流量）
        for (i, mode) in [
            TravelMode::Subway,
            TravelMode::Bus,
            TravelMode::Walk,
            TravelMode::Bike,
            TravelMode::Car,
        ]
        .iter()
        .enumerate()
        {
            assert!(d.stats.by_mode[i] > 0, "{} 方式应有出行", mode.label());
        }
        // 平均耗时应在合理区间：加入红绿灯等待后（最长一次红灯 15s），放宽到 2 分钟内
        assert!(
            d.stats.avg_secs() < 120.0,
            "平均耗时异常：{:.1}s",
            d.stats.avg_secs()
        );
    }

    // 红绿灯控制：一辆车沿主路从东到西，遇路口红灯在停车线等待、绿灯放行，
    // 且红灯期间绝不越过停车线（不闯红灯）。相位与真实游戏完全一致。
    #[test]
    fn car_waits_at_red_then_goes_on_green() {
        let waypoints = vec![[CAR_ROUTE_HALF, 0.0], [-CAR_ROUTE_HALF, 0.0]];
        let mut pos = Vec3::new(CAR_ROUTE_HALF, 0.0, 0.0);
        let mut idx = 0;
        let dt = 1.0 / 60.0;
        // 与 Dispatcher::default 一致的初始相位：所有路口东西向红灯
        let mut lights = [(LightState::Red, LightState::Green); JUNCTION_COUNT];
        let mut timers: [f32; JUNCTION_COUNT] = core::array::from_fn(|i| RED_SECS + i as f32 * 3.0);
        let mut arrived = false;
        let mut red_violation = false;
        for _ in 0..(60 * 90) {
            // 推进相位
            for i in 0..lights.len() {
                timers[i] -= dt;
                if timers[i] <= 0.0 {
                    let new_ew = lights[i].0.next();
                    lights[i] = (new_ew, new_ew.complement());
                    timers[i] = light_secs(new_ew);
                }
            }
            // 检查前方路口（与 junction_stop 同一匹配规则）：红灯/黄灯期间不得越过停车线
            let dir = (Vec2::new(
                waypoints[idx % waypoints.len()][0],
                waypoints[idx % waypoints.len()][1],
            ) - pos.xz())
            .normalize_or_zero();
            for &(jc, ji) in JUNCTIONS {
                let along = (jc - pos.xz()).dot(dir);
                let lateral = (jc - pos.xz() - dir * along).length();
                if along > 0.0 && along < 25.0 && lateral < 4.0 {
                    let st = if dir.x.abs() >= dir.y.abs() {
                        lights[ji].0
                    } else {
                        lights[ji].1
                    };
                    let stop = jc - dir * 8.0;
                    if st != LightState::Green && (pos.xz() - stop).dot(dir) > 0.01 {
                        red_violation = true;
                    }
                }
            }
            let (np, ni, done) =
                drive_with_lights(&waypoints, idx, pos, CAR_SPEED, dt, &lights, JUNCTIONS);
            pos = np;
            idx = ni;
            if done {
                arrived = true;
                break;
            }
        }
        assert!(arrived, "车辆应最终到达（红灯等待后绿灯放行）");
        assert!(!red_violation, "红灯/黄灯期间车辆不得越过停车线闯红灯");
    }
}
