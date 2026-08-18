//! Bevy 0.19 入门示例：演示把场景作为系统（scene.spawn()）。
//!
//! 学习重点：
//! - 用返回 impl SceneList 的函数描述整个场景
//! - scene.spawn() 把场景函数变成系统，直接挂到 Schedule 上
//! - 不再需要手写 setup(Commands) 逐个 spawn，场景声明式地一次生成
//!
//! 观察：整个画面（相机 + 三个敌人 + 标题）由 scene.spawn() 系统生成。

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

// 标题场景
fn title() -> impl Scene {
    bsn! {
        Text2d::new("场景作为系统：scene.spawn() 声明式生成")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -220.0, 0.0)
    }
}

// 整个场景就是一个 SceneList
fn scene() -> impl SceneList {
    bsn_list![
        Camera2d,
        enemy("敌人 A".to_string(), -200.0),
        enemy("敌人 B".to_string(), 0.0),
        enemy("敌人 C".to_string(), 200.0),
        title(),
    ]
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        // scene 是函数，.spawn() 把它变成系统（来自 SpawnListSystem trait）
        .add_systems(Startup, scene.spawn())
        .run()
}
