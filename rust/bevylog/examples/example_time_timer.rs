//! Bevy 0.19 入门示例：演示定时器（Timer）与时间资源（Time）的区别。
//! 一个圆按固定间隔闪烁（Timer 组件），顶部显示运行总时长（Time 资源），底部显示倒计时（Timer 资源）。
//!
//! 学习重点：
//! - Time 资源：只读的「全局时间」，delta_secs / elapsed_secs，Bevy 每帧自动更新，无需手动推进
//! - Timer：需要手动调用 tick(delta) 推进的「倒计时 / 循环定时器」；
//!   可挂在组件上（每个实体独立计时）或作为资源（全局共享）
//! - Timer 本身不是 Component，要挂到实体上需包一层自定义组件（本示例的 BlinkTimer）
//! - Timer::from_seconds + TimerMode（Once 单次 / Repeating 循环）
//! - tick 后用 just_finished()（本 tick 刚好结束，只触发一次）判断，
//!   区别于 is_finished()（已结束，持续为 true）

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 倒计时资源：全局共享的循环定时器 + 当前剩余秒数
#[derive(Resource)]
struct Countdown {
    timer: Timer,
    remaining: u32,
}

// 闪烁组件：内嵌一个 Timer，作为「定时器组件」挂在圆形上。
// 注意：Timer 本身不实现 Component，需要这样包一层自定义组件才能挂到实体上。
#[derive(Component)]
struct BlinkTimer(Timer);

// 运行时长文字标记（用于系统里定位并更新文字）
#[derive(Component, Clone, Default)]
struct ElapsedText;

// 倒计时文字标记
#[derive(Component, Clone, Default)]
struct CountdownText;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        // 倒计时资源：每 1 秒 tick 一次，循环（Repeating）；初始剩余 5 秒
        .insert_resource(Countdown {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            remaining: 5,
        })
        .add_systems(Startup, setup)
        // 三个系统：推进圆形闪烁、更新运行时长、推进倒计时
        .add_systems(Update, (tick_blinker, update_elapsed_text, tick_countdown))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 圆形：挂 BlinkTimer 组件（每 0.5 秒循环），演示「Timer 作为组件」的用法。
    // 每个实体拥有独立的计时器，互不影响。
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.6, 1.0)))),
        Transform::from_xyz(0.0, 80.0, 0.0),
        BlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Visibility::Visible,
    ));

    // 顶部：运行时长文字（由 update_elapsed_text 每帧更新）
    commands.spawn_scene(bsn! {
        ElapsedText
        Text2d::new("已运行 0.0 秒")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, 250.0, 0.0)
    });

    // 底部：倒计时文字（由 tick_countdown 更新）
    commands.spawn_scene(bsn! {
        CountdownText
        Text2d::new("倒计时：5")
        TextColor(Color::srgb(1.0, 0.8, 0.2))
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, -250.0, 0.0)
    });
}

// 圆形闪烁：推进每个带 BlinkTimer 组件的实体，tick 结束时切换可见 / 隐藏。
fn tick_blinker(
    time: Res<Time>,
    mut query: Query<(&mut BlinkTimer, &mut Visibility)>,
    // 系统本地状态：记录当前是否可见（跨帧保留，这里只有一个闪烁实体）
    mut visible: Local<bool>,
) {
    for (mut blink_timer, mut visibility) in &mut query {
        // BlinkTimer 是元组结构体，字段 .0 才是真正的 Timer；
        // tick 用 time.delta()（Duration 类型）推进定时器；
        // just_finished() 只在「本 tick 刚好结束」的那一帧返回 true（不会每帧都触发）。
        if blink_timer.0.tick(time.delta()).just_finished() {
            *visible = !*visible;
            *visibility = if *visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            info!(
                "[定时器] 圆形闪烁：{}",
                if *visible { "显示" } else { "隐藏" }
            );
        }
    }
}

// 运行时长：用 Time::elapsed_secs 更新顶部文字（每 1 秒一次，避免每帧重排 CJK 文本刷屏）。
// 注意：Time 是只读资源，Bevy 每帧自动更新，无需像 Timer 那样手动 tick。
fn update_elapsed_text(
    time: Res<Time>,
    mut query: Query<&mut Text2d, With<ElapsedText>>,
    mut last_update: Local<f32>,
) {
    if time.elapsed_secs() - *last_update > 1.0 {
        *last_update = time.elapsed_secs();
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("已运行 {:.1} 秒", time.elapsed_secs());
        }
    }
}

// 倒计时：推进倒计时资源（每 1 秒），每次 tick 结束递减数字并更新文字。
fn tick_countdown(
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut query: Query<&mut Text2d, With<CountdownText>>,
) {
    if countdown.timer.tick(time.delta()).just_finished() {
        // 递减倒计时；到 0 时重置为 5（循环演示）
        countdown.remaining = if countdown.remaining == 0 {
            5
        } else {
            countdown.remaining - 1
        };
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("倒计时：{}", countdown.remaining);
        }
        info!("[倒计时] 剩余 {} 秒", countdown.remaining);
    }
}
