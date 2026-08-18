#include "StdThread.h"

#include <atomic>     // std::atomic
#include <format>     // std::format [C++20]
#include <iostream>
#include <mutex>      // std::mutex / std::lock_guard / std::once_flag
#include <thread>     // std::thread
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

namespace
{
    // 带锁的共享计数器（对应 Go 的 `var mu sync.Mutex; var count int`）
    struct Counter {
        std::mutex mutex;
        int value = 0;
    };

    void Incr(Counter& counter, int times)
    {
        for (int i = 0; i < times; i++) {
            std::lock_guard lock(counter.mutex); // 加锁（对应 mu.Lock()/Unlock()）
            counter.value++;                     // 临界区
        }
    }

    void AtomicIncr(std::atomic<int>& counter, int times)
    {
        for (int i = 0; i < times; i++) {
            counter.fetch_add(1); // 原子自增（对应 count.Add(1)），无需加锁
        }
    }
} // namespace

StdThread::StdThread()
{
    // ===== 1. 只执行一次（对应 sync.Once）=====
    std::cout << "--- 只执行一次 ---" << std::endl;
    std::once_flag once;
    int count = 0;
    for (int i = 0; i < 5; i++) {
        std::call_once(once, [&] { count++; }); // 无论调用多少次，只执行一次
    }
    PRINT("count = {}", count); // count = 1

    // ===== 2. 等待一组线程完成（对应 sync.WaitGroup）=====
    std::cout << "--- 等待一组线程完成 ---" << std::endl;
    std::vector<std::thread> threads;
    for (int i = 0; i < 5; i++) {
        threads.emplace_back([i] { PRINT("线程 {}", i); });
    }
    for (auto& t : threads) {
        t.join(); // 对应 wg.Wait()：阻塞直到线程结束
    }
    PRINT("所有线程完成");

    // ===== 3. 互斥锁保护共享变量（对应 sync.Mutex）=====
    std::cout << "--- 互斥锁 ---" << std::endl;
    Counter counter;
    threads.clear();
    for (int i = 0; i < 10; i++) {
        threads.emplace_back(Incr, std::ref(counter), 100); // 10 线程 × 100 次
    }
    for (auto& t : threads) {
        t.join();
    }
    PRINT("count = {}（加锁保证 1000）", counter.value);

    // ===== 4. 原子操作（对应 sync/atomic）=====
    std::cout << "--- 原子操作 ---" << std::endl;
    std::atomic<int> atomic_count{0};
    threads.clear();
    for (int i = 0; i < 10; i++) {
        threads.emplace_back(AtomicIncr, std::ref(atomic_count), 100);
    }
    for (auto& t : threads) {
        t.join();
    }
    PRINT("count = {}（原子自增，无需加锁）", atomic_count.load());
}
