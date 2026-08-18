use bevy::state::state::States;

// ==================== 游戏状态定义 ====================
/// 游戏的三种状态：主菜单、游戏中、游戏结束
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu, // 主菜单界面
    Playing,  // 游戏进行中
    GameOver, // 游戏结束界面
}
