<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\NumberDemo;
use PHPUnit\Framework\TestCase;

/**
 * 数值类型测试（对应 Rust number.rs 的 #[cfg(test)] 练习题）。
 */
final class NumberTest extends TestCase
{
    // 练习 1：格式化输出 666 的 8 位十六进制（前面补零）
    public function testExercise1HexLeadingZeros(): void
    {
        $this->assertSame('0000029A', sprintf('%08X', 666));
    }

    // 练习 2：饱和乘法（MAX * 3 不溢出/饱和）
    public function testExercise2SaturatingMul(): void
    {
        $result = min(PHP_INT_MAX, PHP_INT_MAX * 3);
        $this->assertSame(PHP_INT_MAX, $result);
    }

    // 练习 3：filter_var 严格校验（对应 TryFrom 返回 Err）
    public function testExercise3TryFromOverflow(): void
    {
        $this->assertFalse(filter_var('not_a_number', FILTER_VALIDATE_INT));
        $this->assertSame(200, filter_var('200', FILTER_VALIDATE_INT));
    }

    // 练习 4：溢出提升为 float（对应 checked_add 的差异）
    public function testExercise4OverflowPromotesToFloat(): void
    {
        $this->assertIsFloat(PHP_INT_MAX + 1);
    }

    // 练习 5：向零截断与四舍五入
    public function testExercise5TruncateAndRound(): void
    {
        $this->assertSame(3, (int) 3.99);        // 向零截断
        $this->assertSame(4, (int) round(3.99)); // 四舍五入
    }

    // 运行完整 Demo
    public function testRunNumberDemo(): void
    {
        $this->expectNotToPerformAssertions();
        NumberDemo::numberToString();
        NumberDemo::overflow();
        NumberDemo::typeConversion();
    }
}
