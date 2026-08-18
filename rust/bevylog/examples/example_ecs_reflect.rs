//! Bevy 0.19 入门示例：演示反射（Reflect）。
//!
//! 反射让代码能在「运行时」动态读取/修改一个类型的字段，而不需要在编译期知道它的具体类型。
//! Bevy 的编辑器、场景序列化、Inspector 等都建立在反射之上。
//!
//! 学习重点：
//! - #[derive(Reflect)] 让类型可被反射
//! - App::register_type 把类型注册到反射注册表
//! - reflect_ref() 拿到结构化视图，按字段名读取值
//! - downcast_ref::<T>() 把反射值还原成具体类型

use std::any::TypeId;

use bevy::reflect::ReflectRef;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 可反射的结构体：derive Reflect 后，字段能在运行时被按名字访问
#[derive(Reflect)]
struct PlayerStats {
    name: String,
    level: u32,
    hp: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        // 把类型注册到反射注册表，之后可通过 TypeRegistry 查找到它
        .register_type::<PlayerStats>()
        .add_systems(Startup, setup)
        .add_systems(Update, inspect)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn_scene(bsn! {
        Text2d::new("反射示例：观察终端日志读取字段")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(28.0),
        }
        Transform::from_xyz(0.0, 80.0, 0.0)
    });
}

// 每秒用反射读取一次字段，展示「无需知道具体类型也能访问字段」
fn inspect(type_registry: Res<AppTypeRegistry>, time: Res<Time>, mut last: Local<f32>) {
    if time.elapsed_secs() - *last > 1.0 {
        *last = time.elapsed_secs();

        // 1. 从反射注册表确认类型已注册，并打印类型路径
        let registry = type_registry.read();
        if let Some(registration) = registry.get(TypeId::of::<PlayerStats>()) {
            info!(
                "[反射] 已注册类型 = {}",
                registration.type_info().type_path()
            );
        }

        // 2. 构造一个实例，用反射按字段名读取值
        let stats = PlayerStats {
            name: "玩家".to_string(),
            level: 3,
            hp: 80.0,
        };
        if let ReflectRef::Struct(s) = stats.reflect_ref() {
            // field(name) 返回 Option<&dyn PartialReflect>，再用 try_downcast_ref 还原成具体类型
            if let Some(name) = s.field("name").and_then(|v| v.try_downcast_ref::<String>()) {
                info!("[反射] name = {}", name);
            }
            if let Some(level) = s.field("level").and_then(|v| v.try_downcast_ref::<u32>()) {
                info!("[反射] level = {}", level);
            }
            if let Some(hp) = s.field("hp").and_then(|v| v.try_downcast_ref::<f32>()) {
                info!("[反射] hp = {}", hp);
            }
        }
    }
}
