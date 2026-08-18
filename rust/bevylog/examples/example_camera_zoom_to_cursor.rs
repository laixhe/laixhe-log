//! Bevy 0.19 入门示例：演示相机缩放跟随鼠标（Zoom to Cursor）。
//!
//! 学习重点：
//! - 缩放时保持鼠标指向的世界点不动（镜头朝鼠标位置拉近/拉远）
//! - 理解 2D 正交相机：世界点 = 相机中心 + 屏幕偏移 × scale
//! - AccumulatedMouseScroll 读取滚轮滚动量
//!
//! 操作：把鼠标移到想聚焦的位置，滚动滚轮缩放。

use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, zoom_to_cursor)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 网格参照点，缩放时能明显看到「鼠标处不动、四周移动」
    for i in -4..=4 {
        for j in -3..=3 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(8.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.35, 0.45)))),
                Transform::from_xyz(i as f32 * 120.0, j as f32 * 120.0, 0.0),
            ));
        }
    }
}

fn zoom_to_cursor(
    scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window, With<PrimaryWindow>>,
    projection: Single<&mut Projection, With<Camera2d>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    if scroll.delta.y == 0.0 {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    // 旧的缩放倍率
    let mut proj = projection.into_inner();
    let Projection::Orthographic(ortho) = &mut *proj else {
        return;
    };
    let old_scale = ortho.scale;

    // 更新缩放倍率：滚轮向上（delta.y > 0）放大（scale 变小）
    let new_scale = (old_scale * (1.0 - scroll.delta.y * 0.1)).clamp(0.1, 10.0);
    ortho.scale = new_scale;

    // 鼠标相对屏幕中心的偏移（像素，屏幕坐标 +y 朝下）
    let center = Vec2::new(window.width(), window.height()) / 2.0;
    let rel = cursor - center;
    // 转成世界坐标（+y 朝上）
    let rel_world = Vec2::new(rel.x, -rel.y);

    // 缩放前后，鼠标指向的世界点变化量 = rel_world * (old_scale - new_scale)
    // 反向补偿相机位置，让鼠标处保持不动
    camera.translation += (rel_world * (old_scale - new_scale)).extend(0.0);
}
