use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// 工作线程
struct Worker {
    id: usize,
    worker_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // 初始化工作线程
    // id       线程ID
    // receiver 带原子锁的消费者，用来保证同时只能只有一个消费者
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        // 创建新的线程
        let worker_thread = thread::spawn(move || {
            // 循环等待
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("worker {} got a job!", id);
                        job();
                    },
                    Message::End => {
                        println!("worker {} got a end!", id);
                        break;
                    }
                }
            }
        });

        Worker{
            id: id,
            worker_thread: Some(worker_thread),
        }
    }
}

// 发送类型-定义处理的类型
type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job), // 普通消息
    End,         // 结束消息
}

// 线程池
pub struct ThreadPool {
    workers: Vec<Worker>,          // 工作线程
    sender: mpsc::Sender<Message>, // 生产者 (消息传递来跨线程传递数据 channel)
}

impl ThreadPool {
    // size 线程池的个数
    pub fn new(size: usize) -> ThreadPool {

        // 初始化并设置容量
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        // sender生产者，receiver消费者 (多个生产者，一个消费者 (消息传递来跨线程传递数据 channel))
        let (sender, receiver) = mpsc::channel();
        // 带原子锁的消费者，用来保证同时只能只有一个消费者 (Arc<T> 多重所有权的原子引用计数类型 (用于多线程场景))
        let arc_mutex_receiver = Arc::new(Mutex::new(receiver));

        // 初始化工作线程
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&arc_mutex_receiver)));
        }

        ThreadPool{
            workers: workers,
            sender: sender,
        }

    }

    // 处理请求
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        // 发送闭包
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// 实现 Drop 类似于 析构函数
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 发送 结束消息
        for _ in &mut self.workers {
            self.sender.send(Message::End).unwrap();
        }

        for worker in &mut self.workers {
            // 等待 线程结束
            // worker.worker_thread.take() 将 Option 中的存放的类型移动出来，并设置为 None
            if let Some(worker_thread) = worker.worker_thread.take() {
                worker_thread.join().unwrap();
            }
        }

    }
}