//! 环形缓冲区 ringbuf：**单生产者单消费者（SPSC）** 高性能无锁缓冲。
//!
//! ## 前置知识
//! 环形缓冲区（Circular Buffer / Ring Buffer）用一块固定大小的连续内存 + 头尾指针实现：
//! - 满了：生产者等待或丢弃
//! - 空了：消费者等待
//!
//! ringbuf crate 提供多种实现：
//! | 类型 | 线程安全 | 分配方式 | 说明 |
//! |---|---|---|---|
//! | `HeapRb<T>`     | ✅（跨线程共享） | 堆动态 | 推荐大多数场景，默认用它 |
//! | `SharedRb<T>`   | ✅                | 可选 | HeapRb/StaticRb 的通用别名 |
//! | `LocalRb<T>`    | ❌（单线程）      | 堆     | 仅单线程内部解耦生产消费 |
//! | `StaticRb<T, N>`| ✅                | 栈静态 | 避免堆分配，容量是 const 泛型 |
//!
//! 典型用途：
//! - 音视频数据帧缓冲（不能丢帧 + 低延迟）
//! - 日志异步写入磁盘（先写进 buffer，专用线程批量刷盘）
//! - 串口/USB 数据流处理
//!
//! ## 练习题
//! 1. 把容量改成 1，观察 `try_push` 第 3 次会返回什么错误？
//! 2. 开两个线程：一个线程不断 push、一个不断 pop，演示真正跨线程 SPSC。
//! 3. 查文档：`push_blocking` / `pop_blocking`（等待空位/数据）和 `try_push` 的区别。

use ringbuf::{traits::*, HeapRb};

fn main() {
    // ---- 场景 1：基础 push/pop 演示 ----
    basic_demo();

    // ---- 场景 2：满容量时的行为（覆盖？丢弃？报错？）----
    full_capacity_demo();

    println!("\n✅ 所有 ringbuf 断言通过");
}

fn basic_demo() {
    println!("==== basic_demo ====");

    // 容量 2 的堆上环形缓冲区
    let rb = HeapRb::<i32>::new(2);
    // split() 把 ringbuf 分成「生产者句柄 prod」和「消费者句柄 cons」
    // 这样两者可以分别 move 到不同线程
    let (mut prod, mut cons) = rb.split();

    // try_push：非阻塞推送，满时返回 Err(PushError(value))
    assert!(prod.try_push(10).is_ok());
    assert!(prod.try_push(20).is_ok());
    println!("容量 2：push 10, 20 → 已满（剩余空位={}）", prod.vacant_len());

    // 此时再 push 就会失败，返回原值
    let result = prod.try_push(30);
    assert!(result.is_err(), "满了再 push 必须 Err");
    println!("push 第 3 个(30) → 如预期失败，被拒值 = {:?}", result.err());

    // try_pop：非阻塞消费，空时返回 None
    assert_eq!(cons.try_pop(), Some(10));
    assert_eq!(cons.try_pop(), Some(20));
    assert_eq!(cons.try_pop(), None, "空时 pop 返回 None");
    println!("依次 pop → 10, 20，再 pop → None（空）");
}

fn full_capacity_demo() {
    println!("\n==== full_capacity_demo ====");

    let rb = HeapRb::<String>::new(3);
    let (mut prod, mut cons) = rb.split();

    // 填满
    for i in 1..=3 {
        prod.try_push(format!("item-{}", i)).ok(); // 丢弃 Err
    }
    println!("填满容量 3 → 内容 [item-1, item-2, item-3]");

    // 消费一半 → 继续生产（环形指针绕回来）
    let first = cons.try_pop();
    println!("pop 第 1 个 = {:?}", first); // item-1

    prod.try_push("item-4".into()).ok();
    println!("再 push item-4 → 现在内部是 [item-2, item-3, item-4]");

    // 依次 pop 剩下 3 个
    while let Some(v) = cons.try_pop() {
        println!("  pop → {}", v); // 2 → 3 → 4（严格先进先出）
    }
}
