//! Bevy 0.19 入门示例：演示自定义 3D 材质（Material + WGSL 着色器）。
//!
//! 类似 example_2d_custom_material，但用于 3D：用 Material trait（而不是 Material2d）
//! 定义材质，配合 MeshMaterial3d 渲染 3D 网格。
//!
//! 学习重点：
//! - `#[derive(AsBindGroup, Asset, TypePath)]` 定义材质 + uniform 数据
//! - `impl Material`，只实现 `fragment_shader`（顶点着色器用默认）
//! - `MaterialPlugin` 注册材质
//! - `Mesh3d` + `MeshMaterial3d<MyMaterial3d>` 使用自定义材质
//! - 3D 材质绑定组是 `@group(3)`（bindless 渲染），而 2D 是 `@group(2)`

use bevy::pbr::{Material, MaterialPlugin};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

// 自定义 3D 材质：一个颜色 uniform（对应 shaders/my_material_3d.wgsl 里的 @group(3) @binding(0)）
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct MyMaterial3d {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for MyMaterial3d {
    fn fragment_shader() -> ShaderRef {
        "shaders/my_material_3d.wgsl".into()
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册自定义材质
        .add_plugins(MaterialPlugin::<MyMaterial3d>::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial3d>>,
) {
    // 3D 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 两个用自定义材质渲染的 3D 物体
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(MyMaterial3d {
            color: LinearRgba::rgb(0.2, 0.8, 0.4),
        })),
        Transform::from_xyz(-1.2, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(MyMaterial3d {
            color: LinearRgba::rgb(0.9, 0.6, 0.2),
        })),
        Transform::from_xyz(1.2, 0.0, 0.0),
    ));
}
