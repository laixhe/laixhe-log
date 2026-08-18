//! Bevy 0.19 入门示例：演示网格布局（程序化排列物体）。
//!
//! 学习重点：
//! - 用嵌套循环程序化生成排列整齐的物体
//! - 共享同一个 Mesh 句柄，减少资产数量
//! - 用 (x + y) % 2 生成棋盘格交替效果
//!
//! 观察：8x8 棋盘格，颜色交替排列。

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.07)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 共享同一个矩形网格
    let rect = meshes.add(Rectangle::new(48.0, 48.0));
    let light = materials.add(ColorMaterial::from(Color::srgb(0.30, 0.35, 0.45)));
    let dark = materials.add(ColorMaterial::from(Color::srgb(0.15, 0.18, 0.24)));

    for x in 0..8 {
        for y in 0..8 {
            let mat = if (x + y) % 2 == 0 {
                light.clone()
            } else {
                dark.clone()
            };

            commands.spawn((
                Mesh2d(rect.clone()),
                MeshMaterial2d(mat),
                Transform::from_xyz(x as f32 * 54.0 - 189.0, y as f32 * 54.0 - 189.0, 0.0),
            ));
        }
    }
}
