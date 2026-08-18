//! 并发基础：线程 / 互斥锁 / 消息通道。
//!
//! ## 前置知识
//! - `std::thread::spawn` 创建线程（对应 Go goroutine / Java 线程 / Python threading）
//! - `Mutex` 互斥锁保护共享数据（对应 Go Mutex / C# lock）
//! - `mpsc::channel` 线程间消息传递（对应 Go channel / Python queue）
//!
//! ## 练习题
//! 1. 用 mpsc 通道实现生产者-消费者，生产者发 5 个数字。
//! 2. 用 Mutex 保护计数器，10 个线程各加 100 次，验证结果。

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============ 线程基础 ============
pub fn threads() {
    // spawn 创建线程（对应 Go go func / Java new Thread）
    let handle = thread::spawn(|| {
        println!("子线程运行中");
    });

    handle.join().unwrap(); // 等待线程结束（对应 Go WaitGroup.Wait）
    println!("主线程继续");
}

// ============ 线程闭包捕获 ============
pub fn thread_capture() {
    let data = vec![1, 2, 3];

    // move：将 data 所有权移入线程（对应 Go 捕获变量需要复制）
    let handle = thread::spawn(move || {
        println!("子线程数据: {:?}", data);
    });

    handle.join().unwrap();
    // println!("{:?}", data); // ❌ data 已被 move，主线程无法访问
}

// ============ 互斥锁 Mutex ============
pub fn mutex_demo() {
    // Mutex：互斥锁（对应 Go sync.Mutex / C# lock）
    let counter = Arc::new(Mutex::new(0)); // Arc：多线程共享所有权

    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().unwrap(); // 加锁（对应 Go Lock）
            *guard += 1; // 临界区
            // guard 离开作用域自动解锁（RAII，对应 Go Unlock）
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Mutex 计数: {}", *counter.lock().unwrap()); // 10
}

// ============ 消息通道 mpsc ============
pub fn channel_demo() {
    // mpsc：多生产者单消费者通道（对应 Go channel / Python queue）
    let (tx, rx) = mpsc::channel();

    // 生产者线程：发送 3 个值
    let tx2 = tx.clone(); // 多生产者需要 clone
    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    thread::spawn(move || {
        tx2.send(100).unwrap();
    });

    // 消费者：接收所有消息（对应 Go range channel）
    let mut received = vec![];
    for msg in rx {
        received.push(msg);
    }
    println!("收到消息: {:?}", received); // 包含 1 2 3 100（顺序不保证）
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    // 练习 1：生产者-消费者
    #[test]
    fn exercise_1_producer_consumer() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for i in 1..=5 {
                tx.send(i).unwrap();
            }
        });
        let sum: i32 = rx.iter().sum();
        assert_eq!(sum, 15);
    }

    // 练习 2：Mutex 计数 10 线程各加 100
    #[test]
    fn exercise_2_mutex_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    *c.lock().unwrap() += 1;
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*counter.lock().unwrap(), 1000);
    }
}
