use bevy::prelude::*;
use bevy::text::FontSource;
use rand::RngExt;

use crate::pages::router::GameState;

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
// 收集音效（与 example_audio 共用）
const BLIP_SOUND: &str = "audio/blip.wav";

// ==================== 游戏常量配置 ====================
const GRAVITY: f32 = -9.8; // 重力加速度（世界单位/秒²）
const GROUND_Y: f32 = 0.0; // 地面高度（Plane3d 位于 y=0）
const PLAYER_HALF_HEIGHT: f32 = 0.5; // 角色立方体半高（站立时角色中心离地 0.5）
const PLAYER_SPEED: f32 = 5.0; // 水平移动速度
const JUMP_SPEED: f32 = 8.0; // 起跳初速度
const ARENA_HALF: f32 = 11.0; // 场地半尺寸（限制角色移动范围）
const WIN_SCORE: u32 = 10; // 集满多少金币获胜
pub const GAME_TIME: f32 = 45.0; // 倒计时（秒）
const COIN_COUNT: u32 = 10; // 场上金币数量

// ==================== 组件定义 ====================
// 游戏根标记：所有 Playing 页的实体都挂上，OnExit 时一键清理
#[derive(Component, Clone, Default)]
pub struct GameRoot;

// 玩家（方块角色）
#[derive(Component)]
pub struct Player;

// 速度组件（重力 / 跳跃共用）
#[derive(Component)]
pub struct Velocity(Vec3);

// 可收集的金币（PBR 金属材质，原地旋转）
#[derive(Component)]
pub struct Coin;

// HUD 文本标记
#[derive(Component, Clone, Default)]
pub struct ScoreText;
#[derive(Component, Clone, Default)]
pub struct TimerText;

// ==================== 资源定义 ====================
#[derive(Resource)]
pub struct Score(pub u32);

// 角色朝向（yaw）：WASD 相对朝向移动，Q/E 转身
#[derive(Resource)]
pub struct PlayerYaw(pub f32);

// 胜利 / 超时标记：GameOver 页据此显示不同文案
#[derive(Resource, Default)]
pub struct WinFlag(pub bool);

// 倒计时：Once 模式，归零且没集满金币 → 超时失败
#[derive(Resource)]
pub struct GameTimer(pub Timer);

// ==================== 进入游戏页 ====================
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut score: ResMut<Score>,
    mut timer: ResMut<GameTimer>,
    mut win_flag: ResMut<WinFlag>,
) {
    score.0 = 0;
    win_flag.0 = false;
    timer.0 = Timer::from_seconds(GAME_TIME, TimerMode::Once);
    info!(
        "[游戏] 开始：3D 收集金币（目标 {} 枚 / {:.0}s）",
        WIN_SCORE, GAME_TIME
    );

    // 3D 相机（初始位置，follow_camera 系统每帧会覆盖）
    commands.spawn((
        GameRoot,
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光：模拟太阳光，让物体有明暗立体感
    commands.spawn((
        GameRoot,
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面：深色大平面
    commands.spawn((
        GameRoot,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(30.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.25, 0.3),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 玩家：绿色立方体 + 速度组件（跳跃用）
    commands.spawn((
        GameRoot,
        Player,
        Velocity(Vec3::ZERO),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.5),
            ..default()
        })),
        Transform::from_xyz(0.0, PLAYER_HALF_HEIGHT, 0.0),
    ));

    // 场地四周的参照柱：既装饰，也帮助判断移动距离
    for corner in [
        Vec3::new(-ARENA_HALF, 1.0, -ARENA_HALF),
        Vec3::new(ARENA_HALF, 1.0, -ARENA_HALF),
        Vec3::new(-ARENA_HALF, 1.0, ARENA_HALF),
        Vec3::new(ARENA_HALF, 1.0, ARENA_HALF),
    ] {
        commands.spawn((
            GameRoot,
            Mesh3d(meshes.add(Cuboid::new(0.6, 2.0, 0.6))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.5, 0.7),
                ..default()
            })),
            Transform::from_translation(corner),
        ));
    }

    // 金币资产：金属金色小球（metallic=1 高光，roughness 低 → 镜面反射），全部金币共享句柄
    let coin_mesh = meshes.add(Sphere::new(0.4));
    let coin_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.3),
        metallic: 1.0,
        perceptual_roughness: 0.2,
        ..default()
    });

    // 生成金币：随机落在场地地面上
    for _ in 0..COIN_COUNT {
        let pos = random_ground_pos();
        commands.spawn((
            GameRoot,
            Coin,
            Mesh3d(coin_mesh.clone()),
            MeshMaterial3d(coin_material.clone()),
            Transform::from_xyz(pos.x, 0.4, pos.z),
        ));
    }

    // HUD（屏幕左上角）：得分 + 倒计时
    commands
        .spawn((
            GameRoot,
            Node {
                width: percent(100),
                padding: UiRect::all(px(12)),
                row_gap: px(4),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                Text::new("金币: 0/10"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
            ));
            parent.spawn((
                TimerText,
                Text::new("剩余时间: 45s"),
                TextColor(Color::srgb(0.9, 0.7, 0.4)),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
            ));
        });
}

// 随机地面坐标（避开场地边缘，防止金币刷到墙外）
fn random_ground_pos() -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(
        rng.random_range(-(ARENA_HALF - 1.0)..(ARENA_HALF - 1.0)),
        0.0,
        rng.random_range(-(ARENA_HALF - 1.0)..(ARENA_HALF - 1.0)),
    )
}

// ==================== 退出游戏页 ====================
pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ==================== 角色移动 ====================
pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut yaw: ResMut<PlayerYaw>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut tf, mut velocity) = player.into_inner();
    let dt = time.delta_secs();

    // 由 yaw 推导 forward / right（水平方向）
    let forward = Vec3::new(yaw.0.sin(), 0.0, -yaw.0.cos());
    let right = Vec3::new(yaw.0.cos(), 0.0, yaw.0.sin());

    let mut dir = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir -= right;
    }
    if dir != Vec3::ZERO {
        tf.translation += dir.normalize() * PLAYER_SPEED * dt;
    }

    // Q / E 转身
    if keys.pressed(KeyCode::KeyQ) {
        yaw.0 += 2.0 * dt;
    }
    if keys.pressed(KeyCode::KeyE) {
        yaw.0 -= 2.0 * dt;
    }
    // 角色面向 yaw 方向
    tf.rotation = Quat::from_rotation_y(yaw.0);

    // 空格起跳：仅当站在地面上才能跳。
    // 站立时角色中心 y = GROUND_Y + PLAYER_HALF_HEIGHT（0.5），
    // 所以用「接近站立高度」判断是否在地面（之前误用 0.01 导致永远跳不起来）。
    if keys.just_pressed(KeyCode::Space) && tf.translation.y <= GROUND_Y + PLAYER_HALF_HEIGHT + 0.01
    {
        velocity.0.y = JUMP_SPEED;
        info!("[跳跃] 起跳");
    }
}

// ==================== 物理：重力 + 地面 + 场地边界 ====================
pub fn apply_physics(time: Res<Time>, mut q: Query<(&mut Velocity, &mut Transform), With<Player>>) {
    let dt = time.delta_secs();
    for (mut velocity, mut tf) in &mut q {
        // 重力加速度作用于 y 方向速度，速度积分更新位置
        velocity.0.y += GRAVITY * dt;
        tf.translation += velocity.0 * dt;

        // 地面：落到地面则停住（角色中心高度 = 半高）
        if tf.translation.y < GROUND_Y + PLAYER_HALF_HEIGHT {
            tf.translation.y = GROUND_Y + PLAYER_HALF_HEIGHT;
            velocity.0.y = 0.0;
        }

        // 场地边界：水平方向 clamp，防止走出场地（边界 = 场地半宽 - 角色半宽）
        tf.translation.x = tf.translation.x.clamp(
            -ARENA_HALF + PLAYER_HALF_HEIGHT,
            ARENA_HALF - PLAYER_HALF_HEIGHT,
        );
        tf.translation.z = tf.translation.z.clamp(
            -ARENA_HALF + PLAYER_HALF_HEIGHT,
            ARENA_HALF - PLAYER_HALF_HEIGHT,
        );
    }
}

// ==================== 第三人称跟随相机 ====================
pub fn follow_camera(
    player: Single<&Transform, With<Player>>,
    yaw: Res<PlayerYaw>,
    // Without<Player>：相机写 Transform、玩家读 Transform，用过滤器保证不相交（Bevy B0001）
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let forward = Vec3::new(yaw.0.sin(), 0.0, -yaw.0.cos());
    let target = player.translation;
    // 相机始终位于角色「后方 + 上方」
    let distance = 8.0;
    let height = 4.5;
    camera.translation = target - forward * distance + Vec3::Y * height;
    camera.look_at(target, Vec3::Y);
}

// ==================== 金币旋转 ====================
pub fn rotate_coins(time: Res<Time>, mut q: Query<&mut Transform, With<Coin>>) {
    for mut tf in &mut q {
        tf.rotate_y(time.delta_secs() * 2.0);
    }
}

// ==================== 收集金币 ====================
pub fn collect_coins(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<&Transform, With<Player>>,
    coins: Query<(&Transform, Entity), With<Coin>>,
    mut score: ResMut<Score>,
    mut win_flag: ResMut<WinFlag>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player_pos = player.translation;
    for (tf, entity) in &coins {
        // 3D 距离检测：球心距离 < 半径之和（玩家半宽 0.5 + 金币半径 0.4）
        if player_pos.distance(tf.translation) < 1.0 {
            commands.entity(entity).despawn();
            score.0 += 1;
            // 收集音效
            commands.spawn((
                AudioPlayer::new(asset_server.load(BLIP_SOUND)),
                PlaybackSettings::DESPAWN,
            ));
            info!("[收集] 金币 +1，当前 {}/{}", score.0, WIN_SCORE);

            // 集满 → 胜利
            if score.0 >= WIN_SCORE {
                info!("[胜利] 集满 {} 枚金币！", WIN_SCORE);
                win_flag.0 = true;
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

// ==================== 倒计时 ====================
pub fn game_timer_system(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    score: Res<Score>,
    mut win_flag: ResMut<WinFlag>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() && score.0 < WIN_SCORE {
        info!("[超时] 时间到，仅收集 {} 枚", score.0);
        win_flag.0 = false;
        next_state.set(GameState::GameOver);
    }
}

// ==================== HUD 更新 ====================
pub fn update_score_display(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("金币: {}/{}", score.0, WIN_SCORE);
    }
}

pub fn update_timer_display(timer: Res<GameTimer>, mut query: Query<&mut Text, With<TimerText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("剩余时间: {:.0}s", timer.0.remaining_secs().max(0.0));
    }
}

// ESC 返回主菜单
pub fn esc_to_menu(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
