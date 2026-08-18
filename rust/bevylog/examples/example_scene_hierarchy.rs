//! Bevy 0.19 入门示例：演示场景层级（bsn! 中的父子关系）。
//!
//! 学习重点：
//! - bsn! 里用 Children [ ... ] 建立父子关系（ChildOf / Children）
//! - 子实体的 Transform 是「相对父实体」的局部坐标
//! - 父实体移动时，子实体会跟着一起动（GlobalTransform 由层级自动计算）
//! - 用标记组件 + 系统让父节点上下移动，观察子节点跟随
//!
//! 观察：父节点上下浮动，两个子节点始终跟随。

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 标记组件：用来在系统里移动父节点
#[derive(Component, Default, Clone)]
struct Movable;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_parent)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一个父文本 + 两个子文本，子文本挂在父文本下面
    commands.spawn_scene(bsn! {
        Movable
        Text2d::new("父节点")
        TextColor(Color::srgb(0.9, 0.7, 0.2))
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(32.0),
        }
        Transform::from_xyz(0.0, 100.0, 0.0)
        Children [
            (
                Text2d::new("子节点 A")
                TextColor(Color::srgb(0.4, 0.8, 0.4))
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(24.0),
                }
                Transform::from_xyz(0.0, -70.0, 0.0)
            ),
            (
                Text2d::new("子节点 B")
                TextColor(Color::srgb(0.4, 0.6, 0.9))
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(24.0),
                }
                Transform::from_xyz(0.0, -140.0, 0.0)
            ),
        ]
    });
}

// 让父节点上下浮动，子节点会自动跟随
fn move_parent(time: Res<Time>, mut q: Query<&mut Transform, With<Movable>>) {
    for mut tf in &mut q {
        tf.translation.y = 100.0 + (time.elapsed_secs() * 2.0).sin() * 40.0;
    }
}
