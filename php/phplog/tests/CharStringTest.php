<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\CharStringDemo;
use PHPUnit\Framework\TestCase;

/**
 * 字符与字符串测试（对应 Rust char_string.rs 的 #[cfg(test)] 练习题）。
 */
final class CharStringTest extends TestCase
{
    // 练习 1：trim + 替换逗号为空格 + 按空格切词
    public function testExercise1TrimReplaceSplit(): void
    {
        $replaced = str_replace(',', ' ', trim('  Hello,Rust!  '));
        $this->assertSame(['Hello', 'Rust!'], preg_split('/\s+/', $replaced));
    }

    // 练习 2：判断是否为中文（CJK 统一汉字区间 U+4E00 ~ U+9FFF）
    public function testExercise2IsChineseChar(): void
    {
        $codePoint = mb_ord('中');
        $this->assertTrue($codePoint >= 0x4E00 && $codePoint <= 0x9FFF);
        $this->assertFalse(mb_ord('A') >= 0x4E00 && mb_ord('A') <= 0x9FFF);
    }

    // 练习 3：sprintf 拼接（不改变原字符串）
    public function testExercise3FormatBorrows(): void
    {
        $slice = 'Hello';
        $owned = ' Rust';
        $combined = sprintf('%s%s', $slice, $owned);
        $this->assertSame('Hello Rust', $combined);
        $this->assertSame('Hello', $slice); // 原字符串不变
    }

    // 练习 4：length / 字符数 / 字节数的区别
    public function testExercise4LengthBytesChars(): void
    {
        $text = '你好😀';
        $this->assertSame(10, strlen($text));          // UTF-8 字节数
        $this->assertSame(3, mb_strlen($text, 'UTF-8')); // 字符数
    }

    // 练习 5：字符串解析
    public function testExercise5StringParse(): void
    {
        $this->assertSame(666, (int) '666');
        $this->assertSame(88.88, (float) '88.88');
        $this->assertFalse(filter_var('not_a_number', FILTER_VALIDATE_INT));
    }

    // 运行完整 Demo
    public function testRunCharStringDemo(): void
    {
        $this->expectNotToPerformAssertions();
        CharStringDemo::stdChar();
        CharStringDemo::stdString();
        CharStringDemo::stringParse();
    }
}
