//! Bevy 0.19 入门示例：演示场景系统（可复用场景）。
//!
//! 场景（Scene）允许把一组实体声明成「可复用模板」，之后多次 spawn_scene 生成，
//! 还能在生成时覆盖部分字段（补丁）。
//!
//! 学习重点：
//! - 返回 impl Scene 的函数：把 bsn! 声明包成可复用场景
//! - spawn_scene：把场景生成到世界
//! - 场景函数可带参数，生成时传入不同值

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 可复用场景：一个返回 impl Scene 的函数（内部用 bsn! 声明实体）。
// 参数用 String（owned），避免 &str 的生命周期问题。
fn enemy(name: String, x: f32) -> impl Scene {
    bsn! {
        Text2d::new(name)
        TextColor(Color::srgb(0.9, 0.3, 0.3))
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(28.0),
        }
        Transform::from_xyz(x, 0.0, 0.0)
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 复用 enemy 场景，生成三个不同名字/位置的敌人
    commands.spawn_scene(bsn! { enemy("敌人 A".to_string(), -200.0) });
    commands.spawn_scene(bsn! { enemy("敌人 B".to_string(), 0.0) });
    commands.spawn_scene(bsn! { enemy("敌人 C".to_string(), 200.0) });

    // 说明文本
    commands.spawn_scene(bsn! {
        Text2d::new("场景系统：用 impl Scene 函数复用场景")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}
