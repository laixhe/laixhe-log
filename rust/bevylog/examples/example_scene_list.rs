//! Bevy 0.19 入门示例：演示场景列表（bsn_list! 一次生成多个根场景）。
//!
//! 学习重点：
//! - bsn_list! 把一个或多个 Scene 组合成一个 SceneList
//! - spawn_scene_list 一次生成多个「根实体」场景
//! - 与 spawn_scene 逐个生成相比，列表更紧凑、可整体复用
//!
//! 观察：一次生成三个敌人文本 + 标题。

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 敌人场景
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

// 场景列表：相机 + 三个敌人 + 标题，作为一个整体
fn scene_list() -> impl SceneList {
    bsn_list![
        Camera2d,
        enemy("敌人 A".to_string(), -200.0),
        enemy("敌人 B".to_string(), 0.0),
        enemy("敌人 C".to_string(), 200.0),
        (
            Text2d::new("场景列表：一次生成多个根场景")
            TextColor(Color::WHITE)
            TextFont {
                font: FontSourceTemplate::Handle(FONT_PATH),
                font_size: FontSize::Px(24.0),
            }
            Transform::from_xyz(0.0, -220.0, 0.0)
        ),
    ]
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_scene_list(scene_list());
}
