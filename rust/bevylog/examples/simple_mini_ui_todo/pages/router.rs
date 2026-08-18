use bevy::state::state::States;

// ==================== 页面状态定义 ====================
/// 应用的两个页面：待办清单页 / 设置页
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppPage {
    #[default]
    Todo, // 待办清单页（主页面）
    Settings, // 设置页
}
