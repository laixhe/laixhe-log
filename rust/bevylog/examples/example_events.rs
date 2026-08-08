//! Bevy 0.19 入门示例：演示事件系统（Event / Observer / On / trigger）。
//! 按空格键触发「跳跃」事件，一个 Observer 监听并响应，累计跳跃 5 次后自动退出应用。
//!
//! 学习重点：
//! - Bevy 0.19 把「事件」分成两类，要区分清楚：
//!   * `Event`（本示例用的）：基于「观察者模式」，触发时所有观察者立即执行。
//!     适合「瞬时发生、触发即响应」的信号（如按键、碰撞、开火）。
//!     不需要关心系统执行顺序，也不需要 .chain() 排序。
//!   * `Message`：基于「双缓冲队列」，写入后由读取系统消费。
//!     同一帧内若写入系统先于读取系统执行，当帧就能读到；顺序反了则下一帧才能读到
//!     （双缓冲保证不丢失，最多延迟一帧）。需要关注系统顺序，常用 .chain() / .after() 排序。
//!     适合「一对多通知、需要跨帧处理」的场景。AppExit 就是 Message。
//! - 事件（Event）和资源（Resource）的区别：事件是「瞬时信号」，资源是「持续状态」（如分数、配置）。
//!
//! 本示例演示最常见的「全局事件 + 全局观察者」模式：
//! - #[derive(Event)] 定义事件（默认用 GlobalTrigger，触发时所有观察者都运行）
//! - App::add_observer 注册一个全局观察者函数
//! - Commands::trigger 在系统里触发事件

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册观察者：监听 JumpEvent，每当该事件被触发时就立即执行 handle_jump_event。
        // add_observer 可以链式调用注册多个观察者。
        // 观察者函数的第一个参数必须是 On<T>（T 是监听的事件类型）。
        .add_observer(handle_jump_event)
        // Startup 调度：启动时执行一次，初始化场景
        .add_systems(Startup, setup)
        // Update 调度：每帧执行一次，send_jump_event 检测空格键并触发事件。
        // 注意：和旧版 EventWriter/EventReader 不同，这里不需要 .chain() 排序——
        // 观察者在 trigger 命令应用时立即运行，不存在「当帧发送、下一帧才能收到」的跨帧延迟。
        .add_systems(Update, send_jump_event)
        .run()
}

// 跳跃事件：携带 player_id 表示「哪个玩家跳跃」。
// 事件本质就是普通结构体，关键是 #[derive(Event)] 让它成为事件类型。
// 默认派生会用 GlobalTrigger：触发时所有监听该事件的观察者都会运行。
// 字段可以是任意数据（这里用 u32 演示携带数据；也可以是空结构体，仅作信号用）。
#[derive(Event)]
struct JumpEvent {
    // 触发跳跃的玩家 ID（本示例只有玩家 0，用常量表示）
    player_id: u32,
}

// 玩家 ID 常量（避免魔法数字）
const PLAYER_ID: u32 = 0;
// 累计跳跃多少次后自动退出应用
const MAX_JUMPS: u32 = 5;

fn setup(mut commands: Commands) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);
    // 生成提示文本：告诉用户按空格键跳跃（spawn_scene + bsn! 宏声明式构建实体）
    commands.spawn_scene(bsn! {
        Text2d::new("按 空格 跳跃（累计 5 次后退出）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::default()
    });
}

// 事件触发系统：监听空格键按下，触发 JumpEvent。
// 不需要 EventWriter——通过 Commands::trigger 直接触发事件。
fn send_jump_event(
    // 键盘输入：ButtonInput 是资源，记录每个按键的按下/松开状态；Res 表示只读访问
    keyboard: Res<ButtonInput<KeyCode>>,
    // 命令队列：用于排队执行 spawn / trigger 等操作，命令会在当前系统结束后应用
    mut commands: Commands,
) {
    // just_pressed 检测「本帧刚按下」（区别于 pressed 的「按住中」）。
    // 用 just_pressed 而不是 pressed：避免按住空格时每帧都触发跳跃（只在按下瞬间触发一次）。
    if keyboard.just_pressed(KeyCode::Space) {
        // 触发一个跳跃事件：commands.trigger 把事件加入命令队列，
        // 命令应用时（本系统结束后立即应用）会立刻执行所有监听 JumpEvent 的观察者。
        commands.trigger(JumpEvent {
            player_id: PLAYER_ID,
        });
        info!("[触发] 玩家 {} 触发跳跃事件", PLAYER_ID);
    }
}

// 事件观察者：监听 JumpEvent，事件被触发时立即执行。
// 第一个参数必须是 On<T>，通过它可以访问事件数据（支持 Deref 到 T）。
// 后面可以加任意 SystemParam（如 Commands、Query、Res、Local 等），和普通系统一样。
fn handle_jump_event(
    // On<JumpEvent> 是触发上下文，通过 Deref 可以像 &JumpEvent 一样访问字段
    event: On<JumpEvent>,
    // 系统本地状态：每个观察者实例独有一份、跨帧保留。
    // 这里累计跳跃次数，达到 MAX_JUMPS 时请求应用退出。
    // Local<u32> 首次调用时用 u32::default() 初始化（即 0）
    mut jump_count: Local<u32>,
    // AppExit 是 Message（不是 Event），所以用 MessageWriter 写入而非 commands.trigger。
    // App::new() 默认已注册 AppExit 消息，无需手动 add_message。
    // 当帧结束后若检测到 AppExit 消息存在，主循环就会退出。
    mut exit_writer: MessageWriter<AppExit>,
) {
    *jump_count += 1;
    // event.player_id 通过 Deref 访问事件字段（On<T> 实现了 Deref<Target = T>）
    info!(
        "[观察者] 玩家 {} 跳跃！已累计 {} / {} 次",
        event.player_id, *jump_count, MAX_JUMPS
    );

    // 达到上限则请求退出
    if *jump_count >= MAX_JUMPS {
        info!("达到 {} 次上限，请求退出应用", MAX_JUMPS);
        // 写入 AppExit 消息：AppExit 是 Bevy 内置 Message 类型，专门用来请求应用退出。
        // AppExit::Success 表示正常退出（类似进程 exit code 0）；
        // 也可以用 AppExit::Error(NonZeroU8) 表示异常退出。
        // 写入后 Bevy 主循环会在当前帧结束时检测到并退出。
        exit_writer.write(AppExit::Success);
    }
    // 观察者执行完毕，事件即被消费——不存在「跨帧延迟」或「需要 MessageReader 读取」的情况。
    // 这是 Event（Observer 模式）与 Message（双缓冲队列）的核心区别：
    // Event 触发即执行、即时消费；Message 写入后要等读取系统消费，可能跨帧。
}
