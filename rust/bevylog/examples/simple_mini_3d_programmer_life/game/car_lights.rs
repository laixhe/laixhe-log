//! 独立可复用的车辆车灯系统（不依赖具体车辆模型）：
//! - 转向灯：行驶方向明显变化时，对应侧（左 / 右）橙色灯闪烁一段时间后自动熄灭；
//! - 刹车灯：车辆减速 / 停车时，车尾红色灯亮起；
//! - 车头大灯：晚上（`Phase::Evening`）自动开启，白天熄灭。
//!
//! 使用方式：车辆实体挂载 `TurnLight` / `HeadLight` / `BrakeLight` 子灯（各灯存自己的
//! 材质 Handle，emissive 由本系统动态切换），并注册 `car_lights_tick` 系统即可；
//! 转向状态（`turn` / `turn_timer`）由车辆的行驶逻辑写入 `Vehicle` 组件，本系统只读。

use bevy::prelude::*;
use std::collections::HashMap;

use super::resources::{GameClock, Phase};
use super::traffic::Vehicle;

// ==================== 组件 ====================

/// 转向方向（车头朝局部 +X，左侧 +Z / 右侧 -Z）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TurnSide {
    Left,
    Right,
}

/// 车辆转向灯（挂在车头 / 车尾左右两侧的子实体上）；同侧两灯共享同一材质，
/// 转弯时对应侧会一起闪烁。
#[derive(Component)]
pub struct TurnLight {
    pub side: TurnSide,
    pub mat: Handle<StandardMaterial>, // 该侧转向灯材质（emissive 动态切换）
}

/// 车头大灯：晚上开启照亮，白天熄灭
#[derive(Component)]
pub struct HeadLight {
    pub mat: Handle<StandardMaterial>,
}

/// 车尾刹车灯：车辆减速 / 停车时亮起（红色）
#[derive(Component)]
pub struct BrakeLight {
    pub mat: Handle<StandardMaterial>,
}

// ==================== 参数与灯色 ====================

// 转向灯参数
const TURN_THRESHOLD: f32 = 0.25; // 判定转向的最小方向变化角（弧度，约 14°）
/// 转向灯持续亮起时长（秒，期间闪烁）；车辆行驶逻辑触发转向时使用
pub const TURN_DURATION: f32 = 2.0;
const TURN_BLINK_PERIOD: f32 = 0.4; // 转向灯闪烁半周期（秒）

// 各灯点亮时的自发光颜色
const TURN_ON: LinearRgba = LinearRgba::new(1.0, 0.65, 0.12, 1.0); // 转向灯橙黄
const HEADLIGHT_ON: LinearRgba = LinearRgba::new(1.0, 0.9, 0.5, 1.0); // 大灯暖白
const BRAKE_ON: LinearRgba = LinearRgba::new(1.0, 0.08, 0.05, 1.0); // 刹车灯红

// ==================== 判定（纯函数，便于测试） ====================

/// 判断行驶方向变化是否构成转向：返回应打哪一侧转向灯。
/// 直行（方向变化 < 阈值）或掉头（≈π）不算转向；
/// 左 / 右按「上一方向 → 当前方向」的旋向判定（俯视顺时针 = 右转）。
pub fn detect_turn(last_dir: Vec2, cur_dir: Vec2) -> Option<TurnSide> {
    if last_dir.length_squared() < 0.001 || cur_dir.length_squared() < 0.001 {
        return None; // 静止（无方向）不算转向
    }
    let angle = last_dir.angle_to(cur_dir).abs();
    if !(TURN_THRESHOLD..=std::f32::consts::PI * 0.9).contains(&angle) {
        return None; // 直行或掉头
    }
    let cross = last_dir.x * cur_dir.y - last_dir.y * cur_dir.x;
    Some(if cross > 0.0 {
        TurnSide::Right
    } else {
        TurnSide::Left
    })
}

/// 判定车辆本帧是否在减速 / 停车：实际位移不足全速步长即为刹车。
pub fn is_braking(moved: f32, full_step: f32) -> bool {
    moved < full_step - 0.001
}

// ==================== 车灯系统 ====================

/// 车灯主系统：读取车辆的转向状态（`Vehicle::turn` / `turn_timer`）与位移，更新
/// 所有车灯的 emissive。应在车辆的行驶逻辑（更新转向状态、移动）之后运行。
#[allow(clippy::too_many_arguments)]
pub fn car_lights_tick(
    time: Res<Time>,
    clock: Res<GameClock>,
    vehicles: Query<(Entity, &Vehicle, &Transform, &Children)>,
    turn_lights: Query<&TurnLight>,
    head_lights: Query<&HeadLight>,
    brake_lights: Query<&BrakeLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut last_pos: Local<HashMap<Entity, Vec3>>, // 记录每辆车上一帧位置（算刹车）；退出场景后残留条目很小，且 Entity 带 generation 不会误命中新车辆
) {
    let dt = time.delta_secs();
    let night = clock.phase == Phase::Evening;
    for (e, v, tf, children) in &vehicles {
        let cur = tf.translation;
        let prev = last_pos.get(&e).copied().unwrap_or(cur);
        last_pos.insert(e, cur);
        // 刹车：本帧实际位移不足全速
        let braking = is_braking((cur - prev).xz().length(), v.speed * dt);
        // 转向灯闪烁相位（由剩余时长决定亮 / 灭）：从「亮」开始闪烁
        let blink_on =
            v.turn_timer > 0.0 && (((v.turn_timer / TURN_BLINK_PERIOD) as i32 + 1) % 2 == 0);

        // 车头大灯：晚上开启，白天熄灭
        for child in children.iter() {
            let Ok(light) = head_lights.get(child) else {
                continue;
            };
            if let Some(mut m) = materials.get_mut(&light.mat) {
                m.emissive = if night {
                    HEADLIGHT_ON
                } else {
                    LinearRgba::BLACK
                };
            }
        }
        // 刹车灯：减速 / 停车时亮起
        for child in children.iter() {
            let Ok(light) = brake_lights.get(child) else {
                continue;
            };
            if let Some(mut m) = materials.get_mut(&light.mat) {
                m.emissive = if braking { BRAKE_ON } else { LinearRgba::BLACK };
            }
        }
        // 转向灯：对应侧闪烁
        for child in children.iter() {
            let Ok(light) = turn_lights.get(child) else {
                continue;
            };
            let on = v.turn == Some(light.side) && blink_on;
            if let Some(mut m) = materials.get_mut(&light.mat) {
                m.emissive = if on { TURN_ON } else { LinearRgba::BLACK };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_signal_detects_left_and_right() {
        // 直行 / 小角度波动 → 不打灯
        assert_eq!(detect_turn(Vec2::X, Vec2::X), None);
        let slight = Vec2::new(1.0, 0.05).normalize();
        assert_eq!(detect_turn(Vec2::X, slight), None, "小角度波动不算转向");
        // 从 +X 转到 +Z（俯视顺时针）= 右转；从 +X 转到 -Z = 左转
        assert_eq!(detect_turn(Vec2::X, Vec2::Y), Some(TurnSide::Right));
        assert_eq!(detect_turn(Vec2::X, -Vec2::Y), Some(TurnSide::Left));
        // 掉头（≈π）不打转向灯
        assert_eq!(detect_turn(Vec2::X, -Vec2::X), None, "掉头不算转向");
        // 静止（无方向）不算转向
        assert_eq!(detect_turn(Vec2::ZERO, Vec2::X), None);
    }

    #[test]
    fn braking_detects_slow_and_stop() {
        // 停车（位移 0）与减速（位移不足全速）都应亮刹车灯；全速前进不亮
        assert!(is_braking(0.0, 0.18), "停车应亮刹车灯");
        assert!(is_braking(0.06, 0.18), "减速应亮刹车灯");
        assert!(!is_braking(0.18, 0.18), "全速前进不亮刹车灯");
    }
}
