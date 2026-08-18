//! Bevy 0.19 入门示例：时间控制（暂停 / 时间缩放）。
//!
//! Bevy 有三种时间：
//! - Time<Virtual>（默认，Res<Time>）：受暂停和缩放影响，游戏逻辑用它
//! - Time<Fixed>：固定步长（见 example_time_fixed_timestep）
//! - Time<Real>：真实时间，不受暂停/缩放影响（用于 UI、动画等）
//!
//! 学习重点：
//! - Time<Virtual>：pause() / unpause() / is_paused() / set_relative_speed()
//! - 暂停后 delta_secs 变 0，基于 dt 的移动自然停止
//! - set_relative_speed 缩放时间流逝速度（慢动作 / 快进）

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Moving;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_circle, handle_input))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 一个会水平移动的圆
    commands.spawn((
        Moving,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::from_xyz(-300.0, 0.0, 0.0),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("空格：暂停/继续  |  + / - ：加速/减速")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 移动圆：用虚拟时间（Res<Time> = Time<Virtual>）。
// 暂停时 delta_secs 为 0，圆自然停下；缩放时移动速度随之改变。
fn move_circle(time: Res<Time>, mut query: Query<&mut Transform, With<Moving>>) {
    for mut transform in &mut query {
        transform.translation.x += 200.0 * time.delta_secs();
        // 越界回绕到左侧
        if transform.translation.x > 400.0 {
            transform.translation.x = -400.0;
        }
    }
}

// 输入：空格暂停/继续，+ / - 调整时间缩放
fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
            info!("[时间] 继续");
        } else {
            time.pause();
            info!("[时间] 暂停");
        }
    }

    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        let speed = (time.relative_speed() + 0.5).min(3.0);
        time.set_relative_speed(speed);
        info!("[时间] 速度 = {:.1}x", speed);
    }

    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        let speed = (time.relative_speed() - 0.5).max(0.5);
        time.set_relative_speed(speed);
        info!("[时间] 速度 = {:.1}x", speed);
    }
}
