//! 独立可复用的十字路口红绿灯模块（不依赖具体城市布局）：
//! - 三态信号灯：红 → 绿 → 黄 循环，亮灯带自发光（符合现实的三色信号灯）；
//! - 双相位互补：每个路口一套主相位（东西向），南北向自动取互补，
//!   东西向绿灯 ⇔ 南北向红灯，两方向绝不同时放行，模拟完整十字路口通行；
//! - 生成时只需传入「路口坐标 + 路口下标」列表即可在任何地图上复用；
//! - 车辆 / 行人通过查询 `JunctionLight` 组件获取任意路口的双相位灯态。

use bevy::prelude::*;

use super::components::{GameRoot, SceneRoot};

// ==================== 信号灯三态 ====================
// 红 → 绿 → 黄 → 红（现实中黄灯在绿灯之后警示）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LightState {
    Red,
    Green,
    Yellow,
}

impl LightState {
    pub fn next(self) -> LightState {
        match self {
            LightState::Red => LightState::Green,
            LightState::Green => LightState::Yellow,
            LightState::Yellow => LightState::Red,
        }
    }

    /// 对向车道的互补相位：东西向绿灯时南北向红灯；
    /// 黄灯（警示即将变红）期间对向保持红灯，避免两方向同时抢行。
    pub fn complement(self) -> LightState {
        match self {
            LightState::Red => LightState::Green,
            LightState::Green => LightState::Red,
            LightState::Yellow => LightState::Red,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            LightState::Red => "红灯",
            LightState::Green => "绿灯",
            LightState::Yellow => "黄灯",
        }
    }
}

pub const RED_SECS: f32 = 15.0;
pub const GREEN_SECS: f32 = 12.0;
pub const YELLOW_SECS: f32 = 3.0;

/// 各灯态持续时间（供相位推进与外部仿真 / 测试复用）
pub fn light_secs(s: LightState) -> f32 {
    match s {
        LightState::Red => RED_SECS,
        LightState::Green => GREEN_SECS,
        LightState::Yellow => YELLOW_SECS,
    }
}

// ==================== 组件 ====================

/// 路口信号主相位（挂在无实体的主实体上）：东西向为主相位，
/// 南北向 = 东西向互补，保证十字路口两方向绝不同时放行。
#[derive(Component)]
pub struct JunctionLight {
    pub junction: usize, // 路口下标（对应生成时传入的路口列表）
    pub ew: LightState,  // 东西向车道相位
    pub timer: f32,
}

impl JunctionLight {
    /// 南北向车道相位（东西向的互补）
    pub fn ns(&self) -> LightState {
        self.ew.complement()
    }
}

/// 单个灯杆（面向某条路的来车方向）；phase=0 显示东西向灯态，phase=1 显示南北向
#[derive(Component)]
pub struct LightRig {
    pub junction: usize,
    pub phase: u8,
}

/// 灯头标记（红 / 黄 / 绿，材质各自独立以便切换亮度）
#[derive(Component)]
pub struct LightBulb {
    pub kind: LightState,
}

// ==================== 生成 ====================

/// 在传入的路口列表上生成红绿灯：每个路口一套主相位实体 + 两套灯杆
/// （东西向灯杆面向东西向车道，南北向灯杆面向南北向车道，相位互补）。
/// `junctions` 为 (路口坐标, 路口下标) 列表，由调用方（城市布局）提供，便于复用。
pub fn spawn_lights(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    junctions: &[(Vec2, usize)],
) {
    let pole = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.24, 0.28),
        perceptual_roughness: 0.6,
        ..default()
    });
    let housing = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.10),
        perceptual_roughness: 0.7,
        ..default()
    });
    let pole_mesh = meshes.add(Cylinder::new(0.09, 3.4));
    let box_mesh = meshes.add(Cuboid::new(1.1, 2.4, 0.35));
    let bulb_mesh = meshes.add(Sphere::new(0.30));

    for (i, (pos, ji)) in junctions.iter().enumerate() {
        // 主相位实体：只存信号状态，无网格（供车辆 / 行人查询）
        commands.spawn((
            GameRoot,
            SceneRoot,
            JunctionLight {
                junction: *ji,
                ew: LightState::Red,
                timer: RED_SECS + i as f32 * 3.0, // 各路口相位错开，避免整城同时变灯
            },
        ));
        // 两套灯杆：东西向灯杆（朝东西向车道）、南北向灯杆（朝南北向车道）
        spawn_light_pole(
            commands,
            materials,
            Vec3::new(pos.x + 3.6, 0.0, pos.y - 3.6),
            0.0,
            *ji,
            0,
            &pole,
            &housing,
            &pole_mesh,
            &box_mesh,
            &bulb_mesh,
        );
        spawn_light_pole(
            commands,
            materials,
            Vec3::new(pos.x - 3.6, 0.0, pos.y + 3.6),
            std::f32::consts::FRAC_PI_2,
            *ji,
            1,
            &pole,
            &housing,
            &pole_mesh,
            &box_mesh,
            &bulb_mesh,
        );
    }
}

/// 生成一套灯杆（灯杆 + 灯箱 + 红黄绿三灯头），yaw 让灯面向对应车道
#[allow(clippy::too_many_arguments)]
fn spawn_light_pole(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
    yaw: f32,
    junction: usize,
    phase: u8,
    pole: &Handle<StandardMaterial>,
    housing: &Handle<StandardMaterial>,
    pole_mesh: &Handle<Mesh>,
    box_mesh: &Handle<Mesh>,
    bulb_mesh: &Handle<Mesh>,
) {
    // 每个灯头独立材质（初始只有红灯亮）
    let red = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.15, 0.10),
        emissive: LinearRgba::new(1.0, 0.15, 0.08, 1.0),
        ..default()
    });
    let yellow = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.55, 0.10),
        emissive: LinearRgba::BLACK,
        ..default()
    });
    let green = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.55, 0.20),
        emissive: LinearRgba::BLACK,
        ..default()
    });
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            LightRig { junction, phase },
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(yaw)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(pole_mesh.clone()),
                MeshMaterial3d(pole.clone()),
                Transform::from_xyz(0.0, 1.7, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh.clone()),
                MeshMaterial3d(housing.clone()),
                Transform::from_xyz(0.0, 3.5, 0.0),
            ));
            p.spawn((
                LightBulb {
                    kind: LightState::Red,
                },
                Mesh3d(bulb_mesh.clone()),
                MeshMaterial3d(red),
                Transform::from_xyz(0.0, 4.35, 0.22),
            ));
            p.spawn((
                LightBulb {
                    kind: LightState::Yellow,
                },
                Mesh3d(bulb_mesh.clone()),
                MeshMaterial3d(yellow),
                Transform::from_xyz(0.0, 3.55, 0.22),
            ));
            p.spawn((
                LightBulb {
                    kind: LightState::Green,
                },
                Mesh3d(bulb_mesh.clone()),
                MeshMaterial3d(green),
                Transform::from_xyz(0.0, 2.75, 0.22),
            ));
        });
}

// ==================== 运行 ====================

/// 车辆在路口前的停车决策（纯函数，真实游戏与仿真调度器复用）：
/// 沿行驶方向 25m 内最近的交叉路口，对应轴向（东西向/南北向）为红灯或黄灯时，
/// 返回停车点（路口前 8m 的停车线）；绿灯返回 None 直接通行。
/// `lights[j]` = (东西向灯态, 南北向灯态)，`junctions` 为 (路口坐标, 下标) 列表。
pub fn junction_stop(
    pos: Vec2,
    dir: Vec2,
    lights: &[(LightState, LightState)],
    junctions: &[(Vec2, usize)],
) -> Option<Vec2> {
    // 车辆按自身行驶轴向看对应信号：沿 X 走看东西向灯，沿 Z 走看南北向灯
    let along_x = dir.x.abs() >= dir.y.abs();
    for &(jc, ji) in junctions {
        let to_j = jc - pos;
        let along = to_j.dot(dir);
        let lateral = (to_j - dir * along).length();
        if along > 0.0 && along < 25.0 && lateral < 4.0 {
            let (ew, ns) = lights[ji];
            let st = if along_x { ew } else { ns };
            if st != LightState::Green {
                let stop = jc - dir * 8.0;
                // 已越过停车线（停车点落在身后）→ 视为已在路口内，清空路口继续通行，不倒车
                if (stop - pos).dot(dir) > 0.0 {
                    return Some(stop); // 红灯 / 黄灯都在停车线前停
                }
                return None;
            }
            return None;
        }
    }
    None
}

/// 红绿灯相位推进 + 两套灯杆（东西向 / 南北向）亮度切换。
/// 南北向灯态 = 东西向互补，两方向绝不同时放行。
pub fn traffic_tick(
    time: Res<Time>,
    mut lights: Query<&mut JunctionLight>,
    rigs: Query<(&LightRig, &Children)>,
    mut bulbs: Query<(&LightBulb, &mut MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_secs();
    for mut light in &mut lights {
        light.timer -= dt;
        if light.timer <= 0.0 {
            light.ew = light.ew.next();
            light.timer = light_secs(light.ew);
            info!(
                "[交通] 路口{} 东西{} / 南北{}",
                light.junction,
                light.ew.name(),
                light.ns().name(),
            );
        }
        let ns = light.ns();
        // 每套灯杆按自己的相位亮灯（自发光），其余熄灭
        for (rig, children) in &rigs {
            if rig.junction != light.junction {
                continue;
            }
            let state = if rig.phase == 0 { light.ew } else { ns };
            for child in children.iter() {
                let Ok((bulb, mat)) = bulbs.get_mut(child) else {
                    continue;
                };
                let Some(mut m) = materials.get_mut(&mat.0) else {
                    continue;
                };
                let on = bulb.kind == state;
                m.emissive = match bulb.kind {
                    LightState::Red if on => LinearRgba::new(1.0, 0.15, 0.08, 1.0),
                    LightState::Yellow if on => LinearRgba::new(1.0, 0.85, 0.15, 1.0),
                    LightState::Green if on => LinearRgba::new(0.15, 1.0, 0.3, 1.0),
                    _ => LinearRgba::BLACK,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_road_phases_are_complementary() {
        // 东西向绿灯 ⇔ 南北向红灯；黄灯期间对向保持红灯 → 两方向绝不同时放行
        assert_eq!(LightState::Green.complement(), LightState::Red);
        assert_eq!(LightState::Red.complement(), LightState::Green);
        assert_eq!(LightState::Yellow.complement(), LightState::Red);
        let l = JunctionLight {
            junction: 0,
            ew: LightState::Green,
            timer: 0.0,
        };
        assert_eq!(l.ew, LightState::Green);
        assert_eq!(l.ns(), LightState::Red);
    }

    #[test]
    fn phase_cycle_follows_red_green_yellow() {
        // 依序循环 红→绿→黄→红，且各相位时长为正
        assert_eq!(LightState::Red.next(), LightState::Green);
        assert_eq!(LightState::Green.next(), LightState::Yellow);
        assert_eq!(LightState::Yellow.next(), LightState::Red);
        for s in [LightState::Red, LightState::Green, LightState::Yellow] {
            assert!(light_secs(s) > 0.0);
        }
    }

    #[test]
    fn junction_stop_waits_at_red_and_passes_on_green() {
        // 迷你路口 (0,0)：东西向红灯 → 东西向车停在停车线 (-8,0)；绿灯直接通行
        let junctions = &[(Vec2::ZERO, 0)];
        let red = [(LightState::Red, LightState::Green); 1];
        assert_eq!(
            junction_stop(Vec2::new(-10.0, 0.0), Vec2::X, &red, junctions),
            Some(Vec2::new(-8.0, 0.0)),
            "东西向红灯应在停车线停"
        );
        let green = [(LightState::Green, LightState::Red); 1];
        assert_eq!(
            junction_stop(Vec2::new(-10.0, 0.0), Vec2::X, &green, junctions),
            None,
            "东西向绿灯直接通行"
        );
        // 南北向车看南北向灯：南北向绿灯 → 通行（不受东西向红灯影响）
        assert_eq!(
            junction_stop(Vec2::new(0.0, -10.0), Vec2::Y, &red, junctions),
            None,
            "南北向绿灯应通行"
        );
        // 黄灯同样停车
        let yellow = [(LightState::Yellow, LightState::Red); 1];
        assert_eq!(
            junction_stop(Vec2::new(-10.0, 0.0), Vec2::X, &yellow, junctions),
            Some(Vec2::new(-8.0, 0.0)),
            "东西向黄灯应停车"
        );
    }
}
