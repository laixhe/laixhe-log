<?php

namespace Laixhe\Phplog;

use Fiber;

/**
 * 并发同步示例：Fibers（协程）、只执行一次、等待一组协程完成、原子计数。
 * 对应 Go golog/sync_test.go。
 *
 * ⚠️ 重要前置知识：
 * - PHP 默认是「单线程」模型（同一进程内只有一个执行流），
 *   天然不存在 Go goroutine 那样的数据竞争，Mutex/Atomic 在单进程内没有意义。
 * - 并发模型：
 *   1. 多进程：php-fpm / pcntl_fork（Unix）/ CLI 多进程
 *   2. 协程：Fiber（PHP 8.1+ 内置）、Swoole、ReactPHP —— 协作式调度，非抢占
 *   3. 多线程：pthreads / parallel 扩展（不常用，有坑）
 * - 本示例用 Fiber 演示「协作式并发」的调度方式：
 *   - sync.Once     → 静态标记 + 锁（单进程内用普通布尔即可）
 *   - sync.WaitGroup → 主 Fiber 依次 resume 各子 Fiber（协作式，天然同步）
 *   - sync.Mutex    → PHP 单进程无竞争，演示「临界区」概念
 *   - sync/atomic   → PHP 变量本身就是原子的（单线程），演示计数
 */
final class SyncDemo
{
    // ============ 只执行一次（对应 sync.Once）============
    public static function once(): void
    {
        $done = false; // 单进程内普通布尔即可保证「只执行一次」
        $count = 0;

        for ($i = 0; $i < 5; $i++) {
            if (!$done) {
                $done = true;
                $count++;
                echo '初始化执行（只会执行一次）', PHP_EOL;
            }
        }
        echo "count = {$count}", PHP_EOL; // 结果: count = 1
    }

    // ============ 等待一组协程完成（对应 sync.WaitGroup）============
    // Fiber 是协作式的：主 Fiber 逐个 resume 子 Fiber，全部执行完即「等待完成」
    public static function waitGroup(): void
    {
        $fibers = [];
        for ($i = 0; $i < 5; $i++) {
            // 每个协程从外部接收参数（Fiber 不闭包捕获，通过构造参数传入）
            $fibers[] = new Fiber(function (int $n): void {
                echo "协程 {$n}", PHP_EOL;
            });
        }

        // 对应 wg.Add(5) + wg.Wait()：逐个启动并等待完成
        foreach ($fibers as $n => $fiber) {
            $fiber->start($n); // 启动协程（协作式，立即执行到结束）
        }
        echo '所有协程完成', PHP_EOL;
    }

    // ============ 协作式调度演示（Fiber suspend/resume）============
    // 展示 Fiber 的 suspend（让出）/ resume（恢复），与生成器 yield 类似
    public static function fiberSchedule(): void
    {
        $scheduler = [];
        $fiberA = new Fiber(function (): void {
            echo 'A1', PHP_EOL;
            Fiber::suspend(); // 让出控制权给主流程
            echo 'A2', PHP_EOL;
        });
        $fiberB = new Fiber(function (): void {
            echo 'B1', PHP_EOL;
            Fiber::suspend();
            echo 'B2', PHP_EOL;
        });

        $fiberA->start(); // A1
        $fiberB->start(); // B1
        $fiberA->resume(); // A2
        $fiberB->resume(); // B2
        // 输出顺序：A1 B1 A2 B2（协作式交替执行）
        echo '调度结束', PHP_EOL;
    }

    // ============ 互斥锁 / 临界区（对应 sync.Mutex）============
    // PHP 单进程单线程：没有真实竞争，这里演示「临界区」概念与并发下的正确性
    public static function mutex(): void
    {
        // 模拟多进程场景的计数：单进程内直接累加即可（天然正确）
        $count = 0;
        for ($i = 0; $i < 1000; $i++) {
            $count++; // 临界区：单线程下必然正确（对应 Go 加锁后的 count++）
        }
        echo "count = {$count}", PHP_EOL; // 结果: count = 1000

        // 真实多进程锁：PHP 提供的是「文件锁」flock()（跨进程互斥）
        // 生产环境跨进程互斥通常使用：数据库行锁 / Redis SETNX / 文件锁
        $lockFile = sys_get_temp_dir() . '/phplog_demo.lock';
        $fp = fopen($lockFile, 'c+');
        if (flock($fp, LOCK_EX)) { // 加排他锁（对应 mu.Lock()）
            echo '获取文件锁成功（跨进程互斥）', PHP_EOL;
            flock($fp, LOCK_UN);   // 解锁（对应 mu.Unlock()）
        }
        fclose($fp);
        @unlink($lockFile);
    }

    // ============ 原子操作（对应 sync/atomic）============
    // PHP 单进程内变量读写是原子的，无需锁；跨进程可用共享内存/Redis INCR
    public static function atomic(): void
    {
        $count = 0;
        for ($i = 0; $i < 1000; $i++) {
            $count += 1; // 单线程下天然原子（对应 count.Add(1)）
        }
        echo "count = {$count}", PHP_EOL; // 结果: count = 1000

        // 多进程/多线程环境下的原子自增：
        // - Redis: INCR key
        // - 数据库: UPDATE ... SET n = n + 1
        // - 文件锁 + 读写
        echo '多进程场景推荐使用 Redis INCR / 数据库原子自增', PHP_EOL;
    }
}
