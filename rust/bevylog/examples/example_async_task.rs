//! Bevy 0.19 入门示例：演示异步任务（AsyncComputeTaskPool）。
//!
//! 把耗时计算丢到后台线程池执行，主线程（游戏循环）不阻塞，
//! 每帧轮询任务是否完成，完成后取回结果。
//!
//! 学习重点：
//! - AsyncComputeTaskPool::get().spawn()：在后台线程池启动异步任务，返回 Task<T>
//! - Task::is_finished()：非阻塞地检查任务是否完成
//! - block_on(poll_once(...))：从已完成的 Task 取出结果
//! - 适合 CPU 密集计算；I/O 密集场景用 IoTaskPool

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};

// 保存后台任务的句柄
#[derive(Resource)]
struct ComputeTask(Task<u64>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, check_task)
        .run()
}

fn setup(mut commands: Commands) {
    // 在后台线程池启动耗时计算（模拟 CPU 密集任务）
    let task = AsyncComputeTaskPool::get().spawn(async move {
        // 模拟耗时 3 秒的计算
        std::thread::sleep(std::time::Duration::from_secs(3));
        // 返回计算结果
        42u64
    });
    commands.insert_resource(ComputeTask(task));
    info!("[异步任务] 已在后台启动耗时计算，主线程继续运行");
}

// 每帧轮询任务状态，完成后取回结果（只处理一次）
fn check_task(
    mut task: ResMut<ComputeTask>,
    mut done: Local<bool>,
    mut last_log: Local<f32>,
    time: Res<Time>,
) {
    if *done {
        return;
    }

    // 任务完成前，每秒打印一次状态，展示主线程没有被阻塞
    if !task.0.is_finished() {
        if time.elapsed_secs() - *last_log > 1.0 {
            *last_log = time.elapsed_secs();
            info!("[异步任务] 计算中...（主线程未阻塞）");
        }
        return;
    }

    // 任务已完成，取出结果
    if let Some(result) = block_on(poll_once(&mut task.0)) {
        info!("[异步任务] 计算完成，结果 = {}", result);
        *done = true;
    }
}
