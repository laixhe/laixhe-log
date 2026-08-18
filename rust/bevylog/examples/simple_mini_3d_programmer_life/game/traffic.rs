//! 城市交通设施：行驶车辆、过马路行人、斑马线（红绿灯见独立的 traffic_light 模块）。
//! - 车辆沿主路 / 环路自动行驶，按自己行驶轴向看对应车道信号灯（traffic_light::JunctionLight），
//!   红灯 / 黄灯在停车线（路口前 8m）停下，绿灯通行；前方有玩家或行人时让行；
//! - 过马路行人横穿某条道路时，在该道路车辆放行（绿/黄）时停在人行道等灯，
//!   道路红灯（车辆停）时过马路；没走完滞留在路中的行人车辆会减速鸣笛；
//! - 玩家被车辆实体推挤（玩家不能直接穿过车）。

use bevy::prelude::*;

use super::car_lights::{BrakeLight, HeadLight, TurnLight, TurnSide};
use super::components::{GameRoot, PlayerRoot, SceneRoot, WanderNpc};
use super::facing;
use super::sim;
use super::traffic_light::{JunctionLight, LightState, junction_stop};

// ==================== 路口布局 ====================
// 本城十字路口坐标（双相位信号系统见 traffic_light 模块）
pub const JUNCTIONS: &[(Vec2, usize)] = &[
    (Vec2::ZERO, 0),            // 市中心十字
    (Vec2::new(24.0, 0.0), 1),  // 环路东交点
    (Vec2::new(-24.0, 0.0), 2), // 环路西交点
    (Vec2::new(0.0, 24.0), 3),  // 环路北交点
    (Vec2::new(0.0, -24.0), 4), // 环路南交点
];

// 路口数量常量：各系统按它分配灯态数组，避免硬编码 5 造成多处强耦合/越界风险
pub const JUNCTION_COUNT: usize = JUNCTIONS.len();

// ==================== 车辆 ====================
// 转向灯 / 刹车灯 / 大灯组件与车灯系统见独立的 car_lights 模块。

/// 行驶车辆：沿 waypoints 循环行驶（到达终点后回到第一个点，实现往返 / 绕环）。
/// 路点用 [f32;2] 存储，与 transit::TransitState 一致，可直接复用 sim::drive_step。
/// 转向状态（last_dir / turn / turn_timer）由本模块的行驶逻辑维护，车灯系统只读。
#[derive(Component)]
pub struct Vehicle {
    pub waypoints: Vec<[f32; 2]>,
    pub idx: usize,
    pub speed: f32,                                // 米/秒
    pub last_dir: Vec2,                            // 上一帧行驶方向（用于检测转向）
    pub turn: Option<super::car_lights::TurnSide>, // 当前转向方向（转向灯亮起中）
    pub turn_timer: f32,                           // 转向灯剩余时长（秒）
}

// ==================== 过马路行人 ====================
/// 在斑马线 A↔B 之间往返的行人；横穿道路车辆放行时停在人行道等灯
#[derive(Clone, Copy, Component)]
pub struct CrossingPed {
    pub junction: usize,
    pub a: Vec2,
    pub b: Vec2,
    pub t: f32,
    pub dir: f32,
    pub speed: f32,
}

// ==================== 生成 ====================

/// 在本城五个路口生成红绿灯（信号系统实现见 traffic_light 模块）
pub fn spawn_lights(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    super::traffic_light::spawn_lights(commands, meshes, materials, JUNCTIONS);
}

/// 生成一辆小车（车身 + 车顶 + 车轮），沿 waypoints 行驶
fn spawn_car(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    waypoints: Vec<Vec2>,
    body_color: Color,
    speed: f32,
) {
    let body = materials.add(StandardMaterial {
        base_color: body_color,
        perceptual_roughness: 0.7,
        ..default()
    });
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.78, 0.9),
        perceptual_roughness: 0.3,
        ..default()
    });
    let wheel = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.12, 0.14),
        perceptual_roughness: 0.8,
        ..default()
    });
    let headlight = materials.add(StandardMaterial {
        base_color: Color::srgb(0.98, 0.92, 0.6),
        emissive: LinearRgba::BLACK, // 白天熄灭，晚上由 vehicle_tick 点亮
        ..default()
    });
    // 车尾刹车灯材质（减速 / 停车时点亮）
    let brake = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.1, 0.08),
        emissive: LinearRgba::BLACK,
        ..default()
    });
    // 左 / 右两侧转向灯各一套材质：转弯时对应侧两灯（前后）一起闪烁
    let turn_left = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.72, 0.2),
        emissive: LinearRgba::BLACK,
        ..default()
    });
    let turn_right = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.72, 0.2),
        emissive: LinearRgba::BLACK,
        ..default()
    });
    let body_mesh = meshes.add(Cuboid::new(1.7, 0.55, 0.85));
    let roof_mesh = meshes.add(Cuboid::new(0.85, 0.35, 0.72));
    let wheel_mesh = meshes.add(Cylinder::new(0.17, 0.14));
    let light_mesh = meshes.add(Cuboid::new(0.12, 0.1, 0.04));
    let turn_mesh = meshes.add(Cuboid::new(0.18, 0.12, 0.06));
    let brake_mesh = meshes.add(Cuboid::new(0.16, 0.1, 0.05));

    let waypoints: Vec<[f32; 2]> = waypoints.iter().map(|p| [p.x, p.y]).collect();
    let start = Vec2::new(waypoints[0][0], waypoints[0][1]);
    // 初始行驶方向（首个路点到第二个路点）
    let start_dir = if waypoints.len() > 1 {
        (Vec2::new(waypoints[1][0], waypoints[1][1]) - start).normalize_or_zero()
    } else {
        Vec2::X
    };
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            Vehicle {
                waypoints,
                idx: 0,
                speed,
                last_dir: start_dir,
                turn: None,
                turn_timer: 0.0,
            },
            Transform::from_xyz(start.x, 0.0, start.y),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(body_mesh.clone()),
                MeshMaterial3d(body.clone()),
                Transform::from_xyz(0.0, 0.42, 0.0),
            ));
            p.spawn((
                Mesh3d(roof_mesh.clone()),
                MeshMaterial3d(glass.clone()),
                Transform::from_xyz(-0.18, 0.72, 0.0),
            ));
            for (dx, dz) in [(-0.6, 0.35), (0.6, 0.35), (-0.6, -0.35), (0.6, -0.35)] {
                p.spawn((
                    Mesh3d(wheel_mesh.clone()),
                    MeshMaterial3d(wheel.clone()),
                    Transform::from_xyz(dx, 0.17, dz),
                ));
            }
            // 车头大灯（车头在模型局部 +X，朝向约定见 facing::facing_x；晚上点亮）
            p.spawn((
                HeadLight {
                    mat: headlight.clone(),
                },
                Mesh3d(light_mesh.clone()),
                MeshMaterial3d(headlight.clone()),
                Transform::from_xyz(0.72, 0.42, 0.35),
            ));
            p.spawn((
                HeadLight {
                    mat: headlight.clone(),
                },
                Mesh3d(light_mesh.clone()),
                MeshMaterial3d(headlight.clone()),
                Transform::from_xyz(0.72, 0.42, -0.35),
            ));
            // 转向灯：车头 / 车尾左右各一（橙色，转弯时对应侧前后一起闪烁）。
            // 车头朝局部 +X → 前侧 x=0.72、后侧 x=-0.72；左侧 +Z、右侧 -Z。
            for (x, z, side) in [
                (0.72, 0.42, TurnSide::Left),
                (0.72, -0.42, TurnSide::Right),
                (-0.72, 0.42, TurnSide::Left),
                (-0.72, -0.42, TurnSide::Right),
            ] {
                let mat = if side == TurnSide::Left {
                    turn_left.clone()
                } else {
                    turn_right.clone()
                };
                p.spawn((
                    TurnLight {
                        side,
                        mat: mat.clone(),
                    },
                    Mesh3d(turn_mesh.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_xyz(x, 0.5, z),
                ));
            }
            // 刹车灯：车尾左右各一（红色，减速 / 停车时点亮）
            for z in [0.3f32, -0.3] {
                p.spawn((
                    BrakeLight { mat: brake.clone() },
                    Mesh3d(brake_mesh.clone()),
                    MeshMaterial3d(brake.clone()),
                    Transform::from_xyz(-0.72, 0.4, z),
                ));
            }
        });
}

/// 生成主路 / 环路行驶车辆
pub fn spawn_vehicles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 主路东西向（往返，经过市中心路口）
    spawn_car(
        commands,
        meshes,
        materials,
        vec![Vec2::new(-52.0, 0.0), Vec2::new(52.0, 0.0)],
        Color::srgb(0.85, 0.42, 0.30),
        11.0,
    );
    spawn_car(
        commands,
        meshes,
        materials,
        vec![Vec2::new(52.0, 0.0), Vec2::new(-52.0, 0.0)],
        Color::srgb(0.38, 0.55, 0.85),
        10.0,
    );
    // 主路南北向
    spawn_car(
        commands,
        meshes,
        materials,
        vec![Vec2::new(0.0, -52.0), Vec2::new(0.0, 52.0)],
        Color::srgb(0.55, 0.78, 0.42),
        11.0,
    );
    spawn_car(
        commands,
        meshes,
        materials,
        vec![Vec2::new(0.0, 52.0), Vec2::new(0.0, -52.0)],
        Color::srgb(0.92, 0.72, 0.30),
        10.0,
    );
    // 环路（顺时针 / 逆时针各一）
    let ring = vec![
        Vec2::new(24.0, 24.0),
        Vec2::new(24.0, -24.0),
        Vec2::new(-24.0, -24.0),
        Vec2::new(-24.0, 24.0),
    ];
    spawn_car(
        commands,
        meshes,
        materials,
        ring.clone(),
        Color::srgb(0.55, 0.45, 0.80),
        9.0,
    );
    spawn_car(
        commands,
        meshes,
        materials,
        ring.into_iter().rev().collect(),
        Color::srgb(0.30, 0.75, 0.70),
        9.0,
    );
}

/// 中心路口的人行横道（白条）
pub fn spawn_crosswalk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let stripe = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.92, 0.88),
        perceptual_roughness: 0.9,
        ..default()
    });
    let stripe_ns = meshes.add(Cuboid::new(0.5, 0.02, 0.14)); // 横穿南北向主路（沿 X 铺开）
    let stripe_ew = meshes.add(Cuboid::new(0.14, 0.02, 0.5)); // 横穿东西向主路（沿 Z 铺开）
    // 南北向主路的斑马线：z = ±3.2（行人横穿 x=0 的主路，跨度与 spawn_crossing_ped 的 ±7 一致）
    for z in [3.2f32, -3.2] {
        for i in 0..15 {
            let x = -7.0 + i as f32 * 1.0;
            commands.spawn((
                GameRoot,
                SceneRoot,
                Mesh3d(stripe_ns.clone()),
                MeshMaterial3d(stripe.clone()),
                Transform::from_xyz(x, 0.045, z),
            ));
        }
    }
    // 东西向主路的斑马线：x = ±3.2（行人横穿 z=0 的主路，跨度与 ±7 一致）
    for x in [3.2f32, -3.2] {
        for i in 0..15 {
            let z = -7.0 + i as f32 * 1.0;
            commands.spawn((
                GameRoot,
                SceneRoot,
                Mesh3d(stripe_ew.clone()),
                MeshMaterial3d(stripe.clone()),
                Transform::from_xyz(x, 0.045, z),
            ));
        }
    }
}

/// 过马路行人：在斑马线上往返，横穿道路车辆放行时在端点等灯，道路红灯时过马路。
/// t0 为初始前进距离（0=起点端，len/2=斑马线中间），用于错开行人的横穿相位。
#[allow(clippy::too_many_arguments)]
pub fn spawn_crossing_ped(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    junction: usize,
    a: Vec2,
    b: Vec2,
    t0: f32,
    speed: f32,
    shirt: Color,
) {
    let cloth = materials.add(StandardMaterial {
        base_color: shirt,
        perceptual_roughness: 0.75,
        ..default()
    });
    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.78, 0.62),
        perceptual_roughness: 0.7,
        ..default()
    });
    let pants = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.32, 0.4),
        perceptual_roughness: 0.8,
        ..default()
    });
    // 出生位置按 t0 落在斑马线上（而非固定在端点），避免首帧瞬移；
    // 朝向与行走方向一致（局部 +Z 前向约定，见 facing::facing_z）
    let len = a.distance(b);
    let start = if len > 0.0 {
        a.lerp(b, (t0 / len).clamp(0.0, 1.0))
    } else {
        a
    };
    let fwd = (b - a).normalize_or_zero();
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            CrossingPed {
                junction,
                a,
                b,
                t: t0,
                dir: 1.0,
                speed,
            },
            Transform::from_translation(Vec3::new(start.x, 0.0, start.y))
                .with_rotation(facing::facing_z(fwd)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.4, 0.6, 0.26))),
                MeshMaterial3d(cloth.clone()),
                Transform::from_xyz(0.0, 0.78, 0.0),
            ));
            p.spawn((
                Mesh3d(meshes.add(Sphere::new(0.18))),
                MeshMaterial3d(skin.clone()),
                Transform::from_xyz(0.0, 1.3, 0.0),
            ));
            p.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.18, 0.5, 0.18))),
                MeshMaterial3d(pants),
                Transform::from_xyz(0.0, 0.3, 0.0),
            ));
        });
}

// ==================== 运行 ====================

/// 判断车辆对一个横穿行人应如何反应：返回 (block 停车, slow 减速鸣笛)。
/// 用几何求交：求「行人横穿线」与「车辆行驶线」的交点——
/// 只有行人正横穿且交汇点在本车前方、行人已接近交汇点时才反应；
/// 与横穿线平行（如相邻车道）永不误报；在端点等灯的行人不触发。
/// 阈值含义（米）：平行判定的叉积阈值 0.1；交汇点参数 t ∈ (-2, 7) 视为
/// 「在本车前方 7m 内」（-2 是有意的容差，允许交汇点略在车后即已接近）；
/// 行人横穿进度在 0.15..0.85 才视为「正在横穿」；行人与交汇点距离 < 3.0
/// 视为已接近；交汇点距本车 < 2.2 时急停，否则减速鸣笛。
fn react_to_crosser(car: Vec2, dir: Vec2, ped: &CrossingPed, ped_pos: Vec2) -> (bool, bool) {
    let len = ped.a.distance(ped.b);
    if len <= 0.0 {
        return (false, false);
    }
    let cross_dir = (ped.b - ped.a).normalize_or_zero();
    // 车辆行驶方向 × 行人横穿方向的叉积（z 分量）：≈0 表示两线平行（不相交）
    let cross = dir.x * cross_dir.y - dir.y * cross_dir.x;
    if cross.abs() < 0.1 {
        return (false, false);
    }
    // 求交汇点参数 t（沿车辆行驶方向，从本车到交汇点的距离）
    let to_ped = ped_pos - car;
    let t = (to_ped.x * cross_dir.y - to_ped.y * cross_dir.x) / cross;
    if !(t > -2.0 && t < 7.0) {
        return (false, false); // 交汇点不在前方 7m 内
    }
    let mid = ped.t > len * 0.15 && ped.t < len * 0.85; // 横穿中（不在人行道端点）
    if !mid {
        return (false, false);
    }
    // 行人当前是否已接近交汇点（离得远说明还没横穿到本车车道）
    let intersect = car + dir * t;
    if ped_pos.distance(intersect) > 3.0 {
        return (false, false);
    }
    if t < 2.2 {
        (true, false) // 交汇点近在眼前 → 停车
    } else {
        (false, true) // 前方有横穿行人 → 减速鸣笛
    }
}

// 车辆在路口前的停车决策见 traffic_light::junction_stop
// （按行驶轴向看对应相位，红灯 / 黄灯在停车线前停，绿灯通行）。

/// 行人是否被信号灯冻住（停在人行道端点）：行人横穿某条道路，
/// 在该道路车辆放行（绿/黄）时等在端点；道路红灯（车辆停）时过马路。
fn ped_frozen_by_light(ped: &CrossingPed, ew: LightState, ns: LightState) -> bool {
    let len = ped.a.distance(ped.b);
    if len <= 0.0 {
        return true;
    }
    // 沿 X 走 = 横穿南北向道路 → 看南北向灯；沿 Z 走 = 横穿东西向道路 → 看东西向灯
    let walk = (ped.b - ped.a).normalize_or_zero();
    let crossing_ns = walk.x.abs() >= walk.y.abs();
    let car_light = if crossing_ns { ns } else { ew };
    let at_end = ped.t <= 0.05 || ped.t >= len - 0.05;
    car_light != LightState::Red && at_end
}

/// 车辆行驶：沿路点前进；前方路口红灯 / 黄灯在停车线前停；
/// 有行人/玩家挡路则停车让行；行人横穿马路时减速并鸣笛。
/// 车辆本身也作为动态障碍，玩家会被推挤（见 movement::move_player）。
#[allow(clippy::too_many_arguments)]
pub fn vehicle_tick(
    time: Res<Time>,
    mut vehicles: Query<(&mut Vehicle, &mut Transform)>,
    player: Single<&Transform, (With<PlayerRoot>, Without<Vehicle>)>,
    crossing_peds: Query<(&CrossingPed, &Transform), (Without<PlayerRoot>, Without<Vehicle>)>,
    street_peds: Query<&Transform, (With<WanderNpc>, Without<PlayerRoot>, Without<Vehicle>)>,
    lights: Query<&JunctionLight>,
    bank: Res<super::sfx::SoundBank>,
    mut commands: Commands,
    mut horn_cd: Local<f32>, // 鸣笛冷却（秒），避免持续刷喇叭
) {
    let dt = time.delta_secs();
    // 收集各路口双相位灯态 (东西向, 南北向)
    let mut states = [(LightState::Red, LightState::Red); JUNCTION_COUNT];
    for l in &lights {
        if l.junction < states.len() {
            states[l.junction] = (l.ew, l.ns());
        }
    }
    for (mut v, mut tf) in &mut vehicles {
        let n = v.waypoints.len();
        if n == 0 {
            continue;
        }
        let pos = tf.translation;
        let t = v.waypoints[v.idx % n];
        let target = Vec2::new(t[0], t[1]);
        let dir = (target - pos.xz()).normalize_or_zero();

        // —— 转向状态：行驶方向明显变化 → 打对应侧转向灯（闪烁由 car_lights 系统处理）——
        if let Some(side) = super::car_lights::detect_turn(v.last_dir, dir) {
            v.turn = Some(side);
            v.turn_timer = super::car_lights::TURN_DURATION;
        }
        v.last_dir = dir;
        if v.turn_timer > 0.0 {
            v.turn_timer -= dt;
            if v.turn_timer <= 0.0 {
                v.turn = None;
            }
        }

        // 行驶方向前方 7m 处的关注点
        let ahead = pos.xz() + dir * 7.0;

        // 1) 普通挡路（玩家 / 人行道行人）→ 停车让行
        let mut block = player.translation.xz().distance(ahead) < 2.2
            || street_peds
                .iter()
                .any(|p| p.translation.xz().distance(ahead) < 2.0);

        // 2) 行人横穿本车道 → 减速 + 鸣笛；很近则停车
        let mut slow = false;
        for (ped, tf) in &crossing_peds {
            let (b, s) = react_to_crosser(pos.xz(), dir, ped, tf.translation.xz());
            if b {
                block = true;
            }
            if s {
                slow = true;
            }
        }
        let mut step = if block { 0.0 } else { v.speed * dt }; // 挡路 → 停车
        if slow {
            // 减速到约 1/3 速度通过，同时鸣笛提醒（带冷却）
            *horn_cd -= dt;
            if *horn_cd <= 0.0 {
                *horn_cd = 1.5;
                super::sfx::play(&mut commands, &bank, super::sfx::Sfx::Horn);
                info!("[交通] 汽车鸣笛，减速让行横穿行人");
            }
            step *= 0.35;
        }

        // 信号灯：按行驶轴向看对应相位，红灯 / 黄灯在停车线（路口前 8m）停下
        let stop_line = junction_stop(pos.xz(), dir, &states, JUNCTIONS);

        if let Some(stop) = stop_line {
            let d = (stop - pos.xz()).length();
            if d > 1.0 {
                let mv = (stop - pos.xz()).normalize_or_zero() * step.min(d);
                tf.translation.x += mv.x;
                tf.translation.z += mv.y;
            }
        } else {
            let (np, ni, _) = sim::drive_step(&v.waypoints, v.idx, pos, step);
            tf.translation = np;
            v.idx = ni % n;
        }

        // 面向下一个路点（车头朝局部 +X，见 facing::facing_x）
        let nt = v.waypoints[v.idx % n];
        let next = (Vec2::new(nt[0], nt[1]) - tf.translation.xz()).normalize_or_zero();
        if next.length_squared() > 0.001 {
            tf.rotation = facing::facing_x(next);
        }
        // 车灯（刹车灯 / 大灯 / 转向灯闪烁）由 car_lights::car_lights_tick 渲染
    }
}

/// 过马路行人：沿斑马线往返；横穿道路车辆放行（绿/黄）时在端点等灯，
/// 道路红灯（车辆停）时过马路；滞留在路中的行人继续走完（车辆会让行鸣笛）
pub fn crossing_tick(
    time: Res<Time>,
    mut peds: Query<(&mut CrossingPed, &mut Transform)>,
    lights: Query<&JunctionLight>,
) {
    let dt = time.delta_secs();
    let mut states = [(LightState::Red, LightState::Red); JUNCTION_COUNT];
    for l in &lights {
        if l.junction < states.len() {
            states[l.junction] = (l.ew, l.ns());
        }
    }
    for (mut ped, mut tf) in &mut peds {
        let len = ped.a.distance(ped.b);
        if len <= 0.0 {
            continue;
        }
        // 横穿道路车辆放行时在端点等灯；道路红灯（车辆停）时过马路
        let (ew, ns) = match states.get(ped.junction) {
            Some(&s) => s,
            None => continue, // 越界防护：路口下标由调用方保证，异常时跳过该行人
        };
        if ped_frozen_by_light(&ped, ew, ns) {
            continue;
        }
        ped.t += dt * ped.speed * ped.dir;
        if ped.t >= len {
            ped.t = len;
            ped.dir = -1.0;
        }
        if ped.t <= 0.0 {
            ped.t = 0.0;
            ped.dir = 1.0;
        }
        let pos = ped.a.lerp(ped.b, ped.t / len);
        tf.translation = Vec3::new(pos.x, 0.05, pos.y);
        // 正面朝行走方向（局部 +Z 前向约定，见 facing::facing_z）
        let fwd = (ped.b - ped.a).normalize_or_zero();
        tf.rotation = facing::facing_z(fwd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // 相位时长常量来自独立的 traffic_light 模块
    use crate::game::traffic_light::{RED_SECS, light_secs};
    // 转向灯判定测试见 car_lights 模块

    // 构造一个在 z=3.2 斑马线（x -7..7）上的行人：t 为前进距离，x 为当前位置
    fn crosser(t: f32, x: f32) -> (CrossingPed, Vec2) {
        let ped = CrossingPed {
            junction: 0,
            a: Vec2::new(-7.0, 3.2),
            b: Vec2::new(7.0, 3.2),
            t,
            dir: 1.0,
            speed: 1.1,
        };
        (ped, Vec2::new(x, 3.2))
    }

    #[test]
    fn slow_for_crossing_pedestrian() {
        // 车自南向北接近，行人在斑马线中间横穿 → 减速鸣笛（不停车）
        let car = Vec2::new(0.0, 0.0);
        let dir = Vec2::new(0.0, 1.0);
        let (ped, pos) = crosser(6.0, -1.0);
        let (block, slow) = react_to_crosser(car, dir, &ped, pos);
        assert!(slow, "横穿行人应触发减速鸣笛");
        assert!(!block, "距离尚远不应急停");
    }

    #[test]
    fn stop_when_crosser_is_close() {
        // 车已到交汇点近前（距交汇点 < 2.2m）→ 停车
        let car = Vec2::new(0.0, 1.5);
        let dir = Vec2::new(0.0, 1.0);
        let (ped, pos) = crosser(7.0, 0.0);
        let (block, slow) = react_to_crosser(car, dir, &ped, pos);
        assert!(block, "行人贴近应停车");
        assert!(!slow);
    }

    #[test]
    fn ignore_ped_on_other_lane() {
        // 东西向车在 z=0 主路，行人在 z=3.2 斑马线横穿 → 行驶线与横穿线平行，不误报
        let car = Vec2::new(0.0, 0.0);
        let dir = Vec2::new(1.0, 0.0);
        let (ped, pos) = crosser(7.0, 0.0);
        let (block, slow) = react_to_crosser(car, dir, &ped, pos);
        assert!(!block && !slow, "相邻车道行人不应触发");
    }

    #[test]
    fn ignore_ped_waiting_at_sidewalk() {
        // 行人停在人行道端点等灯（t=0）→ 不触发
        let car = Vec2::new(0.0, -3.0);
        let dir = Vec2::new(0.0, 1.0);
        let (ped, pos) = crosser(0.0, -7.0);
        let (block, slow) = react_to_crosser(car, dir, &ped, pos);
        assert!(!block && !slow, "端点等灯行人不应触发");
    }

    #[test]
    fn ignore_ped_far_from_crossing() {
        // 行人横穿中但还离交汇点很远（x=-4.5，d_ped>3）→ 不触发（尚未进入本车车道）
        let car = Vec2::new(0.0, 0.0);
        let dir = Vec2::new(0.0, 1.0);
        let (ped, pos) = crosser(11.5, -4.5);
        let (block, slow) = react_to_crosser(car, dir, &ped, pos);
        assert!(!block && !slow, "远离交汇点的行人不应触发");
    }

    // 运动仿真：车辆匀速穿过斑马线、行人在横穿时，多个初始相位下应至少触发一次减速鸣笛。
    // 证明「行人横穿 → 减速鸣笛」不是仅靠静态坐标成立，而是在真实运动中可发生。
    #[test]
    fn vehicle_honks_when_crossing_in_motion() {
        let dt = 1.0 / 60.0;
        let dir = Vec2::new(0.0, 1.0);
        let ped = CrossingPed {
            junction: 0,
            a: Vec2::new(7.0, -3.2),
            b: Vec2::new(-7.0, -3.2),
            t: 0.0,
            dir: 1.0,
            speed: 1.0,
        };
        let mut any_honk = false;
        for t0 in [0.0f32, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0] {
            let mut car = Vec2::new(0.0, -8.0); // 从停车线出发（绿灯后）
            let mut ped = CrossingPed { t: t0, ..ped };
            let mut t = t0;
            let mut honked = false;
            for _ in 0..(60 * 20) {
                car.y += 11.0 * dt; // 车辆 11 m/s 匀速向北
                t += 1.0 * dt; // 行人横穿（x 单调减小，到端后重来）
                if t > 14.0 {
                    t = 0.0;
                }
                ped.t = t; // 推进行人横穿进度
                let x = 7.0 - t;
                let (block, slow) = react_to_crosser(car, dir, &ped, Vec2::new(x, -3.2));
                if slow {
                    honked = true;
                    break;
                }
                if block {
                    break;
                }
                if car.y > 50.0 {
                    break;
                }
            }
            if honked {
                any_honk = true;
            }
        }
        assert!(any_honk, "车辆运动中经过横穿行人，应至少触发一次减速鸣笛");
    }

    // ===== 十字路口信号交互 =====
    // （纯相位互补测试见 traffic_light 模块）

    #[test]
    fn vehicle_stops_only_on_its_axis_red() {
        // 中心路口：东西向红灯、南北向绿灯 → 东西向车停在停车线，南北向车直接通行
        let mut lights = [(LightState::Red, LightState::Red); 5];
        lights[0] = (LightState::Red, LightState::Green);
        let ew_car = Vec2::new(-10.0, 0.0);
        let stop = junction_stop(ew_car, Vec2::X, &lights, JUNCTIONS);
        assert!(stop.is_some(), "东西向红灯，东西向车应停车");
        assert_eq!(stop.unwrap(), Vec2::new(-8.0, 0.0)); // 路口 (0,0) 前 8m 的停车线
        let ns_car = Vec2::new(0.0, -10.0);
        assert!(
            junction_stop(ns_car, Vec2::Y, &lights, JUNCTIONS).is_none(),
            "南北向绿灯，南北向车应通行"
        );
    }

    #[test]
    fn vehicle_stops_on_yellow_and_ignores_other_axis() {
        // 黄灯同样停车；自身轴向绿灯时不受对向红灯影响
        let mut lights = [(LightState::Red, LightState::Red); 5];
        lights[0] = (LightState::Yellow, LightState::Red);
        assert!(
            junction_stop(Vec2::new(-10.0, 0.0), Vec2::X, &lights, JUNCTIONS).is_some(),
            "东西向黄灯应停车"
        );
        let mut lights2 = [(LightState::Red, LightState::Red); 5];
        lights2[0] = (LightState::Red, LightState::Green);
        assert!(
            junction_stop(Vec2::new(0.0, -10.0), Vec2::Y, &lights2, JUNCTIONS).is_none(),
            "南北向绿灯应通行"
        );
    }

    #[test]
    fn ped_crosses_when_road_red_and_waits_on_green() {
        // 行人沿 X 走 = 横穿南北向道路 → 看南北向灯：红灯（车辆停）时过马路，绿/黄时端点等灯
        let ped = CrossingPed {
            junction: 0,
            a: Vec2::new(-7.0, 3.2),
            b: Vec2::new(7.0, 3.2),
            t: 0.0,
            dir: 1.0,
            speed: 1.0,
        };
        // 南北向红灯（东西向绿灯）→ 可过马路
        assert!(!ped_frozen_by_light(
            &ped,
            LightState::Green,
            LightState::Red
        ));
        // 南北向绿灯 / 黄灯 → 在端点等灯
        assert!(ped_frozen_by_light(
            &ped,
            LightState::Red,
            LightState::Green
        ));
        assert!(ped_frozen_by_light(
            &ped,
            LightState::Red,
            LightState::Yellow
        ));
        // 滞留在路中的行人不被冻住（继续走完）
        let mid = CrossingPed { t: 7.0, ..ped };
        assert!(!ped_frozen_by_light(
            &mid,
            LightState::Red,
            LightState::Green
        ));
    }

    #[test]
    fn complete_intersection_flow_simulation() {
        // 中心路口完整通行仿真（南北向车道）：
        // - 南北向红灯：车辆停在停车线（路口前 8m）、行人过马路；
        // - 南北向绿灯：车辆通行、行人在端点等灯；
        // - 红灯期间车辆绝不进入路口区域。
        let dt = 1.0 / 60.0;
        let mut ew = LightState::Red;
        let mut timer = RED_SECS;
        let stop = Vec2::new(0.0, -8.0);
        let mut car = Vec2::new(0.0, -52.0);
        let mut ped = CrossingPed {
            junction: 0,
            a: Vec2::new(-7.0, 3.2),
            b: Vec2::new(7.0, 3.2),
            t: 0.0,
            dir: 1.0,
            speed: 0.8,
        };
        let len = 14.0;
        let mut any_walk = false;
        let mut any_wait = false;
        // 红灯期间从停车线越线进入路口 = 违规；绿灯时已进入路口的车清空不算
        let mut at_stop_line = false;
        let mut entered_during_red = false;
        for _ in 0..(60 * 120) {
            timer -= dt;
            if timer <= 0.0 {
                ew = ew.next();
                timer = light_secs(ew);
            }
            let ns = ew.complement();
            // 车辆：红灯/黄灯在停车线前停（绿灯时已在路中的车清空路口），绿灯通行
            if ns != LightState::Green {
                if at_stop_line && car.y > stop.y + 0.01 {
                    entered_during_red = true;
                }
                if car.y < stop.y {
                    car.y = (car.y + 11.0 * dt).min(stop.y);
                }
                at_stop_line = (car.y - stop.y).abs() <= 0.02;
            } else {
                at_stop_line = false;
                car.y += 11.0 * dt;
                if car.y > 52.0 {
                    car.y = -52.0;
                }
            }
            // 行人：横穿南北向道路看南北向灯
            let at_end = ped.t <= 0.05 || ped.t >= len - 0.05;
            if ns != LightState::Red && at_end {
                any_wait = true;
            } else {
                ped.t += 0.8 * dt * ped.dir;
                if ped.t >= len {
                    ped.t = len;
                    ped.dir = -1.0;
                }
                if ped.t <= 0.0 {
                    ped.t = 0.0;
                    ped.dir = 1.0;
                }
                any_walk = true;
            }
        }
        assert!(any_walk, "南北向红灯期间行人应能过马路");
        assert!(any_wait, "南北向绿灯期间行人应在端点等灯");
        assert!(!entered_during_red, "红灯期间车辆不得从停车线越线进入路口");
    }

    #[test]
    fn car_honks_at_straggler_ped_after_green() {
        // 行人在绿灯前没走完、滞留在路中（x≈-2 附近），南北向变绿灯后车辆起步，
        // 应触发减速鸣笛（或急停），保证不会撞上没走完的行人。
        let dt = 1.0 / 60.0;
        let dir = Vec2::Y;
        let ped = CrossingPed {
            junction: 0,
            a: Vec2::new(-7.0, 3.2),
            b: Vec2::new(7.0, 3.2),
            t: 5.0, // 路中（x = -2）
            dir: 1.0,
            speed: 0.8,
        };
        let mut car = Vec2::new(0.0, -8.0); // 刚从停车线起步
        let mut any = false;
        for _ in 0..(60 * 6) {
            car.y += 11.0 * dt;
            let mut p = ped;
            p.t += 0.8 * dt;
            let pos = p.a.lerp(p.b, p.t / p.a.distance(p.b));
            let (b, s) = react_to_crosser(car, dir, &p, pos);
            if s || b {
                any = true;
                break;
            }
        }
        assert!(any, "绿灯起步遇到滞留路中的行人应鸣笛或停车让行");
    }
}
