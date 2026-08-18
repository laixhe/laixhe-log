//! Bevy 0.19 入门示例：游戏存档（序列化 / 反序列化）。
//! 用 serde + ron 把游戏状态序列化成文本存档，再反序列化读档。
//!
//! 学习重点：
//! - `#[derive(Serialize, Deserialize)]`：让结构体能被序列化 / 反序列化
//! - `ron::to_string`：序列化成 RON 文本（RON 是 Bevy 场景文件用的格式）
//! - `ron::from_str`：从 RON 文本反序列化回结构体（读档）
//! - 存档本质：把内存里的数据变成可保存的文本，读档时再还原
//! - 和 example_ecs_reflect（反射）区别：反射是运行时查看/操作数据，序列化是持久化数据
//!
//! 操作：S 保存，L 读档。分数每秒 +1，保存后继续涨，读档回到保存时的分数。

use bevy::{prelude::*, text::FontSourceTemplate};
use serde::{Deserialize, Serialize};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 游戏状态（存档数据），同时是 Resource 和可序列化
#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
struct SaveData {
    player_name: String,
    score: u32,
    level: u32,
}

// 存档槽：保存序列化后的 RON 文本
#[derive(Resource, Default)]
struct SaveSlot(String);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .insert_resource(SaveData {
            player_name: "小明".to_string(),
            score: 0,
            level: 1,
        })
        .init_resource::<SaveSlot>()
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先涨分，再处理存档，最后更新文本
        .add_systems(
            Update,
            (increase_score, handle_save_load, update_text).chain(),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -200.0, 0.0)
    });
}

// 每秒分数 +1，模拟游戏进度。
fn increase_score(time: Res<Time>, mut data: ResMut<SaveData>, mut last: Local<f32>) {
    if time.elapsed_secs() - *last >= 1.0 {
        *last = time.elapsed_secs();
        data.score += 1;
    }
}

// 处理存档 / 读档。
fn handle_save_load(
    keys: Res<ButtonInput<KeyCode>>,
    mut data: ResMut<SaveData>,
    mut slot: ResMut<SaveSlot>,
) {
    // S：序列化保存
    if keys.just_pressed(KeyCode::KeyS) {
        match ron::to_string(&*data) {
            Ok(s) => {
                slot.0 = s.clone();
                info!("[存档] 已保存：{s}");
            }
            Err(e) => info!("[存档] 保存失败：{e}"),
        }
    }

    // L：反序列化读档
    if keys.just_pressed(KeyCode::KeyL) {
        if slot.0.is_empty() {
            info!("[存档] 还没有存档，先按 S 保存");
            return;
        }
        match ron::from_str::<SaveData>(&slot.0) {
            Ok(loaded) => {
                info!("[存档] 已读档：{loaded:?}");
                *data = loaded;
            }
            Err(e) => info!("[存档] 读档失败：{e}"),
        }
    }
}

// 更新提示文本。
fn update_text(
    data: Res<SaveData>,
    slot: Res<SaveSlot>,
    mut text: Single<&mut Text2d>,
    mut last: Local<String>,
) {
    let has_save = if slot.0.is_empty() { "无" } else { "有" };
    let new_text = format!(
        "S：保存  |  L：读档  |  存档：{has_save}  |  {}：{} 分（等级 {}）",
        data.player_name, data.score, data.level
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
