//! Bevy 0.19 入门示例：avian3d —— 马达（Motor）。
//!
//! 与 2D 版（example_avian2d_motor）相同的马达概念，只是换到 3D：
//! - 风车：RevoluteJoint（铰链，hinge_axis 取 Y 竖直轴）+ AngularMotor 驱动旋转
//! - 电梯：PrismaticJoint（棱柱）+ LinearMotor 驱动上下滑动
//!
//! 学习重点：
//! - RevoluteJoint::with_hinge_axis(Vec3::Y)：指定铰链转轴
//! - AngularMotor / LinearMotor + MotorModel 的用法（默认 SpringDamper 弹簧阻尼模型）
//! - 运行中通过 &mut RevoluteJoint / &mut PrismaticJoint 控制马达
//!
//! 操作：
//! - 观察风车恒速旋转、电梯带着箱子上下来回
//! - 按空格：切换所有马达开关

use avian3d::prelude::*;
use bevy::{prelude::*, text::FontSource};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.09)))
        .insert_resource(ElevatorTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (flip_elevator, toggle_motors))
        .run()
}

// 电梯换向计时器
#[derive(Resource)]
struct ElevatorTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机 + 光照
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 12.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(24.0, 1.0, 24.0),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.14, 0.15, 0.18))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // ============ 左：风车（RevoluteJoint + AngularMotor）============
    let hub = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(-5.0, 3.0, 0.0),
        ))
        .id();
    let blade = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.4, 2.6, 0.4),
            Mesh3d(meshes.add(Cuboid::new(0.4, 2.6, 0.4))),
            MeshMaterial3d(materials.add(Color::srgb(0.9, 0.5, 0.3))),
            Transform::from_xyz(-5.0, 3.0, 0.0),
        ))
        .id();
    commands.spawn((
        RevoluteJoint::new(hub, blade)
            .with_hinge_axis(Vec3::Y) // 绕竖直轴转（风车叶片水平旋转）
            .with_local_anchor1(Vec3::ZERO)
            .with_local_anchor2(Vec3::ZERO)
            // 角马达：默认 SpringDamper 弹簧阻尼模型 + target_velocity 速度控制
            .with_motor(
                AngularMotor::new(MotorModel::default())
                    .with_target_velocity(2.0)
                    .with_max_torque(2000.0),
            ),
    ));

    // ============ 右：电梯（PrismaticJoint + LinearMotor）============
    let rail = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(5.0, 0.0, 0.0),
        ))
        .id();
    let platform = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(2.4, 0.3, 2.4),
            Mesh3d(meshes.add(Cuboid::new(2.4, 0.3, 2.4))),
            MeshMaterial3d(materials.add(Color::srgb(0.4, 0.75, 0.9))),
            Transform::from_xyz(5.0, 1.5, 0.0),
        ))
        .id();
    commands.spawn((
        PrismaticJoint::new(rail, platform)
            .with_slider_axis(Vec3::Y)
            .with_local_anchor1(Vec3::ZERO)
            .with_local_anchor2(Vec3::ZERO)
            .with_limits(0.0, 4.0) // 平台行程 0 ~ 4
            // 线马达：默认弹簧阻尼模型 + target_velocity 速度控制
            .with_motor(
                LinearMotor::new(MotorModel::default())
                    .with_target_velocity(1.5)
                    .with_max_force(2000.0),
            ),
    ));
    // 电梯平台上放一个箱子：平台上升时顶着它一起走
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.8, 0.8, 0.8),
        Friction::new(0.9),
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.8, 0.8))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.5, 0.4))),
        Transform::from_xyz(5.0, 1.9, 0.0),
    ));

    // 提示文字（UI 节点，由 3D 相机直接渲染）
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        Text::new("左：风车(角马达)  右：电梯(线马达)  空格切换马达开关"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
    ));
}

// ---- 每 2 秒把电梯马达的目标速度反向 ----
fn flip_elevator(
    time: Res<Time>,
    mut timer: ResMut<ElevatorTimer>,
    mut joints: Query<&mut PrismaticJoint>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    for mut joint in &mut joints {
        joint.motor.target_velocity = -joint.motor.target_velocity;
    }
}

// ---- 按空格：切换所有马达开关 ----
fn toggle_motors(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut revolute_joints: Query<&mut RevoluteJoint>,
    mut prismatic_joints: Query<&mut PrismaticJoint>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for mut joint in &mut revolute_joints {
        joint.motor.enabled = !joint.motor.enabled;
    }
    for mut joint in &mut prismatic_joints {
        joint.motor.enabled = !joint.motor.enabled;
    }
}
