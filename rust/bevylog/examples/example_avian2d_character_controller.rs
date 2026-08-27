//! Bevy 0.19 入门示例：avian2d —— 角色控制器（MoveAndSlide）。
//!
//! avian 的角色控制器不是"自带一套组件"，而是一个 MoveAndSlide 系统参数：
//! 自己管理角色的速度（输入、重力、跳跃），每帧调用 move_and_slide 把
//! 一个碰撞体沿速度方向推进，碰到地面/墙壁时自动"沿表面滑行"，最后
//! 用返回的位置更新 Transform。这就是 Quake 式 collide-and-slide 算法。
//!
//! 学习重点：
//! - MoveAndSlide 系统参数：move_and_slide(碰撞体, 位置, 旋转, 速度, 帧时长, 配置, 过滤, 命中回调)
//! - 角色实体不需要 RigidBody：位置完全由我们手动更新（运动学角色）
//! - SpatialQueryFilter::from_excluded_entities：排除角色自己，避免自碰撞
//! - 命中回调里的 normal：法线朝上(y > 0.7) 即认为踩到地面，用于跳跃判定
//! - 速度管理：水平直接设定（输入），竖直自己累加重力与跳跃初速度
//! - out.projected_velocity：滑行后的剩余速度，存回下一帧继续用
//!
//! 操作：A/D 或 ←/→ 移动，空格跳跃。

use avian2d::prelude::*;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
const MOVE_SPEED: f32 = 260.0; // 水平移动速度（像素/秒）
const JUMP_SPEED: f32 = 430.0; // 跳跃初速度
const GRAVITY: f32 = 900.0; // 手动模拟的重力加速度

// 角色组件：velocity 是自己维护的速度，grounded 表示是否踩在地面上
#[derive(Component)]
struct Player {
    velocity: Vec2,
    grounded: bool,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.11)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, follow_camera))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // ---- 地面与平台：普通静态刚体 + 矩形碰撞体 ----
    spawn_platform(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(0.0, -200.0),
        Vec2::new(700.0, 40.0),
        Color::srgb(0.3, 0.35, 0.45),
    );
    spawn_platform(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(-250.0, 0.0),
        Vec2::new(200.0, 30.0),
        Color::srgb(0.45, 0.35, 0.5),
    );
    spawn_platform(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(250.0, 40.0),
        Vec2::new(200.0, 30.0),
        Color::srgb(0.35, 0.5, 0.45),
    );

    // ---- 角色：只有碰撞体 + 自定义速度，没有 RigidBody（运动学角色）----
    commands.spawn((
        Collider::rectangle(30.0, 50.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Player {
            velocity: Vec2::ZERO,
            grounded: false,
        },
        Mesh2d(meshes.add(Rectangle::new(30.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.85, 0.6))),
    ));

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("A/D 移动，空格跳跃")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, 300.0, 0.0)
    });
}

fn spawn_platform(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
    size: Vec2,
    color: Color,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}

// ---- 核心：每帧用 MoveAndSlide 推进角色 ----
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut players: Query<(Entity, &Collider, &mut Transform, &mut Player)>,
) {
    for (entity, collider, mut transform, mut player) in &mut players {
        // 1. 水平速度：直接由输入决定（有输入就满速，没有就停）
        let mut input_dir = 0.0;
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            input_dir -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            input_dir += 1.0;
        }
        player.velocity.x = input_dir * MOVE_SPEED;

        // 2. 竖直速度：累加重力（手动模拟）
        player.velocity.y -= GRAVITY * time.delta_secs();

        // 3. 跳跃：只有"上一帧踩着地面"才能起跳（不能在空中二段跳）
        let was_grounded = player.grounded;
        if keyboard.just_pressed(KeyCode::Space) && was_grounded {
            player.velocity.y = JUMP_SPEED;
        }

        // 4. move and slide：把角色沿速度方向推进，撞到东西就滑行
        let mut grounded = false;
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let out = move_and_slide.move_and_slide(
            collider,                                // 用哪个碰撞体来推进
            transform.translation.truncate(),        // 当前位置（2D 取 xy）
            transform.rotation.to_euler(EulerRot::XYZ).2, // 当前旋转角
            player.velocity,                         // 想移动的速度
            time.delta(),                            // 本帧时长
            &MoveAndSlideConfig::default(),
            &filter,                                 // 排除角色自己
            |hit| {
                // 命中回调：如果撞到的表面法线朝上（|normal.y| 接近 1），认为踩到了地面
                if hit.normal.y > 0.7 {
                    grounded = true;
                }
                MoveAndSlideHitResponse::Accept
            },
        );
        transform.translation = out.position.extend(0.0);
        player.velocity = out.projected_velocity; // 滑行后的剩余速度（保留给下一帧）
        player.grounded = grounded;
    }
}

// ---- 相机跟随角色 ----
fn follow_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    camera.translation = player.translation.truncate().extend(0.0);
}
