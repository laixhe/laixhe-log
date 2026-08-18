//! Bevy 0.19 入门示例：打砖块（含启动画面）。
//! 完整游戏循环：启动画面 → 主菜单 → 准备 → 游戏中 → 胜利/失败，ESC 暂停或返回菜单。
//!
//! 学习重点：
//! - 多文件模块组织：mod 拆分 splash / menu / game 三个子模块，各自导出插件
//! - 分层状态机：顶层 GlobalGameState(Splash/Menu/Game)，game 内嵌套 GameState(Ready/Play/Pause/GameOver/GameWin)，menu 内嵌套 MenuState(Main/Settings)
//! - GameSettings 资源：菜单里调整砖块行列数，进入游戏时读取并生成关卡
//! - 插件化装配：splash_plugin / menu_plugin / game_plugin 在 main 中统一 add_plugins

// prelude 是 Bevy 预导出的常用类型集合（App/Component/Resource/Query 等），星号导入省去逐个 use
use bevy::prelude::*;
// 声明三个子模块，各自实现一个插件；通过 super::GlobalGameState 等可跨模块访问
mod game;
mod menu;
mod splash;

// 顶层游戏状态机：States 派生后成为可被 in_state/OnEnter 识别的状态标签
// Hash+Eq+Clone+Copy 是 States 的硬性要求（引擎需比较/哈希状态值）
// #[default] 标注 Splash，App 启动时自动进入启动画面
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GlobalGameState {
    #[default]
    Splash, // 启动画面：显示 Logo 3 秒
    Menu, // 主菜单/设置页
    Game, // 游戏内（再由 GameState 细分 Ready/Play/...）
}

// Resource：全局共享的「单例」数据，任意系统可通过 Res/ResMut 读写
// 这里存放可调参数，菜单改它、游戏读它
#[derive(Resource)]
struct GameSettings {
    brick_rows: usize,    // 砖块行数（菜单可调 1..=10）
    brick_columns: usize, // 砖块列数（菜单可调 1..=20）
}

// 实现 Default 让 init_resource 能自动构造（否则需 insert_resource 显式传入）
impl Default for GameSettings {
    fn default() -> Self {
        Self {
            brick_rows: 5,
            brick_columns: 10,
        }
    }
}

fn main() -> AppExit {
    // App 是 Bevy 应用的核心容器，串联「资源 + 插件 + 系统 + 状态」并驱动主循环
    App::new()
        // DefaultPlugins 含 Window/Render/Input/Asset/Time 等必备插件，.set 可覆盖默认配置
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 打砖块".to_string(),
                resizable: false, // 禁止拉伸，简化碰撞边界计算
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        // ClearColor：每帧清屏颜色（浅灰背景）
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        // 固定时间步长：以 120Hz 推进 FixedUpdate 调度，让物理与帧率解耦（高帧率下球速不变）
        .insert_resource(Time::<Fixed>::from_hz(120.0))
        // 初始化顶层状态机：注册 GlobalGameState 并设为 #[default]=Splash，后续用 ResMut<NextState<..>> 切换
        .init_state::<GlobalGameState>()
        // init_resource 用 Default 构造；insert_resource 则传入现成实例
        .init_resource::<GameSettings>()
        // Startup 调度只在应用启动时跑一次（区别于 Update 每帧、FixedUpdate 固定步长）
        .add_systems(Startup, setup)
        // 装配三个子模块的插件，各自负责注册自己的系统/状态/资源
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        // .run() 进入主循环，阻塞直到 App 退出，返回 AppExit 表示退出状态
        .run()
}

// 启动系统：2D 场景必须有相机才能渲染，Camera2d 是正交相机的便捷构造
// commands.spawn 用「组件元组」创建实体（这里只挂一个 Camera2d 组件）
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
