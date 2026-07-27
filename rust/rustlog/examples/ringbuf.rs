use ringbuf::{traits::*, HeapRb};

/*
环形缓冲区
无锁单生产者-单消费者(`SPMC`)模式的环形缓冲区。
它提供了不同类型的缓冲区，包括适用于单线程环境的 `LocalRb` 和可以在线程间共享的 `SharedRb`，
后者利用动态内存分配（推荐大多数场景使用的是 `HeapRb`）和静态内存分配的选项（如 `StaticRb`）。

在处理大量网络数据或实时日志记录时，使用 `ringbuf` 可以有效地缓冲数据，避免频繁的内存分配与复制。
*/

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
