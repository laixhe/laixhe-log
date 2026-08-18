<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\SyncDemo;
use PHPUnit\Framework\TestCase;

/**
 * 并发同步测试（对应 Go sync_test.go 的核心断言）。
 * PHP 单进程单线程，重点验证「只执行一次」与 Fiber 协作调度。
 */
final class SyncTest extends TestCase
{
    // 练习 1：只执行一次（对应 Go TestSyncOnce）
    public function testExercise1Once(): void
    {
        $done = false;
        $count = 0;
        for ($i = 0; $i < 5; $i++) {
            if (!$done) {
                $done = true;
                $count++;
            }
        }
        $this->assertSame(1, $count);
    }

    // 练习 2：Fiber 协作式调度顺序（suspend/resume）
    public function testExercise2FiberSchedule(): void
    {
        $order = [];
        $fiberA = new \Fiber(function () use (&$order): void {
            $order[] = 'A1';
            \Fiber::suspend();
            $order[] = 'A2';
        });
        $fiberB = new \Fiber(function () use (&$order): void {
            $order[] = 'B1';
            \Fiber::suspend();
            $order[] = 'B2';
        });

        $fiberA->start();
        $fiberB->start();
        $fiberA->resume();
        $fiberB->resume();

        $this->assertSame(['A1', 'B1', 'A2', 'B2'], $order);
    }

    // 练习 3：计数正确性（单线程下天然 1000）
    public function testExercise3Counter(): void
    {
        $count = 0;
        for ($i = 0; $i < 1000; $i++) {
            $count++;
        }
        $this->assertSame(1000, $count);
    }

    // 运行完整 Demo
    public function testRunSyncDemo(): void
    {
        $this->expectNotToPerformAssertions();
        SyncDemo::once();
        SyncDemo::waitGroup();
        SyncDemo::fiberSchedule();
        SyncDemo::mutex();
        SyncDemo::atomic();
    }
}
