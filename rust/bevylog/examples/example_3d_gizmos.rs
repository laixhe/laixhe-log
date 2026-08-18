//! Bevy 0.19 入门示例：演示 3D Gizmos 调试绘制。
//!
//! 与 example_2d_gizmos 的 2D 绘制不同，本示例在 3D 空间中绘制线框，
//! 用于可视化 3D 坐标轴、球体、圆和箭头。
//!
//! 学习重点：
//! - gizmos.line / arrow：3D 直线 / 箭头（Vec3）
//! - gizmos.sphere / circle：3D 球体 / 圆（用 Isometry3d 定位）
//! - 3D Gizmos 需要 Camera3d，且每帧自动清除

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gizmos)
        .run()
}

fn setup(mut commands: Commands) {
    // 3D 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn draw_gizmos(mut gizmos: Gizmos, time: Res<Time>) {
    let t = time.elapsed_secs();

    // 1. 3D 坐标轴（红 X、绿 Y、蓝 Z）
    gizmos.line(
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Color::srgb(0.8, 0.2, 0.2),
    );
    gizmos.line(
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Color::srgb(0.2, 0.8, 0.2),
    );
    gizmos.line(
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(0.0, 0.0, 2.0),
        Color::srgb(0.2, 0.4, 0.9),
    );

    // 2. 一个绕 Y 轴旋转的点（在 XZ 平面做椭圆运动）
    let point = Vec3::new(t.cos() * 1.5, (t * 1.5).sin() * 0.8, t.sin() * 1.5);

    // 3. 球体线框（画在运动点上，随点移动）
    gizmos.sphere(
        Isometry3d::from_translation(point),
        0.3,
        Color::srgb(1.0, 0.8, 0.2),
    );

    // 4. 圆（画在原点，XZ 平面，半径 1）
    gizmos.circle(
        Isometry3d::from_translation(Vec3::ZERO),
        1.0,
        Color::srgb(0.2, 0.8, 0.4),
    );

    // 5. 箭头（从原点到运动点，展示方向）
    gizmos.arrow(Vec3::ZERO, point, Color::srgb(1.0, 0.3, 0.3));
}
