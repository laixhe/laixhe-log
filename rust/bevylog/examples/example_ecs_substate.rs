//! Bevy 0.19 入门示例：演示子状态（SubStates）。
//!
//! 学习重点：
//! - #[derive(SubStates)] + #[source(...)] 定义嵌套在父状态下的子状态
//! - 子状态只在父状态激活时存在，父状态离开时子状态自动移除
//! - 子状态和父状态一样，可用 OnEnter / in_state / set_state
//!
//! 操作：菜单按空格进入游戏；准备阶段按 1 战斗、战斗按 2 结算、结算按 ESC 回菜单。

use bevy::prelude::*;

// 父状态
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

// 子状态：只在 AppState::InGame 下存在
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
enum GamePhase {
    #[default]
    Setup,
    Battle,
    Conclusion,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_sub_state::<GamePhase>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), enter_menu)
        .add_systems(OnEnter(GamePhase::Setup), enter_setup)
        .add_systems(OnEnter(GamePhase::Battle), enter_battle)
        .add_systems(OnEnter(GamePhase::Conclusion), enter_conclusion)
        .add_systems(
            Update,
            (
                menu_input.run_if(in_state(AppState::Menu)),
                game_input.run_if(in_state(AppState::InGame)),
            ),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("[子状态] 菜单：按 空格 进入游戏");
}

fn enter_menu() {
    info!("[子状态] 进入菜单");
}

fn enter_setup() {
    info!("[子状态] 进入准备阶段（按 1 进入战斗）");
}

fn enter_battle() {
    info!("[子状态] 进入战斗阶段（按 2 进入结算）");
}

fn enter_conclusion() {
    info!("[子状态] 进入结算阶段（按 ESC 回菜单）");
}

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Space) {
        commands.set_state(AppState::InGame);
    }
}

fn game_input(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Digit1) {
        commands.set_state(GamePhase::Battle);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        commands.set_state(GamePhase::Conclusion);
    }
    if keys.just_pressed(KeyCode::Escape) {
        commands.set_state(AppState::Menu);
    }
}
