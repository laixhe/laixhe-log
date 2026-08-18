//! 朝向工具：把模型旋转到世界 XZ 平面上的某个方向，供各模块复用。
//! 模型有两种常见的前向约定，按需选用：
//! - [`facing_x`]：车头在模型局部 +X（本项目车辆的约定，车头灯位于 spawn_car 的 +X 侧）；
//! - [`facing_z`]：正面对局部 +Z（行人 / 玩家 / 装饰 NPC 的默认约定）。

use bevy::prelude::*;

/// 让「局部 +X 前向」的模型（如车辆）指向世界方向 `dir`（XZ 平面的 Vec2(x, z)）。
pub fn facing_x(dir: Vec2) -> Quat {
    Quat::from_rotation_y((-dir.y).atan2(dir.x))
}

/// 让「局部 +Z 前向」的模型（如行人 / 玩家）指向世界方向 `dir`（XZ 平面的 Vec2(x, z)）。
pub fn facing_z(dir: Vec2) -> Quat {
    Quat::from_rotation_y(dir.x.atan2(dir.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 把局部前向向量经旋转映射到世界 XZ 平面
    fn map(dir: Vec2, local: Vec3, f: fn(Vec2) -> Quat) -> Vec2 {
        let v = f(dir) * local;
        Vec2::new(v.x, v.z).normalize_or_zero()
    }

    #[test]
    fn facing_x_points_to_drive_direction() {
        // 车头在模型局部 +X，旋转后应指向行驶方向（含四个主轴与斜向）
        for dir in [
            Vec2::X,
            Vec2::Y,
            -Vec2::X,
            -Vec2::Y,
            Vec2::new(1.0, 1.0).normalize(),
        ] {
            let f = map(dir, Vec3::X, facing_x);
            assert!(f.distance(dir) < 0.01, "局部+X 应指向 {dir:?}，实际 {f:?}");
        }
    }

    #[test]
    fn facing_z_points_to_direction() {
        // 正面对局部 +Z，旋转后应指向行走方向
        for dir in [
            Vec2::X,
            Vec2::Y,
            -Vec2::X,
            -Vec2::Y,
            Vec2::new(-1.0, 2.0).normalize(),
        ] {
            let f = map(dir, Vec3::Z, facing_z);
            assert!(f.distance(dir) < 0.01, "局部+Z 应指向 {dir:?}，实际 {f:?}");
        }
    }
}
