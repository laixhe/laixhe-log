<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\StreamDemo;
use PHPUnit\Framework\TestCase;

/**
 * 流/数组函数测试（对应 Rust iterators.rs 的 #[cfg(test)] 练习题）。
 */
final class StreamTest extends TestCase
{
    // 练习 1：偶数平方后求和（1..5 → 2^2 + 4^2 = 20）
    public function testExercise1EvenSquaresSum(): void
    {
        $squares = array_map(fn ($x) => $x * $x, array_filter(
            range(1, 5),
            fn ($x) => $x % 2 === 0
        ));
        $this->assertSame(20, array_sum($squares));
    }

    // 练习 2：enumerate 与 zip 的区别
    public function testExercise2EnumerateZip(): void
    {
        // enumerate：索引固定从 0 开始
        $e = [];
        foreach (['a', 'b', 'c'] as $i => $ch) {
            $e[] = "{$i}:'{$ch}'";
        }
        $this->assertSame(["0:'a'", "1:'b'", "2:'c'"], $e);

        // zip：可以和任意序列配对，这里是 100 开头的序列
        $z = array_map(
            fn ($i, $ch) => ($i + 100) . ":'{$ch}'",
            array_keys(['a', 'b', 'c']),
            ['a', 'b', 'c']
        );
        $this->assertSame(["100:'a'", "101:'b'", "102:'c'"], $z);
    }

    // 练习 3：flatMap 把每个单词展开成字符
    public function testExercise3FlatMapChars(): void
    {
        $chars = str_split(implode('', ['hello', 'world']));
        $this->assertSame(['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'], $chars);
    }

    // 练习 4：综合实战 —— R&D 部门 30 岁以上员工平均月薪
    public function testExercise4Practice(): void
    {
        $staff = [
            ['dept' => 'R&D', 'age' => 28, 'salary' => 30000],
            ['dept' => 'R&D', 'age' => 35, 'salary' => 45000],
            ['dept' => 'R&D', 'age' => 42, 'salary' => 60000],
            ['dept' => 'HR', 'age' => 32, 'salary' => 18000],
            ['dept' => 'R&D', 'age' => 25, 'salary' => 22000],
            ['dept' => 'Sale', 'age' => 38, 'salary' => 25000],
        ];

        $filtered = array_filter(
            $staff,
            fn ($e) => $e['dept'] === 'R&D' && $e['age'] >= 30
        );
        $salaries = array_column($filtered, 'salary');
        $avg = count($salaries) === 0 ? 0 : array_sum($salaries) / count($salaries);
        $this->assertEquals(52500.0, $avg); // (45000+60000)/2 = 52500
    }

    // 练习 5：partition 奇偶分组
    public function testExercise5Partition(): void
    {
        $v = [3, 1, 4, 1, 5, 9, 2, 6];
        $even = [];
        $odd = [];
        foreach ($v as $x) {
            if ($x % 2 === 0) {
                $even[] = $x;
            } else {
                $odd[] = $x;
            }
        }
        $this->assertSame([4, 2, 6], $even);
        $this->assertSame([3, 1, 1, 5, 9], $odd);
    }

    // 运行完整 Demo
    public function testRunStreamDemo(): void
    {
        $this->expectNotToPerformAssertions();
        StreamDemo::basics();
        StreamDemo::adapters();
        StreamDemo::consumers();
        StreamDemo::practice();
    }
}
