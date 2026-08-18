//! 启动画面模块：显示 Bevy Logo 3 秒后自动切换到主菜单。
//!
//! 学习重点：
//! - SplashTimer 资源 + Timer 倒计时驱动状态切换到 Menu
//! - run_if(in_state) 让倒计时系统只在 Splash 态运行
//! - bsn! 场景语法 + commands.spawn_scene 构建 UI 节点
//! - DespawnOnExit::<GlobalGameState>(Splash) 退出 Splash 时自动清理场景实体

use super::GlobalGameState;
use bevy::prelude::*;

// newtype 模式：derive Deref/DerefMut 后可像 Timer 一样调用 .tick()，同时作为 Resource 跨系统共享
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

// 插件函数：把本模块的系统注册进 App。pub 让 main 能跨模块调用
pub fn splash_plugin(app: &mut App) {
    app // OnEnter(Splash)：每次进入 Splash 状态时执行一次 splash_setup
        .add_systems(OnEnter(GlobalGameState::Splash), splash_setup)
        // Update 每帧跑，但 .run_if 限定只在 Splash 态才真正执行 countdown
        .add_systems(Update, countdown.run_if(in_state(GlobalGameState::Splash)));
}

fn splash_setup(mut commands: Commands) {
    // spawn_scene + bsn! 宏：声明式地构建 UI 实体树，每行一个组件或子节点
    commands.spawn_scene(bsn! {
        // 退出 Splash 状态时引擎自动 despawn 此实体，无需手动清理
        DespawnOnExit::<GlobalGameState>(GlobalGameState::Splash)
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)) // 深灰底色
        Node { // Node 是 UI 节点的布局组件（基于 flexbox）
            align_items: AlignItems::Center,         // 交叉轴居中
            justify_content: JustifyContent::Center, // 主轴居中
            width: Val::Percent(100.0),  // 占满父宽度
            height: Val::Percent(100.0), // 占满父高度
        }
        Children [ // 子节点列表
            ImageNode { // 图片节点，字符串路径由 asset_server 自动加载
                image: "images/bevy_logo_bevy.png"
            }
        ]
    });
    // Once：倒计时一次就停（Repeating 则会循环重置）；3 秒后切到菜单
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(
    // NextState 是「下一帧状态」的写入句柄，set 后由引擎在帧末切换；State 则是当前状态的只读视图
    mut game_state: ResMut<NextState<GlobalGameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    // Timer 不会自动推进，需每帧用 time.delta() 喂入流逝时间；到点后 is_finished 才返回 true
    if timer.tick(time.delta()).is_finished() {
        info!("[状态] Splash → Menu（启动画面倒计时结束）");
        game_state.set(GlobalGameState::Menu);
    }
}
