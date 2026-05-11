use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::thread;

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
