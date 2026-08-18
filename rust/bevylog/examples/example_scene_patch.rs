//! Bevy 0.19 入门示例：演示场景补丁（Patching 字段覆盖）。
//!
//! 学习重点：
//! - 场景复用：定义一个 enemy() 场景，包含 Health 组件和文本
//! - 场景补丁：生成时用 Health { max: 200.0 } 只覆盖 max 字段
//! - 未提到的字段保持原值（current 仍为 100），无需重新指定全部字段
//! - Added<Health> 查询过滤器：只在组件刚添加时触发，用于打印生成结果
//!
//! 观察：普通敌人生命 100/100，精英敌人通过补丁变成 100/200。

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 生命组件：current 当前生命，max 最大生命
#[derive(Component, Default, Clone)]
struct Health {
    current: f32,
    max: f32,
}

// 敌人场景：默认生命 100/100
fn enemy(name: String, x: f32) -> impl Scene {
    bsn! {
        Health { current: 100.0, max: 100.0 }
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
        .add_systems(Update, log_health)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 普通敌人：使用默认生命 100/100
    commands.spawn_scene(bsn! { enemy("普通敌人".to_string(), -200.0) });

    // 精英敌人：复用 enemy 场景，只 patch max 字段为 200
    commands.spawn_scene(bsn! {
        enemy("精英敌人".to_string(), 200.0)
        Health { max: 200.0 }
    });
}

// 在 Health 刚添加时打印一次，观察补丁结果
fn log_health(q: Query<(&Text2d, &Health), Added<Health>>) {
    for (text, health) in &q {
        info!(
            "[场景补丁] {} 生命 {}/{}",
            text.0, health.current, health.max
        );
    }
}
