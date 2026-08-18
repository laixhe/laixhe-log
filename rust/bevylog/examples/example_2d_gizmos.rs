//! Bevy 0.19 入门示例：演示 Gizmos 调试绘制。
//! 用 Gizmos 画出坐标轴、直线、圆、矩形和箭头，可视化位置、形状与向量。
//!
//! 学习重点：
//! - Gizmos 是「每帧自动清除」的调试绘制系统参数，不创建实体、不影响游戏逻辑
//! - 常用方法：line_2d（直线）、circle_2d（圆）、rect_2d（矩形）、arrow_2d（箭头）
//! - 位置用 Vec2（2D 坐标系，原点在屏幕中心），颜色用 Color::srgb
//! - Gizmos 由 DefaultPlugins 里的 GizmoPlugin 提供，无需额外注册

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        // 每帧绘制 Gizmos（Gizmos 每帧自动清除，所以能看到动态效果）
        .add_systems(Update, draw_gizmos)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn draw_gizmos(mut gizmos: Gizmos, time: Res<Time>) {
    let t = time.elapsed_secs();

    // 1. 坐标轴：过原点的两条灰色直线
    gizmos.line_2d(
        Vec2::new(-400.0, 0.0),
        Vec2::new(400.0, 0.0),
        Color::srgb(0.3, 0.3, 0.3),
    );
    gizmos.line_2d(
        Vec2::new(0.0, -300.0),
        Vec2::new(0.0, 300.0),
        Color::srgb(0.3, 0.3, 0.3),
    );

    // 2. 一个绕原点旋转的点
    let point = Vec2::new(t.cos() * 200.0, t.sin() * 200.0);

    // 3. 原点到点的连线（演示「位置向量」）
    gizmos.line_2d(Vec2::ZERO, point, Color::srgb(0.2, 0.8, 0.2));

    // 4. 点周围的圆（半径 40）
    gizmos.circle_2d(point, 40.0, Color::srgb(0.2, 0.6, 1.0));

    // 5. 固定的矩形边界框（中心 + 尺寸）
    gizmos.rect_2d(
        Vec2::new(-50.0, -50.0),
        Vec2::new(100.0, 100.0),
        Color::srgb(1.0, 0.8, 0.2),
    );

    // 6. 从点沿切线方向伸出的箭头（演示「方向向量」）
    let tangent = Vec2::new(-t.sin(), t.cos()) * 80.0;
    gizmos.arrow_2d(point, point + tangent, Color::srgb(1.0, 0.3, 0.3));
}
