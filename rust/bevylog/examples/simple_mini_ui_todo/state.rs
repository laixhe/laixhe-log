use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ==================== 应用数据（存档内容） ====================
/// 待办清单 + 设置项，同时是 Resource（运行态）和可序列化（存档）。
/// 序列化用的 serde derive 来自依赖里的 serde crate。
#[derive(Resource, Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppData {
    pub todos: Vec<String>, // 待办事项列表
    pub sound_on: bool,     // 是否启用音效（Checkbox）
    pub volume: f32,        // 音量 0~100（Slider）
    pub theme: usize,       // 主题色索引（RadioGroup，0/1/2）
}

// 存档槽：保存序列化后的 RON 文本（与 example_serialization 同思路，内存存档）
#[derive(Resource, Default)]
pub struct SaveSlot(pub String);

// 列表脏标记：添加 / 删除 / 读档后置 true，重建系统据此重建列表 UI
#[derive(Resource, Default)]
pub struct ListDirty(pub bool);

// 当前选中的待办索引（ListBox 选择结果）
#[derive(Resource, Default)]
pub struct SelectedIndex(pub Option<usize>);

// 主题色：0 蓝色调 / 1 紫色调 / 2 青色调
pub fn theme_color(theme: usize) -> Color {
    match theme % 3 {
        0 => Color::srgb(0.07, 0.09, 0.16),
        1 => Color::srgb(0.14, 0.07, 0.16),
        _ => Color::srgb(0.05, 0.13, 0.16),
    }
}
