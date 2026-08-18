//! Bevy 0.19 入门示例：演示 2D 相机控制（移动 + 缩放）。
//! WASD / 方向键移动相机，滚轮缩放，观察「世界不动、相机在动」的效果。
//!
//! 学习重点：
//! - 相机本身也是一个实体，用 Single<&mut Transform, With<Camera2d>> 查询并移动它的 Transform
//! - 相机的 Transform.translation 决定「看向哪里」；移动相机 = 移动它的 Transform
//! - 缩放用 OrthographicProjection.scale（scale 越小画面越放大，越大越缩小）
//! - Projection 是枚举（Perspective/Orthographic/Custom），2D 相机是 Orthographic 变体，需匹配取出

use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera, zoom_camera))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 在场景里放三个圆，移动 / 缩放相机时观察它们的变化
    let colors = [
        Color::srgb(0.2, 0.6, 1.0),
        Color::srgb(0.9, 0.3, 0.3),
        Color::srgb(0.3, 0.9, 0.4),
    ];
    for (i, color) in colors.into_iter().enumerate() {
        let x = (i as f32 - 1.0) * 200.0;
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(40.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }
}

// WASD / 方向键移动相机
fn move_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 400.0;
        camera.translation += (direction.normalize() * speed * time.delta_secs()).extend(0.0);
    }
}

// 滚轮缩放相机（改 OrthographicProjection.scale）
fn zoom_camera(
    scroll: Res<AccumulatedMouseScroll>,
    projection: Single<&mut Projection, With<Camera2d>>,
) {
    if scroll.delta.y == 0.0 {
        return;
    }
    // into_inner() 取出内部的 Mut<Projection>，再用 &mut * 解引用得到 &mut Projection
    let mut proj = projection.into_inner();
    let Projection::Orthographic(ortho) = &mut *proj else {
        return;
    };
    // 滚轮向上（delta.y > 0）放大，向下缩小；clamp 限制缩放范围
    ortho.scale = (ortho.scale * (1.0 - scroll.delta.y * 0.1)).clamp(0.1, 10.0);
    info!("[相机] 缩放 = {:.2}", ortho.scale);
}
