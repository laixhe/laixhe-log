//! Bevy 0.19 入门示例：演示状态转换事件（StateTransitionEvent）+ 完整游戏流程。
//!
//! 学习重点：
//! - StateTransitionEvent 是 Message，用 MessageReader 读取
//! - 每次状态转换都会发送事件，包含 exited（旧）和 entered（新）状态
//! - 用 Resource 保存分数，跨状态持久存在
//!
//! 操作：菜单按空格开始；游戏中按空格得分、ESC 结束；结算按 R 回菜单（分数清零）。

use bevy::prelude::*;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
struct Score(u32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::GameOver), enter_game_over)
        .add_systems(
            Update,
            (
                menu_input.run_if(in_state(AppState::Menu)),
                playing_input.run_if(in_state(AppState::Playing)),
                game_over_input.run_if(in_state(AppState::GameOver)),
                watch_transitions,
            ),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("[流程] 菜单：按 空格 开始游戏");
}

fn enter_game_over(score: Res<Score>) {
    info!("[流程] 游戏结束！最终得分 {}", score.0);
}

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Space) {
        commands.set_state(AppState::Playing);
    }
}

fn playing_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        score.0 += 1;
        info!("[流程] 得分 +1，当前 {}", score.0);
    }
    if keys.just_pressed(KeyCode::Escape) {
        commands.set_state(AppState::GameOver);
    }
}

fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        score.0 = 0;
        commands.set_state(AppState::Menu);
    }
}

// 监听所有状态转换事件
fn watch_transitions(mut events: MessageReader<StateTransitionEvent<AppState>>) {
    for event in events.read() {
        info!("[转换] {:?} -> {:?}", event.exited, event.entered);
    }
}
