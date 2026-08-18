//! Bevy 0.19 入门示例：异步 I/O 任务池（IoTaskPool）—— 模拟网络下载。
//! 把耗时 I/O（网络请求、磁盘读写）丢到 IoTaskPool 后台执行，主线程不阻塞，
//! 每帧轮询任务，完成后取回结果。
//!
//! 学习重点：
//! - `IoTaskPool`：专门做「阻塞 I/O」的后台线程池（网络、文件读写）
//! - `AsyncComputeTaskPool`：专门做「CPU 密集计算」的线程池（见 example_async_task）
//! - 两者用法一致：`spawn(async move { ... })` 启动，返回 `Task<T>`
//! - 每帧用 `is_finished()` 非阻塞检查，`block_on(poll_once(...))` 取结果
//! - 模拟网络分阶段：连接 → 下载 → 解析（用 sleep 模拟真实耗时）
//!
//! 观察：终端日志分阶段打印，主线程游戏循环始终不卡。

use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use std::time::Duration;

// 保存后台下载任务的句柄和完成标记
#[derive(Resource)]
struct DownloadTask {
    task: Task<String>,
    done: bool,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, check_task)
        .run()
}

fn setup(mut commands: Commands) {
    // 在 I/O 线程池启动「下载」任务（用 sleep 模拟网络耗时）。
    // 真实网络请求见 example_network_request。
    let task = IoTaskPool::get().spawn(async move {
        info!("[下载] 正在连接服务器...");
        std::thread::sleep(Duration::from_millis(800));

        info!("[下载] 连接成功，开始下载...");
        std::thread::sleep(Duration::from_millis(1200));

        info!("[下载] 下载完成，正在解析...");
        std::thread::sleep(Duration::from_millis(400));

        // 返回「下载」到的内容
        "{\"name\": \"Bevy\", \"version\": \"0.19\"}".to_string()
    });

    commands.insert_resource(DownloadTask { task, done: false });
    info!("[下载] 已在后台启动，主线程继续运行");
}

// 每帧轮询任务：完成后取回结果（只处理一次）。
fn check_task(mut task: ResMut<DownloadTask>) {
    if task.done {
        return;
    }

    // 未完成就跳过（非阻塞，主线程不卡）
    if !task.task.is_finished() {
        return;
    }

    // 已完成，取出结果
    if let Some(result) = block_on(poll_once(&mut task.task)) {
        info!("[下载] 完成！结果 = {result}");
        task.done = true;
    }
}
