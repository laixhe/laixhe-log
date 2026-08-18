//! Bevy 0.19 入门示例：真实网络请求（ureq）—— 异步 HTTP GET。
//! 用 IoTaskPool 在后台发起 HTTP 请求，不阻塞主线程，完成后取回响应文本。
//!
//! 学习重点：
//! - `ureq`：轻量同步 HTTP 客户端，`ureq::get(url).call()` 发起请求
//! - 把同步请求丢进 `IoTaskPool` 后台执行，就变成「异步」——主线程不卡
//! - `.config().timeout_global(...)` 设置超时，避免请求永久挂起
//! - 错误处理：网络不可用 / 超时 / 状态码错误
//! - 每帧用 `is_finished()` + `block_on(poll_once(...))` 取回结果
//!
//! 观察：终端打印请求结果（或失败原因）。可把 URL 改成任意接口测试。

use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use std::time::Duration;

// 保存后台请求任务的句柄和完成标记
#[derive(Resource)]
struct RequestTask {
    task: Task<Result<String, String>>,
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
    // 在 I/O 线程池发起真实 HTTP 请求（阻塞式 ureq 放到后台线程，不卡主线程）。
    let task = IoTaskPool::get().spawn(async move {
        // 可替换成任意 URL；httpbin 返回 JSON，方便观察
        let url = "https://httpbin.org/get";

        // ureq 3.x：ureq::get(url) 返回 RequestBuilder，.config().build() 后再 .call()
        // 成功返回 http::Response<Body>，失败返回 ureq::Error
        match ureq::get(url)
            .config()
            .timeout_global(Some(Duration::from_secs(10)))
            .build()
            .call()
        {
            // 读取响应体为字符串；read_to_string 失败（如非 UTF-8）也转成错误信息
            Ok(mut resp) => resp.body_mut().read_to_string().map_err(|e| e.to_string()),
            // 网络不可用 / 超时 / 非 2xx 状态码都会走到这里
            Err(e) => Err(format!("请求失败：{e}")),
        }
    });

    commands.insert_resource(RequestTask { task, done: false });
    info!("[网络] 已发起异步请求，主线程继续运行");
}

// 每帧轮询任务，完成后打印结果或错误。
fn check_task(mut task: ResMut<RequestTask>) {
    if task.done {
        return;
    }

    if !task.task.is_finished() {
        return;
    }

    if let Some(result) = block_on(poll_once(&mut task.task)) {
        match result {
            Ok(body) => info!("[网络] 请求成功，响应：\n{body}"),
            Err(e) => info!("[网络] 请求失败：{e}"),
        }
        task.done = true;
    }
}
