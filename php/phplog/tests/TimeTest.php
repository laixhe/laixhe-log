<?php

namespace Laixhe\Phplog\Tests;

use DateTimeImmutable;
use DateTimeZone;
use Laixhe\Phplog\TimeDemo;
use PHPUnit\Framework\TestCase;

/**
 * 时间处理测试（对应 Go time_test.go 的核心断言）。
 */
final class TimeTest extends TestCase
{
    // 练习 1：时间格式化与解析（对应 Go TestTime / TestTimeParse）
    public function testExercise1FormatParse(): void
    {
        $fmt = 'Y-m-d H:i:s';
        $this->assertSame('2025-06-21 17:18:39', (new DateTimeImmutable('2025-06-21 17:18:39'))->format($fmt));

        $parsed = new DateTimeImmutable('2018-01-06 16:12:00');
        $this->assertSame(2018, (int) $parsed->format('Y'));
        $this->assertSame(1, (int) $parsed->format('n'));
    }

    // 练习 2：时区差异（对应 Go TestTimeParse 中 Parse 与 ParseInLocation）
    public function testExercise2TimezoneOffset(): void
    {
        $utc = new DateTimeImmutable('2018-01-06 16:12:00', new DateTimeZone('UTC'));
        $local = new DateTimeImmutable('2018-01-06 16:12:00', new DateTimeZone('Asia/Shanghai'));
        // 东八区比 UTC 早 8 小时，UTC 时间戳更大
        $this->assertSame(8 * 3600, $utc->getTimestamp() - $local->getTimestamp());
    }

    // 练习 3：时间比较（对应 Go TestTimeBeforeAfter）
    public function testExercise3BeforeAfter(): void
    {
        $now = new DateTimeImmutable();
        $t1 = $now->modify('-1 second');
        $t2 = $now->modify('+1 hour');
        $this->assertFalse($t2 < $t1);
        $this->assertTrue($t2 > $t1);
        $this->assertTrue($t1 < $now && $now < $t2);
    }

    // 练习 4：耗时计算（对应 Go TestTimeSinceUntil）
    public function testExercise4Duration(): void
    {
        $start = microtime(true);
        usleep(50_000);
        $elapsed = microtime(true) - $start;
        $this->assertGreaterThanOrEqual(0.05, $elapsed);

        $remaining = (new DateTimeImmutable('+3 seconds'))->getTimestamp() - time();
        $this->assertLessThanOrEqual(3, $remaining);
    }

    // 运行完整 Demo
    public function testRunTimeDemo(): void
    {
        $this->expectNotToPerformAssertions();
        TimeDemo::timeBasics();
        TimeDemo::timeParse();
        TimeDemo::timeBeforeAfter();
        TimeDemo::timeSinceUntil();
        TimeDemo::timeZone();
    }
}
