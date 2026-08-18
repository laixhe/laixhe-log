//! Bevy 0.19 入门示例：演示 2D 拖拽（Pointer<Drag> 事件）。
//!
//! 学习重点：
//! - On<Pointer<Drag>>：用 observer 监听拖拽事件
//! - Camera::viewport_to_world_2d：把屏幕坐标转换成世界坐标
//! - 拖拽时把物体移动到指针所在的世界位置
//!
//! 操作：按住并拖动圆，它会跟随鼠标移动。

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

    // 可拖拽的圆
    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(60.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.4, 0.4)))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .observe(drag);
}

// 拖拽：把物体移动到指针所在的世界坐标
fn drag(
    event: On<Pointer<Drag>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut q: Query<&mut Transform>,
) {
    let (camera, cam_tf) = *camera;
    let screen_pos = event.event().pointer_location.position;

    if let Ok(world) = camera.viewport_to_world_2d(cam_tf, screen_pos) {
        if let Ok(mut tf) = q.get_mut(event.event().entity) {
            tf.translation.x = world.x;
            tf.translation.y = world.y;
        }
    }
}
