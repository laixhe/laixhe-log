//! Bevy 0.19 入门示例：演示计算状态（ComputedStates）。
//!
//! 学习重点：
//! - ComputedStates 根据源状态自动推导派生状态
//! - compute 返回 None 时，State<Self> 资源被移除
//! - 派生状态不能手动 set_state，完全由源状态变化驱动
//!
//! 操作：菜单按空格进入游戏；游戏内按 P 暂停/继续；按 ESC 回菜单。

use bevy::prelude::*;

// 源状态：InGame 带 paused 字段
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    InGame {
        paused: bool,
    },
}

// 派生状态：当 AppState 是 InGame 时存在
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct InGame;

impl ComputedStates for InGame {
    type SourceStates = AppState;

    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::InGame { .. } => Some(InGame),
            _ => None,
        }
    }
}

// 派生状态：当 AppState 是 InGame { paused: true } 时存在
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Paused;

impl ComputedStates for Paused {
    type SourceStates = AppState;

    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::InGame { paused: true } => Some(Paused),
            _ => None,
        }
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_computed_state::<InGame>()
        .add_computed_state::<Paused>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(InGame), enter_in_game)
        .add_systems(OnExit(InGame), exit_in_game)
        .add_systems(OnEnter(Paused), enter_paused)
        .add_systems(OnExit(Paused), exit_paused)
        .add_systems(Update, input)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("[计算状态] 菜单：按 空格 进入游戏，按 P 暂停/继续");
}

fn enter_in_game() {
    info!("[计算状态] InGame 派生状态激活");
}

fn exit_in_game() {
    info!("[计算状态] InGame 派生状态移除");
}

fn enter_paused() {
    info!("[计算状态] Paused 派生状态激活");
}

fn exit_paused() {
    info!("[计算状态] Paused 派生状态移除");
}

fn input(keys: Res<ButtonInput<KeyCode>>, state: Res<State<AppState>>, mut commands: Commands) {
    match state.get() {
        AppState::Menu => {
            if keys.just_pressed(KeyCode::Space) {
                commands.set_state(AppState::InGame { paused: false });
            }
        }
        AppState::InGame { paused } => {
            if keys.just_pressed(KeyCode::KeyP) {
                commands.set_state(AppState::InGame { paused: !paused });
            }
            if keys.just_pressed(KeyCode::Escape) {
                commands.set_state(AppState::Menu);
            }
        }
    }
}
