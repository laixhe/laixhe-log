use ringbuf::{traits::*, HeapRb};

fn main() {
    let rb = HeapRb::<i32>::new(2); // 创建容量为2的堆上环形缓冲区
    let (mut prod, mut cons) = rb.split(); // 分离生产者和消费者

    // 生产数据
    assert!(prod.try_push(10).is_ok());
    assert!(prod.try_push(20).is_ok()); // 尝试填满缓冲区

    // 消费数据
    assert_eq!(cons.try_pop(), Some(10));
    assert_eq!(cons.try_pop(), Some(20));

    // 注意：尝试超过缓冲区容量推送数据时，将返回错误
}
