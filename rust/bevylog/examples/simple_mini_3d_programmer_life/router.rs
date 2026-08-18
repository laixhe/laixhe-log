use bevy::state::state::States;

// ==================== 页面状态机 ====================
/// 三个页面：主菜单 → 游戏中 → 结局页
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}
