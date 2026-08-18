//! Bevy 0.19 入门示例：演示相机边界限制（Camera Bounds）。
//!
//! 学习重点：
//! - 计算相机可见的世界范围：窗口尺寸 × 投影 scale
//! - 把相机中心 clamp 到「世界边界 - 半视野」范围内，避免看到世界外
//! - 相机移动与限制组合，形成「镜头出不去地图」的效果
//!
//! 操作：WASD / 方向键移动相机，观察无法移出边框。

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// 世界范围（半宽、半高）
const WORLD_HALF: Vec2 = Vec2::new(600.0, 400.0);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 用四条细长矩形画出世界边框
    let border = Color::srgb(0.5, 0.5, 0.6);
    let thickness = 6.0;
    let w = WORLD_HALF.x * 2.0;
    let h = WORLD_HALF.y * 2.0;
    let edges = [
        (Vec2::new(0.0, WORLD_HALF.y), Vec2::new(w, thickness)),
        (Vec2::new(0.0, -WORLD_HALF.y), Vec2::new(w, thickness)),
        (Vec2::new(WORLD_HALF.x, 0.0), Vec2::new(thickness, h)),
        (Vec2::new(-WORLD_HALF.x, 0.0), Vec2::new(thickness, h)),
    ];
    for (pos, size) in edges {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            MeshMaterial2d(materials.add(ColorMaterial::from(border))),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }

    // 放一些网格参照点
    for i in -3..=3 {
        for j in -2..=2 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(6.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.4)))),
                Transform::from_xyz(i as f32 * 180.0, j as f32 * 180.0, 0.0),
            ));
        }
    }
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    projection: Single<&Projection, With<Camera2d>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    // 1. 键盘移动相机
    let mut dir = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        dir.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        dir.x += 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        dir.y += 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        dir.y -= 1.0;
    }
    if dir != Vec2::ZERO {
        camera.translation += (dir.normalize() * 400.0 * time.delta_secs()).extend(0.0);
    }

    // 2. 计算相机可见的半视野（世界单位）
    let proj = projection.into_inner();
    let Projection::Orthographic(ortho) = proj else {
        return;
    };
    let half_view = Vec2::new(window.width(), window.height()) / 2.0 * ortho.scale;

    // 3. 相机中心可移动范围：世界边界向内收半视野
    let clamp_x = (WORLD_HALF.x - half_view.x).max(0.0);
    let clamp_y = (WORLD_HALF.y - half_view.y).max(0.0);

    camera.translation.x = camera.translation.x.clamp(-clamp_x, clamp_x);
    camera.translation.y = camera.translation.y.clamp(-clamp_y, clamp_y);
}
