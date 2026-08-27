//! Bevy 0.19 入门示例：演示 bevy_tween —— 补间动画库。
//!
//! 补间（Tween）：声明「从 A 值到 B 值」，由引擎按时间自动插值过渡，
//! 不用自己在系统里手动 lerp 每帧更新。
//!
//! 核心概念：
//! - tween(时长, EaseKind, 目标) ：一段补间动画
//! - EaseKind 缓动函数：Linear 匀速 / QuadraticOut 先快后慢 / BackOut 带回弹 ...
//! - 组合器：sequence(...) 顺序播放、parallel(...) 同时播放
//! - 目标：实体 .into_target() 指定动画作用于谁，.with(插值器) 指定动哪个属性
//! - Transform 快捷方式：transform_state(初始变换).translation_to(...) / scale_to(...)
//! - 循环：.repeat(Repeat::infinitely()) + .repeat_style(RepeatStyle::PingPong)
//! - 事件：TimeRunnerEnded 整段动画结束时触发
//!
//! 操作：观察三个精灵——① 往返移动（sequence 顺序）② 缩放+变色（parallel 并行）
//! ③ 上下弹跳（无限循环 PingPong）。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded,
    combinator::{parallel, sequence, tween},
    interpolate::sprite_color,
    prelude::*,
};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        // 注册 tween 插件（补间系统、缓动注册、TimeRunner 等）
        .add_plugins(DefaultTweenPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, on_tween_ended)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // ---- 精灵 1：sequence 顺序播放（先右移，再放大 + 变色）----
    let sprite1 = commands
        .spawn((
            Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(60.0)),
            Transform::from_xyz(-300.0, 150.0, 0.0),
        ))
        .id();
    let target1 = sprite1.into_target();
    // 基于初始 Transform 建立状态：translation_to / scale_to 会在上一段动画的终点上继续
    let mut state1 = target1.transform_state(Transform::from_xyz(-300.0, 150.0, 0.0));
    commands.entity(sprite1).animation().insert(sequence((
        // 第 1 段：1 秒右移到中心（QuadraticOut 先快后慢）
        tween(
            Duration::from_secs(1),
            EaseKind::QuadraticOut,
            state1.translation_to(Vec3::new(0.0, 150.0, 0.0)),
        ),
        // 第 2 段：parallel 同时放大 + 变色（BackOut 带轻微回弹）
        parallel((
            tween(
                Duration::from_secs(1),
                EaseKind::BackOut,
                state1.scale_to(Vec3::splat(2.0)),
            ),
            tween(
                Duration::from_secs(1),
                EaseKind::Linear,
                target1.with(sprite_color(
                    Color::srgb(0.2, 0.6, 1.0),
                    Color::srgb(1.0, 0.4, 0.2),
                )),
            ),
        )),
    )));

    // ---- 精灵 2：scale_to + translation_to 同时进行（parallel）----
    let sprite2 = commands
        .spawn((
            Sprite::from_color(Color::srgb(0.3, 0.9, 0.4), Vec2::splat(50.0)),
            Transform::from_xyz(300.0, -80.0, 0.0),
        ))
        .id();
    let mut state2 = sprite2
        .into_target()
        .transform_state(Transform::from_xyz(300.0, -80.0, 0.0));
    commands.entity(sprite2).animation().insert(parallel((
        // 1.5 秒同时：向右移 + 缩小到一半
        tween(
            Duration::from_millis(1500),
            EaseKind::QuadraticInOut,
            state2.translation_to(Vec3::new(500.0, -80.0, 0.0)),
        ),
        tween(
            Duration::from_millis(1500),
            EaseKind::QuadraticInOut,
            state2.scale_to(Vec3::splat(0.5)),
        ),
    )));

    // ---- 精灵 3：无限循环 PingPong 上下弹跳 ----
    let sprite3 = commands
        .spawn((
            Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(55.0)),
            Transform::from_xyz(-300.0, -150.0, 0.0),
        ))
        .id();
    let mut state3 = sprite3
        .into_target()
        .transform_state(Transform::from_xyz(-300.0, -150.0, 0.0));
    commands
        .entity(sprite3)
        .animation()
        // 循环播放：无限重复 + 往返（PingPong 播放完自动倒放）
        .repeat(Repeat::infinitely())
        .repeat_style(RepeatStyle::PingPong)
        .insert(tween(
            Duration::from_secs(1),
            EaseKind::SineInOut,
            state3.translation_to(Vec3::new(-300.0, 100.0, 0.0)),
        ));

    // 底部提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("① 顺序：右移→放大变色 | ② 并行：移动+缩放 | ③ 循环弹跳")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, -280.0, 0.0)
    });
}

// TimeRunnerEnded：动画结束时触发一次。
// 注意：循环动画每次循环结束也会触发该事件，要用 is_completed() 判断是否真正结束。
fn on_tween_ended(mut ended: MessageReader<TimeRunnerEnded>) {
    for t in ended.read() {
        if t.is_completed() {
            info!("[补间] 动画结束，实体 = {:?}", t.entity);
        }
    }
}
