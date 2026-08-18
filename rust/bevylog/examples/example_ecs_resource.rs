//! Bevy 0.19 入门示例：演示资源（Resource）系统。
//!
//! Resource 是「全局唯一」的数据（类似单例），任意系统可通过 Res / ResMut 读写。
//! 与组件（每个实体一份）不同，资源在整个世界里只有一份，适合存分数、配置、全局状态等。
//!
//! 学习重点：
//! - #[derive(Resource)] 自定义资源
//! - init_resource：用 Default / FromWorld 自动构造
//! - insert_resource：直接插入现成实例
//! - Res<T> 只读访问 / ResMut<T> 可变访问
//! - FromWorld：构造资源时可依赖其他资源（资源间依赖）

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 资源1：分数。derive Default 让 Score(0) 可用 init_resource 自动初始化。
#[derive(Resource, Default)]
struct Score(u32);

// 资源2：玩家名。手写 Default 实现自定义初始值。
#[derive(Resource)]
struct PlayerName(String);

impl Default for PlayerName {
    fn default() -> Self {
        Self("新手玩家".to_string())
    }
}

// 资源3：问候语。用 FromWorld 构造，构造时可读取其他资源（展示资源间依赖）。
// 注意：Greeting 依赖 PlayerName，所以 init_resource 顺序必须是 PlayerName 在前。
#[derive(Resource)]
struct Greeting(String);

impl FromWorld for Greeting {
    fn from_world(world: &mut World) -> Self {
        // world.resource::<T>() 读取已初始化的资源（若不存在会 panic）
        let name = world.resource::<PlayerName>();
        Self(format!("你好，{}！按空格加分", name.0))
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        // init_resource 用 FromWorld 构造；Score/PlayerName 实现了 Default，
        // Bevy 对 Default 类型提供了统一的 FromWorld 实现，所以也能 init_resource。
        // 顺序很重要：Greeting 依赖 PlayerName，必须先初始化 PlayerName。
        .init_resource::<Score>()
        .init_resource::<PlayerName>()
        .init_resource::<Greeting>()
        .add_systems(Startup, setup)
        .add_systems(Update, add_score)
        .run()
}

fn setup(mut commands: Commands, greeting: Res<Greeting>) {
    commands.spawn(Camera2d);

    // 从资源读取问候语并打印（演示 Res<T> 只读访问）
    info!("[资源] 问候语 = {}", greeting.0);

    commands.spawn_scene(bsn! {
        Text2d::new("按空格加分（观察资源变化）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(28.0),
        }
        Transform::from_xyz(0.0, 80.0, 0.0)
    });
}

// 按空格给分数 +1（演示 ResMut<T> 可变访问）
fn add_score(keyboard: Res<ButtonInput<KeyCode>>, mut score: ResMut<Score>) {
    if keyboard.just_pressed(KeyCode::Space) {
        score.0 += 1;
        info!("[资源] 当前分数 = {}", score.0);
    }
}
