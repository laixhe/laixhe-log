//! Bevy 0.19 入门示例：演示检测组件被移除（RemovedComponents）。
//! 之前学过 `Added<T>` / `Changed<T>` 检测「新增 / 修改」，本例补上「移除」检测。
//!
//! 学习重点：
//! - `RemovedComponents<T>` 是一个系统参数（类似 MessageReader），用 `.read()` 读取被移除组件的实体
//! - 触发时机：`commands.entity(e).remove::<T>()`（只移除组件）或 `despawn()`（销毁实体）都会触发
//! - 和 `Added`/`Changed` 互补：Added 新增、Changed 修改、RemovedComponents 移除
//! - 移除是延迟命令，命令应用到世界后，下一帧 `RemovedComponents` 才能读到（消息游标保证每条只读一次）
//!
//! 操作方式：
//! - R：移除一个 Item 组件（实体保留，只去掉组件）
//! - D：销毁一个实体（连组件一起消失）

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 标记组件：挂上它表示「这是一个物品」
#[derive(Component)]
struct Item;

// 给物品起个名字，方便日志区分
#[derive(Component)]
struct Named(&'static str);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先处理输入（移除），再检测移除
        .add_systems(
            Update,
            (remove_one, despawn_one, detect_removed, update_text).chain(),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 生成 3 个物品
    for name in ["物品A", "物品B", "物品C"] {
        commands.spawn((Item, Named(name)));
    }

    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, -200.0, 0.0)
    });
}

// R：只移除一个实体的 Item 组件（实体本身还在，只是不再带 Item）。
// 用 remove::<Item>() 而不是 despawn，专门演示「组件移除」也能被检测到。
fn remove_one(
    keys: Res<ButtonInput<KeyCode>>,
    items: Query<(Entity, &Named), With<Item>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        if let Some((entity, name)) = items.iter().next() {
            commands.entity(entity).remove::<Item>();
            info!("[移除] 已移除 {} 的 Item 组件", name.0);
        } else {
            info!("[移除] 没有 Item 组件可移除了");
        }
    }
}

// D：销毁一个实体（连 Item 组件一起消失，同样会触发 RemovedComponents）。
fn despawn_one(
    keys: Res<ButtonInput<KeyCode>>,
    items: Query<(Entity, &Named), With<Item>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyD) {
        if let Some((entity, name)) = items.iter().next() {
            commands.entity(entity).despawn();
            info!("[移除] 已销毁实体 {}", name.0);
        }
    }
}

// 检测 Item 组件被移除的实体。
// read() 返回本帧（自上次读取以来）所有被移除 Item 的实体。
fn detect_removed(mut removed: RemovedComponents<Item>) {
    for entity in removed.read() {
        info!("[检测] Item 组件已从实体 {entity:?} 上移除");
    }
}

// 更新提示文本：显示剩余 Item 数量。
fn update_text(items: Query<&Item>, mut text: Single<&mut Text2d>, mut last: Local<String>) {
    let new_text = format!(
        "R：移除组件  |  D：销毁实体  |  剩余 Item：{}",
        items.iter().count()
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
