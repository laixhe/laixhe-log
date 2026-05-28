use bevy::prelude::*;

// 玩家
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 生成文本
    commands.spawn((
        Text2d::new("Movement"),
        TextFont {
            // asset_server: Res<AssetServer>
            // font: asset_server.load("fonts/xxx.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Player,
    ));
}

fn move_player(
    // 键盘输入
    input: Res<ButtonInput<KeyCode>>,
    // 游戏计时器
    time: Res<Time>,
    // 玩家的位置(要求 Bevy 提供恰好一个同时带有 Transform 组件和 Player 标签的实体)
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    // 二维向量 Vec2::ZERO == Vec2 { x: 0.0, y: 0.0 }
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if direction != Vec2::ZERO {
        // 像素/秒
        let speed = 300.0;
        // normalize() 将向量长度归一化为 1
        // time.delta_secs() 返回帧时间（即自上一帧到下一帧的时间）
        let delta = direction.normalize() * speed * time.delta_secs();
        player_transform.translation.x += delta.x;
        player_transform.translation.y += delta.y;
    }
}
