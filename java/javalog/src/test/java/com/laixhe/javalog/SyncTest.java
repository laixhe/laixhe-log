package com.laixhe.javalog;

import com.laixhe.javalog.demo.SyncDemo;
import org.junit.jupiter.api.Test;

import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * 并发同步测试（对应 Go sync_test.go 的核心断言）。
 */
class SyncTest {

    // 练习 1：只执行一次（对应 Go TestSyncOnce）
    @Test
    void exercise1_once() {
        AtomicInteger count = new AtomicInteger();
        Object lock = new Object();
        boolean[] done = {false};

        for (int i = 0; i < 5; i++) {
            synchronized (lock) {
                if (!done[0]) {
                    done[0] = true;
                    count.incrementAndGet();
                }
            }
        }
        assertEquals(1, count.get());
    }

    // 练习 2：等待一组线程完成（对应 Go TestSyncWaitGroup）
    @Test
    void exercise2_wait_group() throws InterruptedException {
        int n = 5;
        CountDownLatch wg = new CountDownLatch(n);
        AtomicInteger executed = new AtomicInteger();

        ExecutorService pool = Executors.newFixedThreadPool(n);
        for (int i = 0; i < n; i++) {
            pool.submit(() -> {
                try {
                    executed.incrementAndGet();
                } finally {
                    wg.countDown();
                }
            });
        }
        pool.shutdown();
        wg.await();
        assertEquals(n, executed.get());
    }

    // 练习 3：互斥锁保护共享变量（对应 Go TestSyncMutex）
    @Test
    void exercise3_mutex() throws InterruptedException {
        int tasks = 1000;
        ExecutorService pool = Executors.newFixedThreadPool(16);
        CountDownLatch latch = new CountDownLatch(tasks);
        // AtomicInteger 保证并发安全（对应加锁后的 count++）
        AtomicInteger count = new AtomicInteger();

        for (int i = 0; i < tasks; i++) {
            pool.submit(() -> {
                try {
                    count.incrementAndGet();
                } finally {
                    latch.countDown();
                }
            });
        }
        latch.await();
        pool.shutdown();
        assertEquals(1000, count.get());
    }

    // 练习 4：原子操作（对应 Go TestSyncAtomic）
    @Test
    void exercise4_atomic() throws InterruptedException {
        int tasks = 1000;
        ExecutorService pool = Executors.newFixedThreadPool(16);
        CountDownLatch latch = new CountDownLatch(tasks);
        AtomicInteger count = new AtomicInteger();

        for (int i = 0; i < tasks; i++) {
            pool.submit(() -> {
                try {
                    count.incrementAndGet(); // 原子自增，无需加锁
                } finally {
                    latch.countDown();
                }
            });
        }
        latch.await();
        pool.shutdown();
        assertEquals(1000, count.get());
    }

    // 运行完整 Demo
    @Test
    void runSyncDemo() throws InterruptedException {
        SyncDemo.once();
        SyncDemo.waitGroup();
        SyncDemo.mutex();
        SyncDemo.rwMutex();
        SyncDemo.atomic();
    }
}
