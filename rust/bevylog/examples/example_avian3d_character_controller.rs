//! Bevy 0.19 入门示例：avian3d —— 角色控制器（MoveAndSlide）。
//!
//! 与 2D 版（example_avian2d_character_controller）同一套 MoveAndSlide API，
//! 只是向量变成 Vec3：水平速度在 XZ 平面，重力沿 Y 轴。
//! 角色实体同样不需要 RigidBody，位置由 move_and_slide 手动更新。
//!
//! 学习重点：
//! - MoveAndSlide::move_and_slide 在 3D 下的用法（旋转直接传 Quat）
//! - 命中回调：normal.y > 0.7 判断"踩在地面上"
//! - 第三人称跟随相机：相机 = 角色位置 + 偏移，再 look_at 角色
//! - 用碰撞体做形状推进：角色用胶囊体（Collider::capsule）更贴近人形
//!
//! 操作：W/A/S/D 在平面移动，空格跳跃。

use avian3d::prelude::*;
use bevy::{prelude::*, text::FontSource};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
const MOVE_SPEED: f32 = 6.0;
const JUMP_SPEED: f32 = 9.0;
const GRAVITY: f32 = 25.0;

#[derive(Component)]
struct Player {
    velocity: Vec3,
    grounded: bool,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.09)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, follow_camera))
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机由 follow_camera 每帧跟随，这里给个初始位置即可
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 光照
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(24.0, 1.0, 24.0),
        Friction::new(0.8),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.14, 0.15, 0.18))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 几个障碍方块（演示滑行）
    for (x, z) in [(-3.0, 0.0), (3.0, -1.0), (0.0, 3.0)] {
        commands.spawn((
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.5, 0.35))),
            Transform::from_xyz(x, 0.5, z),
        ));
    }

    // 角色：胶囊碰撞体 + 自定义速度，没有 RigidBody
    commands.spawn((
        Collider::capsule(0.4, 1.0),
        Transform::from_xyz(0.0, 1.5, 0.0),
        Player {
            velocity: Vec3::ZERO,
            grounded: false,
        },
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.85, 0.6))),
    ));

    // 提示文字（UI 节点，由 3D 相机直接渲染）
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        Text::new("W/A/S/D 移动，空格跳跃（碰到方块会自动滑行）"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
    ));
}

// ---- 核心：MoveAndSlide 推进角色 ----
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut players: Query<(Entity, &Collider, &mut Transform, &mut Player)>,
) {
    for (entity, collider, mut transform, mut player) in &mut players {
        // 1. 水平输入：WASD 控制 XZ 平面的移动方向
        let mut dir = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            dir.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            dir.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            dir.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            dir.x += 1.0;
        }
        let input = if dir != Vec3::ZERO {
            dir.normalize()
        } else {
            Vec3::ZERO
        };
        player.velocity.x = input.x * MOVE_SPEED;
        player.velocity.z = input.z * MOVE_SPEED;

        // 2. 重力（竖直方向）
        player.velocity.y -= GRAVITY * time.delta_secs();

        // 3. 跳跃（用上一帧的着地状态判定）
        let was_grounded = player.grounded;
        if keyboard.just_pressed(KeyCode::Space) && was_grounded {
            player.velocity.y = JUMP_SPEED;
        }

        // 4. move and slide
        let mut grounded = false;
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let out = move_and_slide.move_and_slide(
            collider,
            transform.translation,
            transform.rotation,
            player.velocity,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &filter,
            |hit| {
                if hit.normal.y > 0.7 {
                    grounded = true;
                }
                MoveAndSlideHitResponse::Accept
            },
        );
        transform.translation = out.position;
        player.velocity = out.projected_velocity;
        player.grounded = grounded;
    }
}

// ---- 第三人称跟随相机：跟在角色后上方，看向角色 ----
fn follow_camera(
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    let desired = player.translation + Vec3::new(0.0, 5.0, 9.0);
    camera.translation = desired;
    camera.look_at(player.translation, Vec3::Y);
}
