//! Bevy 0.19 入门示例：迷你射击游戏（多文件模块组织）。
//! WASD 移动玩家，空格发射子弹，击中红色敌人加分。
//! 演示完整的游戏循环：移动 → 射击 → 碰撞 → 计分。
//!
//! 学习重点：
//! - 多文件模块组织：mod 声明拆分 components / messages / resources / systems / score_ui
//! - Message 跨系统通信：BulletFired 消息从 shoot 传到 spawn_bullet
//! - Resource 全局状态：Score 分数资源，init_resource 初始化、ResMut 修改、Res 读取
//! - 碰撞检测：距离判定 + break 防止重复加分
//! - UI HUD：Node + Text 做分数显示（不随相机移动）

mod components;
mod messages;
mod resources;
mod score_ui;
mod systems;

use bevy::prelude::*;
use messages::BulletFired;
use resources::Score;
use score_ui::{setup_score_display, update_score_display};
use systems::{
    check_collision, cleanup_bullets, move_bullet, move_player, setup, shoot, spawn_bullet,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "简易射击".into(),
                ..default()
            }),
            ..default()
        }))
        // 深色背景，让彩色玩家/敌人/子弹更醒目（与其他示例风格一致）
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.12)))
        // 初始化分数资源：Default 自动给 total 赋 0
        .init_resource::<Score>()
        // 注册 Message：BulletFired 用于 shoot → spawn_bullet 的跨系统通信。
        // Message 是 Bevy 0.19 新特性（类似 Event 但更轻量），详见 messages.rs。
        .add_message::<BulletFired>()
        .add_systems(Startup, (setup, setup_score_display))
        // Update 系统的数据流（按列出顺序执行，部分系统可能延迟一帧，60fps 下不可察觉）：
        //   move_player → shoot → spawn_bullet → move_bullet → check_collision → cleanup_bullets → update_score_display
        //   玩家移动     发射消息  生成子弹       子弹移动     击中加分          清理越界子弹     刷新分数
        // 注意：shoot 发送的 BulletFired 消息，spawn_bullet 可能下一帧才读到（Message 缓冲机制），
        //       子弹延迟一帧生成（约 16ms），玩家无法察觉。如需严格同帧生效可加 .chain()。
        .add_systems(
            Update,
            (
                move_player,
                shoot,
                spawn_bullet,
                move_bullet,
                check_collision,
                cleanup_bullets,
                update_score_display,
            ),
        )
        .run()
}
