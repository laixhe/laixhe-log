//! Bevy 0.19 入门示例：演示 Table vs SparseSet 迭代性能对比。
//!
//! 学习重点：
//! - Table 存储连续排列，迭代快（缓存友好）
//! - SparseSet 存储间接寻址，迭代相对慢，但增删快
//! - 用 std::time::Instant 测量遍历耗时，直观对比
//!
//! 观察：日志对比 Table 与 SparseSet 遍历大量实体的耗时。

use bevy::prelude::*;
use std::time::Instant;

const ENTITY_COUNT: usize = 50_000;

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
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, benchmark)
        .run()
}

fn setup(mut commands: Commands) {
    // 生成大量实体，每个同时带 Table 和 SparseSet 组件（纯数据，不渲染）
    for i in 0..ENTITY_COUNT {
        commands.spawn((TableComp(i as i32), SparseComp(i as i32)));
    }
    info!("[基准] 生成了 {ENTITY_COUNT} 个实体");
}

fn benchmark(
    time: Res<Time>,
    q_table: Query<&TableComp>,
    q_sparse: Query<&SparseComp>,
    mut last_log: Local<f32>,
) {
    if time.elapsed_secs() - *last_log < 1.0 {
        return;
    }
    *last_log = time.elapsed_secs();

    // 计时 Table 迭代（累加求和，强制真实读取，避免被优化掉）
    let start = Instant::now();
    let mut table_sum = 0i64;
    for c in &q_table {
        table_sum += c.0 as i64;
    }
    let table_time = start.elapsed();

    // 计时 SparseSet 迭代
    let start = Instant::now();
    let mut sparse_sum = 0i64;
    for c in &q_sparse {
        sparse_sum += c.0 as i64;
    }
    let sparse_time = start.elapsed();

    info!(
        "[基准] Table 求和 {table_sum} 耗时 {table_time:?} | SparseSet 求和 {sparse_sum} 耗时 {sparse_time:?}"
    );
}
