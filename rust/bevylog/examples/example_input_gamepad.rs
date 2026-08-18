//! Bevy 0.19 入门示例：演示游戏手柄输入（Gamepad）。
//! 用左摇杆移动角色，按下按钮触发动作（终端打印日志）。
//!
//! 学习重点：
//! - ButtonInput<GamepadButton>：手柄按钮（类似键盘的 ButtonInput<KeyCode>）
//! - Axis<GamepadAxis>：手柄摇杆/扳机（模拟量，返回 -1.0 ~ 1.0）
//! - GamepadButton 枚举：South(A)/East(B)/West(X)/North(Y)/DPad/肩键等
//! - GamepadAxis 枚举：LeftStickX/LeftStickY（左摇杆）、RightStickX/RightStickY（右摇杆）
//! - 未连接手柄时按钮不触发、摇杆返回 None，代码能安全运行

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径（bsn! 里用 FontSourceTemplate 自动加载）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 玩家标记
#[derive(Component)]
struct Player;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, gamepad_input)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 玩家小球
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::default(),
    ));

    // 提示文本（世界坐标）
    commands.spawn_scene(bsn! {
        Text2d::new("连接手柄后：左摇杆移动 | A 键跳跃日志")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -200.0, 0.0)
    });
}

// 手柄输入：左摇杆移动，按钮触发动作
fn gamepad_input(
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    // 左摇杆移动：get 返回 Option<f32>，未连接手柄时为 None
    let x = gamepad_axes.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
    let y = gamepad_axes.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
    let direction = Vec2::new(x, y);

    // 摇杆有轻微死区，避免漂移
    if direction.length() > 0.1 {
        player.translation += (direction.normalize() * 300.0 * time.delta_secs()).extend(0.0);
    }

    // 按钮：A（South）和 B（East）
    if gamepad_buttons.just_pressed(GamepadButton::South) {
        info!("[手柄] 按下了 A（South）");
    }
    if gamepad_buttons.just_pressed(GamepadButton::East) {
        info!("[手柄] 按下了 B（East）");
    }
    if gamepad_buttons.just_pressed(GamepadButton::Start) {
        info!("[手柄] 按下了 Start（菜单键）");
    }
}
