//! Bevy 0.19 入门示例：演示秒表（Stopwatch）。
//! Stopwatch 从 0 向上累计计时（记录「经过了多少时间」），和 Timer 的「从设定值向下倒数」正好相反。
//!
//! 学习重点：
//! - `Stopwatch` 需要手动 `tick(time.delta())` 才会前进，Bevy 不会自动给它计时
//! - `elapsed()` / `elapsed_secs()` 读取已累计的时间
//! - `pause()` / `unpause()` / `toggle()` / `reset()` 控制秒表状态
//! - `is_paused()` 判断当前是否暂停
//! - 对比：`Timer` 是「倒数 + finished」，`Stopwatch` 是「正数 + 无结束概念」
//!
//! 操作方式：
//! - 空格：暂停 / 继续
//! - R：重置为 0

use bevy::time::Stopwatch;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// Stopwatch 本身没有实现 Resource，所以包一层结构体作为资源
#[derive(Resource)]
struct StopwatchRes(Stopwatch);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        // 初始即运行（unpaused），从 0 开始累计
        .insert_resource(StopwatchRes(Stopwatch::new()))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先推进秒表，再更新文本（显示的时间是最新的）
        .add_systems(Update, (tick_stopwatch, handle_input, update_text).chain())
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

// 每帧推进秒表：tick 传入本帧经过的时长（虚拟时间，受暂停/缩放影响）。
// 注意：Stopwatch 暂停时 tick 不会累计（内部直接忽略）。
fn tick_stopwatch(time: Res<Time>, mut sw: ResMut<StopwatchRes>) {
    sw.0.tick(time.delta());
}

// 输入：空格暂停/继续，R 重置。
fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut sw: ResMut<StopwatchRes>) {
    if keys.just_pressed(KeyCode::Space) {
        // Stopwatch 没有 toggle 方法，用 is_paused 判断后手动切换
        if sw.0.is_paused() {
            sw.0.unpause();
            info!("[秒表] 继续");
        } else {
            sw.0.pause();
            info!("[秒表] 暂停");
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        sw.0.reset();
        info!("[秒表] 已重置");
    }
}

// 更新提示文本：显示累计秒数和暂停状态。
fn update_text(sw: Res<StopwatchRes>, mut text: Single<&mut Text2d>, mut last: Local<String>) {
    let state = if sw.0.is_paused() {
        "暂停"
    } else {
        "运行中"
    };
    let new_text = format!(
        "空格：暂停/继续  |  R：重置  |  状态：{state}  |  经过：{:.2} 秒",
        sw.0.elapsed_secs()
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
