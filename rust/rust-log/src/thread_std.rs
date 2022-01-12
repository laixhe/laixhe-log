use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

// 创建线程
pub fn thread_std_run(){

    // 创建新的线程
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("for spawn: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("for: {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    // 阻塞并等待线程执行结束
    handle.join().unwrap();
}

// 消息传递来跨线程传递数据 channel
pub fn thread_std_channel_run1(){
    // 多个生产者，一个消费者
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv 阻塞的， try_recv 是不阻塞的
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 消息传递来跨线程传递数据 channel
pub fn thread_std_channel_run2(){
    // 多个生产者，一个消费者
    let (tx, rx) = mpsc::channel();
    // 通过克隆创建多个发送者(生产者)
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {

        let vals = vec![
            String::from("111"),
            String::from("222"),
            String::from("333"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {

        let vals = vec![
            String::from("444"),
            String::from("555"),
            String::from("666"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// 共享状态的并发 Mutex(互斥锁)
pub fn thread_std_mutex_run(){
    // Arc<T> 多重所有权的原子引用计数类型 (用于多线程场景)
    let arc_mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        // 通过克隆创建多个原子锁
        let counter = Arc::clone(&arc_mutex);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *arc_mutex.lock().unwrap());
}