use bevy::prelude::*;

// 分数资源：全局共享的游戏分数，用 init_resource::<Score>() 初始化（见 main.rs）。
// Resource 是全局单例数据（不属于任何实体），任何系统都能读写。
// 与组件（Component，挂在实体上）不同，Resource 是独立的全局状态。
#[derive(Resource, Default)]
pub struct Score {
    // 当前总分：初始 0（Default 自动赋值），每次子弹击中敌人 +100（见 check_collision）
    pub total: u32,
}
