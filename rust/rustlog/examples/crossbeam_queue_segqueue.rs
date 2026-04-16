use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::thread;

/*
无锁队列

在 `Rust` 标准库提供了基础的线程和通道支持，但在处理复杂的并发场景时，往往显得不够顺手。
`Crossbeam` 是一套并发编程工具集，它填补了标准库的空白，特别是提供了高性能的无锁数据结构（`Lock-free Data Structures`）。
相比于使用 `Mutex` 带来的锁竞争开销，`Crossbeam` 的 `SegQueue` 在多生产者、多消费者的场景下表现更为优异。

此队列采用链表形式实现，每个链表节点是一个小型缓冲区，可容纳少量元素。
队列中同时容纳的元素数量没有上限。
不过，由于在推送元素时需要动态分配节点，因此该队列的性能略逊于 `ArrayQueue`。

相比于使用 `Mutex` 带来的锁竞争开销，对于 `SegQueue` 在多生产者、多消费者的场景下表现更为优异。
*/

fn main() {
    // 创建一个跨线程共享的无锁队列
    let log_queue = Arc::new(SegQueue::new());
    let mut tasks = vec![];

    // 模拟4个工作线程并发写入日志
    for i in 0..4 {
        let q = Arc::clone(&log_queue);
        tasks.push(thread::spawn(move || {
            let log_entry = format!("Worker {} done", i);
            q.push(log_entry);
        }));
    }

    // 等待所有线程完成
    for t in tasks {
        t.join().unwrap();
    }

    // 主线程消费队列数据
    while let Some(entry) = log_queue.pop() {
        println!("Log received: {}", entry);
    }
}
