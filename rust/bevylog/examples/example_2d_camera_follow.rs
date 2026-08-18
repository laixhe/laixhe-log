//! Bevy 0.19 入门示例：演示相机平滑跟随（Camera Follow）。
//!
//! 学习重点：
//! - 用键盘移动玩家，相机平滑跟随玩家
//! - 指数平滑（lerp + 帧率无关的 factor）实现跟随
//! - 单实体查询 Single 访问玩家与相机
//!
//! 操作：WASD 移动玩家，相机会平滑地跟随。

use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, camera_follow))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 玩家
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.4, 0.4)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 背景参照物（静止的圆点，方便看出相机在移动）
    for i in 0..5 {
        for j in 0..5 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(8.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.4)))),
                Transform::from_xyz(i as f32 * 150.0 - 300.0, j as f32 * 150.0 - 300.0, 0.0),
            ));
        }
    }
}

// WASD 移动玩家
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    let speed = 220.0;
    let dir = dir.normalize_or_zero();
    player.translation.x += dir.x * speed * time.delta_secs();
    player.translation.y += dir.y * speed * time.delta_secs();
}

// 相机平滑跟随玩家（指数平滑，帧率无关）
fn camera_follow(
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let target = player.translation.truncate();
    let current = camera.translation.truncate();
    let factor = 1.0 - (-5.0 * time.delta_secs()).exp();
    let new = current.lerp(target, factor);
    camera.translation.x = new.x;
    camera.translation.y = new.y;
}
