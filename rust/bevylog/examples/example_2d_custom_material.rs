//! Bevy 0.19 入门示例：演示自定义 2D 材质（Material2d + WGSL 着色器）。
//!
//! 除了内置的 ColorMaterial，Bevy 允许自定义材质：用 #[derive(AsBindGroup)] 定义
//! 传给 GPU 的数据（uniform），用 Material2d trait 指定着色器文件，再用
//! Material2dPlugin 注册，之后就能用 MeshMaterial2d 使用该材质。
//!
//! 学习重点：
//! - #[derive(AsBindGroup, Asset, TypePath)] 定义材质 + uniform 数据
//! - impl Material2d，指定 fragment_shader（WGSL 文件）
//! - Material2dPlugin 注册材质
//! - MeshMaterial2d<MyMaterial> 使用自定义材质

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

// 自定义材质：一个颜色 uniform（对应 shaders/my_material.wgsl 里的 @binding(0)）
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct MyMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for MyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/my_material.wgsl".into()
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册自定义材质
        .add_plugins(Material2dPlugin::<MyMaterial>::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
) {
    commands.spawn(Camera2d);

    // 用自定义材质渲染两个不同颜色的圆
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(80.0))),
        MeshMaterial2d(materials.add(MyMaterial {
            color: LinearRgba::rgb(0.2, 0.8, 0.4),
        })),
        Transform::from_xyz(-120.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(80.0))),
        MeshMaterial2d(materials.add(MyMaterial {
            color: LinearRgba::rgb(0.9, 0.6, 0.2),
        })),
        Transform::from_xyz(120.0, 0.0, 0.0),
    ));
}
