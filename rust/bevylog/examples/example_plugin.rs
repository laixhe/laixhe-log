//! Bevy 0.19 入门示例：演示自定义 Plugin（把系统 / 资源组织进插件）。
//! 把相机、玩家生成、移动等逻辑打包进一个 Plugin，App 只需一行 add_plugins 装配。
//!
//! 学习重点：
//! - Plugin trait：实现 build(&self, app) 方法，在 app 上注册系统 / 资源 / 子插件
//! - 插件化组织：把相关功能打包成一个插件，App 里一行 add_plugins 装配
//! - 这是 Bevy 惯用的代码组织方式（DefaultPlugins 本身就是一组插件的集合）

use bevy::prelude::*;

// 玩家移动速度组件
#[derive(Component)]
struct Speed(f32);

// 玩家标记
#[derive(Component)]
struct Player;

// 自定义插件：把「玩家生成 + 移动」相关功能打包
struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // 在插件里注册系统：App 装配这个插件时，系统会被自动加入
        app.add_systems(Startup, setup)
            .add_systems(Update, move_player);
    }
}

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        // 装配默认插件组 + 自定义插件
        .add_plugins((DefaultPlugins, MovementPlugin))
        .run()
}

// 由插件注册的启动系统：生成相机和玩家
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Speed(300.0),
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::default(),
    ));
}

// 由插件注册的更新系统：移动玩家
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&Speed, &mut Transform), With<Player>>,
) {
    let (speed, mut transform) = player.into_inner();
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
        transform.translation += (direction.normalize() * speed.0 * time.delta_secs()).extend(0.0);
    }
}
