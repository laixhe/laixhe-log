package com.laixhe.javalog.demo;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.ReentrantLock;
import java.util.concurrent.locks.ReentrantReadWriteLock;

/**
 * 并发同步示例：只执行一次、等待一组线程完成、互斥锁、读写锁、原子操作。
 * 对应 Go golog/sync_test.go。
 *
 * 对应关系：
 * - sync.Once        → AtomicBoolean CAS / 双检锁单例
 * - sync.WaitGroup   → CountDownLatch（或 ExecutorService 的 shutdown+awaitTermination）
 * - sync.Mutex       → synchronized / ReentrantLock
 * - sync.RWMutex     → ReentrantReadWriteLock（读并发、写互斥）
 * - sync/atomic      → AtomicInteger / AtomicLong（无锁并发安全计数）
 */
public final class SyncDemo {

    private SyncDemo() {
    }

    // ============ 只执行一次（对应 sync.Once）============
    // 双检锁（Double-Checked Locking）单例：无论调用多少次，init 只执行一次
    public static void once() {
        class OnceHolder {
            private volatile boolean done = false;

            public void doOnce(Runnable action) {
                if (!done) {                       // 第一次检查（无锁快路径）
                    synchronized (this) {
                        if (!done) {               // 第二次检查（加锁后）
                            action.run();
                            done = true;
                        }
                    }
                }
            }
        }

        OnceHolder once = new OnceHolder();
        AtomicInteger count = new AtomicInteger();
        // 即使循环多次调用，动作也只执行一次
        for (int i = 0; i < 5; i++) {
            once.doOnce(() -> {
                count.incrementAndGet();
                System.out.println("初始化执行（只会执行一次）");
            });
        }
        System.out.println("count = " + count); // 结果: count = 1
    }

    // ============ 等待一组线程完成（对应 sync.WaitGroup）============
    public static void waitGroup() throws InterruptedException {
        int n = 5;
        CountDownLatch wg = new CountDownLatch(n); // 计数器初始化为 5

        ExecutorService pool = Executors.newFixedThreadPool(n);
        for (int i = 0; i < n; i++) {
            int task = i;
            pool.submit(() -> {
                try {
                    System.out.println("线程 " + task);
                } finally {
                    wg.countDown(); // 对应 wg.Done()：计数 -1
                }
            });
        }
        pool.shutdown();
        wg.await(); // 阻塞直到计数归零（对应 wg.Wait()）
        System.out.println("所有线程完成");
    }

    // ============ 互斥锁：保护共享变量（对应 sync.Mutex）============
    public static void mutex() throws InterruptedException {
        ReentrantLock mu = new ReentrantLock();
        AtomicInteger count = new AtomicInteger(); // 这里用 AtomicInteger 展示"不加锁也行"
        int[] protectedCount = {0};                // 数组绕过 lambda 捕获限制

        ExecutorService pool = Executors.newFixedThreadPool(16);
        int tasks = 1000;
        CountDownLatch latch = new CountDownLatch(tasks);
        for (int i = 0; i < tasks; i++) {
            pool.submit(() -> {
                try {
                    mu.lock();                     // 加锁（对应 mu.Lock()）
                    protectedCount[0]++;           // 临界区：同一时刻只有一个线程能执行
                    mu.unlock();                   // 解锁（对应 mu.Unlock()）
                    count.incrementAndGet();
                } finally {
                    latch.countDown();
                }
            });
        }
        latch.await(); // 对应 wg.Wait()
        pool.shutdown();
        System.out.println("count = " + count.get()); // 结果: count = 1000
        System.out.println("protectedCount = " + protectedCount[0]); // 1000（不加锁会得到错误结果）
    }

    // ============ 读写锁：读可并发、写互斥（对应 sync.RWMutex）============
    public static void rwMutex() {
        ReentrantReadWriteLock rw = new ReentrantReadWriteLock();
        ReentrantReadWriteLock.WriteLock writeLock = rw.writeLock();
        ReentrantReadWriteLock.ReadLock readLock = rw.readLock();
        String[] data = {"初始值"};

        // 写锁：独占，与所有读/写互斥
        writeLock.lock();
        data[0] = "新值";
        writeLock.unlock();

        // 读锁：多个线程可同时持有
        readLock.lock();
        System.out.println("读取: " + data[0]); // 结果: 读取: 新值
        readLock.unlock();
    }

    // ============ 原子操作（对应 sync/atomic，无锁并发安全计数）============
    public static void atomic() throws InterruptedException {
        AtomicInteger count = new AtomicInteger(); // 对应 atomic.Int64

        ExecutorService pool = Executors.newFixedThreadPool(16);
        int tasks = 1000;
        CountDownLatch latch = new CountDownLatch(tasks);
        for (int i = 0; i < tasks; i++) {
            pool.submit(() -> {
                try {
                    count.incrementAndGet(); // 原子自增，无需加锁（对应 count.Add(1)）
                } finally {
                    latch.countDown();
                }
            });
        }
        latch.await();
        pool.shutdown();
        System.out.println("count = " + count.get()); // 结果: count = 1000
    }
}
