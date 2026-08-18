//! Bevy 0.19 入门示例：演示 2D 拾取（MeshPicking + Click 事件）。
//!
//! 学习重点：
//! - MeshPickingPlugin：mesh 射线拾取后端（默认所有 Mesh 可拾取）
//! - On<Pointer<Click>>：用 observer 监听实体的点击事件
//! - Pointer 事件里携带被点击的实体、指针位置等信息
//!
//! 操作：点击屏幕上的圆或方块，物体会变色，终端打印被点击实体。

use bevy::picking::prelude::*;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 可点击的圆
    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(70.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.4, 0.4)))),
            Transform::from_xyz(-110.0, 0.0, 0.0),
        ))
        .observe(on_click);

    // 可点击的方块
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(140.0, 140.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.4, 0.5, 0.9)))),
            Transform::from_xyz(110.0, 0.0, 0.0),
        ))
        .observe(on_click);
}

// 点击事件：把被点击物体的颜色改成黄色
fn on_click(
    event: On<Pointer<Click>>,
    q_material: Query<&MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = event.event().entity;
    info!("[2D拾取] 点击了实体 {:?}", entity);

    if let Ok(mat) = q_material.get(entity) {
        if let Some(mut material) = materials.get_mut(&mat.0) {
            material.color = Color::srgb(0.9, 0.8, 0.3);
        }
    }
}
