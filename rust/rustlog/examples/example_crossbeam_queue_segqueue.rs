//! 无锁队列 SegQueue（crossbeam-queue）：多生产者、多消费者（MPMC）。
//!
//! ## 前置知识
//! 标准库只有 Mutex 包裹的同步方式，但在高并发场景下：
//! - Mutex 会让线程阻塞、发生上下文切换（开销大）
//! - 还可能有「惊群」「优先级反转」等问题
//!
//! **Crossbeam** 提供了高性能的无锁（Lock-free）数据结构，用 CPU 原子指令（CAS）替代锁。
//!
//! crossbeam-queue 提供两种队列：
//! | 类型 | 容量 | 性能 | 适用场景 |
//! |---|---|---|---|
//! | **SegQueue<T>** | 无界（无限增长） | 略低（push 需要动态分配链表节点） | 消息日志、任务投递、不知道数据上限 |
//! | **ArrayQueue<T>** | 有界（固定容量） | 更高（连续数组、无分配） | 资源受限、背压（满时拒绝） |
//!
//! ## 练习题
//! 1. 改成 `ArrayQueue::new(32)`，在消费者里 `force_pop` 或 `pop`，观察有界队列的行为。
//! 2. 把 `Worker 4 个` 调大到 `100 个线程`，看看 SegQueue 是否仍然正确。
//! 3. 用 `Arc<Mutex<Vec<T>>>` 实现一个相同的日志队列，对比一下两者的代码复杂度。

use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::thread;

fn main() {
    // Arc + SegQueue：跨线程共享无锁队列（无需 Mutex！）
    let log_queue = Arc::new(SegQueue::new());
    let mut tasks = vec![];

    const N_WORKERS: usize = 4;

    // ---- 生产者：4 个工作线程并发写入日志 ----
    for i in 0..N_WORKERS {
        let q = Arc::clone(&log_queue);
        tasks.push(thread::spawn(move || {
            // 每个 worker 写入自己编号的 3 条日志，总共 4*3=12 条
            for seq in 1..=3 {
                let log_entry = format!("Worker {}: message #{}", i, seq);
                q.push(log_entry);
            }
        }));
    }

    // 等待所有生产者线程结束
    for t in tasks {
        t.join().expect("生产者线程 panic");
    }

    println!(
        "✅ 所有 {} 个生产者已完成，队列长度 = {}",
        N_WORKERS,
        log_queue.len()
    );

    // ---- 消费者：主线程依次消费所有日志 ----
    // SegQueue 是 MPMC，这里单消费者演示，实际可以再开多个消费者线程
    let mut consumed = 0;
    while let Some(entry) = log_queue.pop() {
        consumed += 1;
        println!("  [消费 #{}] Log received: {}", consumed, entry);
    }
    // 因为 SegQueue 是链表结构，消费顺序大致是 FIFO（多线程并发 push 时并不严格保序）
    println!("共消费 {} 条日志", consumed);
    assert_eq!(consumed, N_WORKERS * 3, "消息不能丢！");
    println!("✅ 断言通过：所有消息都被消费");
}
