//! Bevy 0.19 入门示例：演示 3D 纹理贴图（Texture Mapping）。
//!
//! 学习重点：
//! - StandardMaterial::base_color_texture 指定纹理贴图
//! - AssetServer::load 加载图片资源
//! - 内置图元（如 Cuboid）自带 UV 坐标，直接把图片贴到表面
//!
//! 观察：bevy_logo 图片贴在旋转的立方体表面。

use bevy::prelude::*;

#[derive(Component)]
struct Spin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 3D 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 加载纹理图片
    let texture: Handle<Image> = asset_server.load("images/bevy_logo.png");

    // 带纹理的立方体
    commands.spawn((
        Spin,
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            ..default()
        })),
        Transform::default(),
    ));
}

// 让立方体绕 Y 轴旋转，展示各面纹理
fn spin(time: Res<Time>, mut q: Query<&mut Transform, With<Spin>>) {
    for mut tf in &mut q {
        tf.rotate_y(time.delta_secs());
    }
}
