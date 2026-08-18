//! Bevy 0.19 入门示例：演示颜色循环动画（动态修改 ColorMaterial）。
//!
//! 学习重点：
//! - ColorMaterial 的颜色可以运行时修改
//! - 用 sin 让 RGB 三个分量循环变化，产生颜色渐变
//! - ResMut<Assets<ColorMaterial>> 访问并修改材质
//!
//! 观察：圆形的颜色在 RGB 之间平滑循环变化。

use bevy::prelude::*;

#[derive(Component)]
struct ColorCycle;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, cycle)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        ColorCycle,
        Mesh2d(meshes.add(Circle::new(90.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::default(),
    ));
}

// 让颜色在 RGB 之间循环
fn cycle(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q: Query<&MeshMaterial2d<ColorMaterial>, With<ColorCycle>>,
) {
    let t = time.elapsed_secs();
    for mat in &q {
        if let Some(mut material) = materials.get_mut(&mat.0) {
            let r = t.sin() * 0.5 + 0.5;
            let g = (t + 2.0).sin() * 0.5 + 0.5;
            let b = (t + 4.0).sin() * 0.5 + 0.5;
            material.color = Color::srgb(r, g, b);
        }
    }
}
