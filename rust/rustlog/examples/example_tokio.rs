//! 异步运行时示例（tokio）。
//!
//! ## 前置知识
//! - **async/await**：Rust 的异步编程语法，async fn 返回 Future，.await 等待完成
//! - **tokio**：最流行的 Rust 异步运行时，提供任务调度、定时器、I/O 等
//! - `#[tokio::main]`：宏，把 async fn main 包装成同步 main + 运行时
//!
//! ## async vs 线程
//! | 特性 | async (tokio) | 线程 (std::thread) |
//! |---|---|---|
//! | 调度方式 | 协作式（.await 让出） | 抢占式（OS 调度） |
//! | 内存开销 | ~2KB/任务 | ~2MB/线程 |
//! | 适合场景 | I/O 密集（HTTP/DB） | CPU 密集 |
//! | 数量上限 | 十万级 | 数千级 |
//!
//! ## 练习题
//! 1. 用 `tokio::spawn` 启动 3 个并发任务，每个等不同时间，用 `join!` 等待全部完成。
//! 2. 用 `tokio::sync::Mutex` 在多个异步任务间共享一个计数器。
//! 3. 用 `tokio::select!` 同时等待两个 channel，哪个先来处理哪个。

use std::time::Duration;
use tokio::time::sleep;

// ============ async fn 基础 ============

// async fn 不会立即执行，返回一个 Future
// 调用 .await 时才真正执行，遇到 I/O 或定时器时让出执行权
async fn say_after(delay: Duration, msg: &str) {
    sleep(delay).await;  // 异步休眠，不阻塞线程
    println!("  → {msg}");
}

pub async fn basic_async() {
    println!(" basic_async：顺序执行两个异步任务");

    // 顺序 await：总耗时 = 100ms + 50ms = 150ms
    say_after(Duration::from_millis(100), "第一个 (100ms)").await;
    say_after(Duration::from_millis(50), "第二个 (50ms)").await;
    println!(" basic_async：完成（顺序执行总耗时 ~150ms）");
}

// ============ spawn：并发任务 ============

pub async fn concurrent_tasks() {
    println!(" concurrent_tasks：3 个任务并发执行");

    // tokio::spawn：创建并发任务（类似 go func）
    // 返回 JoinHandle，可以用 .await 等待完成
    let t1 = tokio::spawn(say_after(Duration::from_millis(100), "任务 1 (100ms)"));
    let t2 = tokio::spawn(say_after(Duration::from_millis(50),  "任务 2 (50ms)"));
    let t3 = tokio::spawn(say_after(Duration::from_millis(150), "任务 3 (150ms)"));

    // join!：等待全部完成，总耗时 ≈ max(100, 50, 150) = 150ms（而非 300ms）
    let _ = tokio::join!(t1, t2, t3);
    println!(" concurrent_tasks：全部完成（并发执行总耗时 ~150ms）");
}

// ============ mpsc channel：任务间通信 ============

pub async fn channel_demo() {
    println!(" channel_demo：mpsc 通道演示（生产者 → 消费者）");

    // mpsc：多生产者单消费者通道
    // channel(8) 表示缓冲区容量为 8
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(8);

    // 生产者任务
    tokio::spawn(async move {
        for i in 0..3 {
            let msg = format!("消息 #{i}");
            tx.send(msg).await.expect("通道发送失败"); // 免责：接收端存活
            sleep(Duration::from_millis(50)).await;
        }
        // tx 离开作用域时自动关闭 → 接收端 rx.recv() 返回 None
    });

    // 消费者：循环接收直到通道关闭
    while let Some(msg) = rx.recv().await {
        println!("  → 收到: {msg}");
    }
    println!(" channel_demo：通道已关闭");
}

// ============ select!：多路复用 ============

pub async fn select_demo() {
    println!(" select_demo：多个 Future 同时等，第一个完成即返回");

    let fast = sleep(Duration::from_millis(50));
    let slow = sleep(Duration::from_millis(200));

    // select!：同时等待多个异步操作，先完成的分支执行，其余取消
    tokio::select! {
        _ = fast => println!("  → fast (50ms) 先完成！"),
        _ = slow => println!("  → slow (200ms) 先完成！"),
    }
    // 期望：fast 先完成
}

// ============ Mutex：异步互斥锁 ============

pub async fn mutex_demo() {
    println!(" mutex_demo：多任务共享计数器（tokio::sync::Mutex）");

    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Arc + Mutex：异步安全的共享可变状态
    // 注意：用 tokio::sync::Mutex（不是 std::sync::Mutex）
    // 原因：std Mutex 在 await 时持锁可能导致死锁，tokio Mutex 不会
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        // 每个任务给计数器 +1
        let handle = tokio::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for h in handles {
        let _ = h.await;
    }

    let final_count = *counter.lock().await;
    println!("  → 5 个任务各自 +1，最终计数 = {final_count}");
}

#[tokio::main]
async fn main() {
    println!("============= basic_async =============");
    basic_async().await;

    println!("\n============= concurrent_tasks =============");
    concurrent_tasks().await;

    println!("\n============= channel_demo =============");
    channel_demo().await;

    println!("\n============= select_demo =============");
    select_demo().await;

    println!("\n============= mutex_demo =============");
    mutex_demo().await;
}
