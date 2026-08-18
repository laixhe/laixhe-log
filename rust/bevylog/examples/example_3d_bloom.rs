//! Bevy 0.19 入门示例：演示泛光后处理（Bloom）。
//!
//! 学习重点：
//! - Bloom 组件挂在相机上启用泛光效果
//! - 发光材质：StandardMaterial::emissive 使用 >1 的 LinearRgba 值（HDR）
//! - Tonemapping 把 HDR 颜色映射到显示器可显示范围
//!
//! 观察：三个高亮发光球体周围产生光晕。

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机：启用泛光 + 亮度映射
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Transform::from_xyz(0.0, 1.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom::NATURAL,
    ));

    // 三个发光球体（emissive 值 > 1 产生 HDR 光晕）
    let emissives = [
        LinearRgba::rgb(8.0, 0.0, 0.0), // 红
        LinearRgba::rgb(0.0, 8.0, 0.0), // 绿
        LinearRgba::rgb(0.0, 0.0, 8.0), // 蓝
    ];
    for (i, emissive) in emissives.iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                emissive: *emissive,
                ..default()
            })),
            Transform::from_xyz((i as f32 - 1.0) * 2.0, 0.5, 0.0),
        ));
    }
}
