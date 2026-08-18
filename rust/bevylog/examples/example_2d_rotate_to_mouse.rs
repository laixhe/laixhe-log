//! Bevy 0.19 入门示例：演示物体朝向鼠标（2D 旋转）。
//!
//! 学习重点：
//! - Window::cursor_position 获取鼠标屏幕坐标
//! - Camera::viewport_to_world_2d 把屏幕坐标转成世界坐标
//! - atan2 计算物体到鼠标的角度，Quat::from_rotation_z 设置旋转
//!
//! 观察：长条矩形始终旋转指向鼠标位置。

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Arrow;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_to_mouse)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 长条矩形（默认长边朝 +x）
    commands.spawn((
        Arrow,
        Mesh2d(meshes.add(Rectangle::new(90.0, 16.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.6, 0.3)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn rotate_to_mouse(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut q: Query<&mut Transform, With<Arrow>>,
) {
    let (camera, cam_tf) = *camera;
    let Some(screen_pos) = window.cursor_position() else {
        return;
    };
    let Ok(mouse_world) = camera.viewport_to_world_2d(cam_tf, screen_pos) else {
        return;
    };

    for mut tf in &mut q {
        let delta = mouse_world - tf.translation.truncate();
        let angle = delta.y.atan2(delta.x);
        tf.rotation = Quat::from_rotation_z(angle);
    }
}
