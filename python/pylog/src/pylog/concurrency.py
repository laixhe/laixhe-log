"""主题：并发同步（threading 线程 / Lock / 事件）。

对应 Go golog/sync_test.go。

对应关系：
- sync.Once         → 双检锁模式（Python 没有内置 Once，用 Lock + 标志实现）
- sync.WaitGroup    → Thread.join() 等待所有线程结束
- sync.Mutex        → threading.Lock（保护共享变量）
- sync.RWMutex      → 读多写少用 Lock 即可（Python 无内置 RWMutex）
- sync/atomic       → GIL 下 int 自增原子性有保证，但推荐用 Lock 显式保护
"""

import threading
import time


def run() -> None:
    print("========== 并发同步 ==========")

    # ---------- 1. 只执行一次（对应 sync.Once）----------
    lock = threading.Lock()
    done = [False]
    count = [0]

    def do_once(action):
        if not done[0]:  # 第一次检查（快路径）
            with lock:
                if not done[0]:  # 第二次检查（加锁后）
                    action()
                    done[0] = True

    def init_once():
        count[0] += 1
        print("初始化执行（只会执行一次）")

    threads = [threading.Thread(target=lambda: do_once(init_once)) for _ in range(5)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    print("count =", count[0])  # 结果: count = 1

    # ---------- 2. 等待一组线程完成（对应 sync.WaitGroup）----------
    results = []

    def worker(n: int) -> None:
        results.append(n)

    threads = [threading.Thread(target=worker, args=(i,)) for i in range(5)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()  # 对应 wg.Wait()：阻塞直到线程结束
    print("所有线程完成, results =", sorted(results))  # [0, 1, 2, 3, 4]

    # ---------- 3. 互斥锁：保护共享变量（对应 sync.Mutex）----------
    mutex = threading.Lock()
    protected = [0]

    def incr():
        for _ in range(100):
            with mutex:  # 加锁（对应 mu.Lock() / mu.Unlock()）
                protected[0] += 1  # 临界区：同一时刻只有一个线程能执行

    threads = [threading.Thread(target=incr) for _ in range(10)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    print("protectedCount =", protected[0])  # 结果: 1000（不加锁可能得到错误结果）

    # ---------- 4. 原子操作（对应 sync/atomic）----------
    # Python 的 GIL 保证单条字节码指令的原子性，但复合操作仍需加锁
    counter = [0]

    def atomic_incr():
        for _ in range(100):
            counter[0] += 1

    threads = [threading.Thread(target=atomic_incr) for _ in range(10)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    print("count =", counter[0])  # 结果: 1000

    # ---------- 5. 线程池（对应 Go goroutine + WaitGroup 组合场景）----------
    from concurrent.futures import ThreadPoolExecutor

    def square(x: int) -> int:
        time.sleep(0.01)  # 模拟耗时
        return x * x

    with ThreadPoolExecutor(max_workers=4) as pool:
        futures = [pool.submit(square, i) for i in range(5)]
        results = [f.result() for f in futures]  # 等待所有任务完成
    print("线程池平方:", results)  # [0, 1, 4, 9, 16]
