//! Bevy 0.19 入门示例：演示组件存储类型（Table vs SparseSet）。
//!
//! 学习重点：
//! - 默认 #[derive(Component)] 使用 Table 存储：适合大多数组件，迭代快
//! - #[component(storage = "SparseSet")] 使用稀疏存储：适合「少数实体拥有」「频繁增删」的组件
//! - 增删 Table 组件会移动实体所在 archetype；增删 SparseSet 组件不会
//! - 两种存储的查询写法完全一致，区别只在底层性能
//!
//! 观察：日志展示两种组件都能正常查询，值相同。

use bevy::prelude::*;

// Table 存储（默认）
#[derive(Component)]
struct TableComp(i32);

// SparseSet 存储
#[derive(Component)]
#[component(storage = "SparseSet")]
struct SparseComp(i32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, report)
        .run()
}

fn setup(mut commands: Commands) {
    for i in 0..5 {
        commands.spawn((TableComp(i), SparseComp(i * 10)));
    }
}

fn report(
    time: Res<Time>,
    q_table: Query<&TableComp>,
    q_sparse: Query<&SparseComp>,
    mut last_log: Local<f32>,
) {
    // 每秒打印一次，避免刷屏
    if time.elapsed_secs() - *last_log < 1.0 {
        return;
    }
    *last_log = time.elapsed_secs();

    let table: Vec<i32> = q_table.iter().map(|c| c.0).collect();
    let sparse: Vec<i32> = q_sparse.iter().map(|c| c.0).collect();
    info!("[存储] Table 组件值: {table:?}");
    info!("[存储] SparseSet 组件值: {sparse:?}");
}
