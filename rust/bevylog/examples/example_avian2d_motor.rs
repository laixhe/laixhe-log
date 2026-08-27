//! Bevy 0.19 入门示例：avian2d —— 马达（Motor）。
//!
//! 关节只约束运动，马达给关节"驱动"，让被约束的刚体自己动起来：
//! - AngularMotor 角马达：挂在 RevoluteJoint（铰链）上，驱动旋转 → 风车
//! - LinearMotor   线马达：挂在 PrismaticJoint（棱柱）上，驱动滑动 → 电梯
//!
//! 学习重点：
//! - MotorModel：马达模型（默认 SpringDamper 弹簧阻尼，另有 ForceBased / AccelerationBased）
//! - AngularMotor::new(model).with_target_velocity(rpm).with_max_torque(力矩上限)
//! - LinearMotor::new(model).with_target_velocity(v).with_max_force(力上限)
//! - 关节组件.with_motor(...) 把马达装到关节上
//! - 运行中控制马达：直接读写 joint.motor.enabled / target_velocity
//!
//! 操作：
//! - 观察风车恒速旋转、电梯上下往复（上面还载着个箱子）
//! - 按空格：切换所有马达的开关（看看停止/恢复）

use avian2d::prelude::*;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.1)))
        // 每 2 秒把电梯马达的目标速度反向，让它上下往复
        .insert_resource(ElevatorTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (flip_elevator, toggle_motors))
        .run()
}

// 电梯换向计时器
#[derive(Resource)]
struct ElevatorTimer(Timer);

// 标记电梯的平台，方便给它下面的马达反向
#[derive(Component)]
struct ElevatorPlatform;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // ============ 左：风车（RevoluteJoint + AngularMotor）============
    // 静态轴心 + 动态叶片，铰链钉在中心，角马达驱动叶片恒速旋转。
    let hub = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(-260.0, 80.0, 0.0),
        ))
        .id();
    // 叶片视觉（一根长条 + 中心圆点）
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(12.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.9, 0.8, 0.3))),
        Transform::from_xyz(-260.0, 80.0, 0.0),
    ));
    let blade = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::rectangle(14.0, 130.0),
            Mesh2d(meshes.add(Rectangle::new(14.0, 130.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.5, 0.3))),
            Transform::from_xyz(-260.0, 80.0, 0.0),
        ))
        .id();
    commands.spawn((
        RevoluteJoint::new(hub, blade)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(Vec2::ZERO)
            // 角马达：默认 SpringDamper 弹簧阻尼模型，用 target_velocity 做目标速度控制，
            // max_torque 限制最大输出力矩（2.5 rad/s 恒速旋转）
            .with_motor(
                AngularMotor::new(MotorModel::default())
                    .with_target_velocity(2.5)
                    .with_max_torque(2000.0),
            ),
    ));

    // ============ 右：电梯（PrismaticJoint + LinearMotor）============
    // 静态导轨 + 动态平台，棱柱关节只允许平台沿 Y 滑动，线马达驱动它上下。
    let rail = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(260.0, 0.0, 0.0),
        ))
        .id();
    let platform = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::rectangle(120.0, 14.0),
            Mesh2d(meshes.add(Rectangle::new(120.0, 14.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.4, 0.75, 0.9))),
            Transform::from_xyz(260.0, -80.0, 0.0),
            ElevatorPlatform,
        ))
        .id();
    commands.spawn((
        PrismaticJoint::new(rail, platform)
            .with_slider_axis(Vec2::Y)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(Vec2::ZERO)
            .with_limits(-160.0, 160.0) // 平台行程范围
            // 线马达：默认弹簧阻尼模型 + target_velocity 速度控制（90 px/s 恒速滑动）
            .with_motor(
                LinearMotor::new(MotorModel::default())
                    .with_target_velocity(90.0)
                    .with_max_force(3000.0),
            ),
    ));
    // 电梯平台上放一个箱子：平台上升时顶着它一起走（演示马达能"带动负载"）
    commands.spawn((
        RigidBody::Dynamic,
        Collider::rectangle(40.0, 40.0),
        Friction::new(0.9),
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.5, 0.4))),
        Transform::from_xyz(260.0, -55.0, 0.0),
    ));

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("左：风车(角马达)  右：电梯(线马达)  空格切换马达开关")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, 300.0, 0.0)
    });
}

// ---- 每 2 秒把电梯马达的目标速度反向：平台上下往复 ----
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

// ---- 按空格：切换所有马达开关（演示运行中控制马达）----
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
