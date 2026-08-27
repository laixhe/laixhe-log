//! Bevy 0.19 入门示例：演示 bevy_enhanced_input —— 增强输入系统。
//!
//! 灵感来自虚幻引擎的 Enhanced Input，核心是把「按键」和「游戏行为」解耦：
//! - Action（动作）：玩家能做的行为（如「移动」「跳跃」），与具体按键无关
//! - Binding（绑定）：把动作连接到具体输入源（键盘按键 / 鼠标 / 手柄）
//! - Context（上下文）：输入所属的状态（本例是 Player 组件），决定哪些动作生效
//!
//! 学习重点：
//! - 插件：EnhancedInputPlugin + app.add_input_context::<Player>() 注册上下文
//! - #[derive(InputAction)] + #[action_output(类型)] 定义动作（f32 / Vec2 / bool）
//! - actions! / bindings! 宏：把动作和绑定挂到上下文实体上
//! - 推式 API（观察者）：On<Fire<A>> 按住时每帧触发；On<Start<A>> 按下瞬间触发一次
//! - 拉式 API（轮询）：直接查询 Single<&Action<A>> 读取动作当前值
//!
//! 操作：A / D 左右移动，空格跳跃（带重力），观察底部信息文本和终端日志。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use bevy_enhanced_input::prelude::*;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
const MOVE_SPEED: f32 = 300.0; // 移动速度（像素/秒）
const JUMP_SPEED: f32 = 500.0; // 起跳速度（像素/秒，向上）
const GRAVITY: f32 = 900.0; // 重力加速度（像素/秒²）

// 输入上下文：加在玩家实体上，动作系统会查询它来决定哪些动作生效
#[derive(Component)]
struct Player;

// 垂直速度组件：跳跃 / 重力只作用于 y 轴（x 轴由 Move 动作直接驱动）
#[derive(Component)]
struct Velocity(f32);

// 动作类型：Move 输出一维方向值（f32，范围 -1~1）
#[derive(InputAction)]
#[action_output(f32)]
struct Move;

// 动作类型：Jump 输出 bool（是否触发）
#[derive(InputAction)]
#[action_output(bool)]
struct Jump;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        .add_plugins(EnhancedInputPlugin)
        // 注册输入上下文：之后 Player 实体上关联的动作才会被求值
        .add_input_context::<Player>()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // 推式 API：观察者响应动作事件
        .add_observer(move_player)
        .add_observer(jump_player)
        // 拉式 API + 常规物理：普通系统轮询动作值 / 处理重力
        .add_systems(Update, (apply_gravity, show_action_state))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 玩家实体：上下文组件 + 视觉 + 动作与绑定
    commands.spawn((
        Player,
        Velocity(0.0),
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(60.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        // actions! 宏：把动作挂到 Player 上下文上（同时生成动作实体和绑定实体）
        actions!(Player[
            // Move：绑定 D / A 两个按键（Bidirectional 预设 → 一维轴向值）
            (
                Action::<Move>::new(),
                Bindings::spawn(Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA)),
            ),
            // Jump：绑定空格（bindings! 宏直接把按键转成绑定）
            (Action::<Jump>::new(), bindings![KeyCode::Space]),
        ]),
    ));

    // 底部信息文本：由拉式 API 每帧更新
    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, -280.0, 0.0)
    });
}

// 推式 API 1：Fire<Move> 在动作「触发中」每帧触发（按住 A / D 期间）
fn move_player(
    trigger: On<Fire<Move>>,
    mut players: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // trigger.context：上下文实体（即 Player 实体）；trigger.value：本帧动作值（f32，-1~1）
    if let Ok(mut transform) = players.get_mut(trigger.context) {
        transform.translation.x += trigger.value * MOVE_SPEED * time.delta_secs();
    }
}

// 推式 API 2：Start<Jump> 在动作「开始触发」的瞬间触发一次（类似 bevy_input 的 just_pressed）
fn jump_player(trigger: On<Start<Jump>>, mut players: Query<&mut Velocity, With<Player>>) {
    if trigger.value {
        info!("[增强输入] 跳跃！（Start 事件，按下瞬间只触发一次）");
        if let Ok(mut velocity) = players.get_mut(trigger.context) {
            velocity.0 = JUMP_SPEED;
        }
    }
}

// 重力 + 落地：普通 Bevy 系统，与增强输入无关
fn apply_gravity(
    mut players: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut players {
        velocity.0 -= GRAVITY * time.delta_secs();
        transform.translation.y += velocity.0 * time.delta_secs();
        // 落地：y 回到 0，速度清零
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            velocity.0 = 0.0;
        }
    }
}

// 拉式 API：直接查询动作组件，轮询当前值（和读普通组件一样）
fn show_action_state(
    move_action: Query<&Action<Move>>,
    mut text: Single<&mut Text2d>,
    mut last_text: Local<String>,
) {
    let Ok(move_action) = move_action.single() else {
        return;
    };
    // &Action<Move> 先解引用到 Action，再解引用到 f32（Action 实现了 Deref<Target = Output>）
    let value: f32 = **move_action;
    let new_text = format!("A/D 左右移动 | 空格跳跃 | Move 当前值 = {:.2}", value);
    // 内容不变时不更新，避免每帧重排 CJK 文本刷 ICU4X 警告
    if *last_text != new_text {
        *last_text = new_text.clone();
        text.0 = new_text;
    }
}
