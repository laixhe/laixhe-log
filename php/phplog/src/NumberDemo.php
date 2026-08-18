<?php

namespace Laixhe\Phplog;

/**
 * 数值类型示例：格式化输出、整数溢出行为、类型转换。
 * 对应 Rust rustlog/src/number.rs 与 Go golog 的数值示例。
 *
 * 前置知识：
 * - PHP 整数是 64 位有符号（PHP_INT_MAX = 9223372036854775807）
 * - PHP 整数溢出时不会抛异常，而是「静默提升为 float」（丢失精度），
 *   这是与 Java Math.addExact 最大的不同，业务上需要自行检查
 * - 需要任意精度时使用 BCMath / GMP 扩展
 */
final class NumberDemo
{
    // ============ 数值转字符串与格式化输出 ============
    public static function numberToString(): void
    {
        $i = 666;
        $f1 = 88.888;
        $f2 = 88.0;

        // 基础转字符串（对应 i.to_string()）
        echo "i={$i}", PHP_EOL; // 结果：i=666

        // 精度控制（四舍五入）
        printf("f1=%.2f" . PHP_EOL, $f1); // 结果 f1=88.89
        printf("f2=%.2f" . PHP_EOL, $f2); // 结果 f2=88.00

        // ===== 更多格式化方式（新手学习重点）=====

        // 十六进制 / 八进制 / 二进制
        printf("666 hex=0x%X  octal=0o%o  binary=0b%s" . PHP_EOL, $i, $i, decbin($i));
        // 结果：666 hex=0x29A  octal=0o1232  binary=0b1010011010

        // 前导零填充 + 宽度控制：%08d 表示「右对齐，总宽度 8，不足补 0」
        printf("666 with leading zeros: %08d" . PHP_EOL, $i);
        // 结果：666 with leading zeros: 00000666

        // 对齐：%-10d 左对齐，%10d 右对齐（默认）
        printf("left=|%-10d| right=|%10d|" . PHP_EOL, $i, $i);
        // 结果：left=|666       | right=|       666|

        // 正负号显式显示
        printf("positive= %+d  negative= %+d" . PHP_EOL, 666, -888);
        // 结果：positive= +666  negative= -888

        // 千分位分组
        echo 'grouping: ', number_format(1_234_567), PHP_EOL; // 1,234,567
    }

    // ============ 整数溢出行为（与 Java/Rust 对比）============
    // PHP 与 Go 类似：整数溢出「静默回绕/提升」，需要自己检查边界
    public static function overflow(): void
    {
        $x = PHP_INT_MAX;

        // 1) PHP 整数溢出：提升为 float，不抛异常（区别于 Java Math.addExact）
        $r = $x + 1;
        echo 'PHP_INT_MAX + 1 = ', var_export($r, true), '（溢出提升为 float）', PHP_EOL;

        // 2) 检查是否溢出：先把结果与最大值比较
        //    推荐：计算前判断「加上这个数是否会超过上限」
        $add = 1;
        if ($add > PHP_INT_MAX - $x) {
            echo "addExact 模拟: MAX + 1 = 溢出了（超出 int 范围）", PHP_EOL;
        } else {
            echo 'addExact 模拟: MAX + 1 = ', $x + $add, PHP_EOL;
        }

        // 3) saturating：饱和运算，溢出时取类型最大值（对应 Rust saturating_add）
        $sat = min($x + 1, PHP_INT_MAX);
        echo 'saturating_add: MAX + 1 = ', $sat, '（饱和，卡在 PHP_INT_MAX）', PHP_EOL;

        // 4) 小整数溢出：8 位无符号 255 + 1 = 256（PHP 无 u8，直接正常计算）
        $b = 255;
        echo '255 + 1 = ', $b + 1, '（PHP 整数默认 64 位，不回绕）', PHP_EOL;

        // 5) 除零会抛 DivisionByZeroError（PHP 8+，区别于早期版本的 INF/NAN）
        try {
            $z = 1 / 0;
        } catch (\DivisionByZeroError $e) {
            echo '1 / 0 = 抛 DivisionByZeroError（区别于 IEEE754 的 INF）', PHP_EOL;
        }
        // PHP 8 起浮点除以 0.0 同样抛异常（不再是 INF）
        try {
            $f = 1 / 0.0;
        } catch (\DivisionByZeroError $e) {
            echo '1 / 0.0 = 同样抛 DivisionByZeroError（PHP 8 起任何除零都抛异常）', PHP_EOL;
        }
    }

    // ============ 数值类型转换 ============
    // PHP 是弱类型语言，类型转换非常宽松，需要理解隐式转换规则
    public static function typeConversion(): void
    {
        // --- 1. 隐式转换（宽松）：int + string 自动转数值 ---
        $sum = 10 + '5';
        var_dump($sum); // int(15)

        // --- 2. 强制转换（cast）---
        $small = 10;
        $big = (float) $small; // int -> float
        var_dump($big);        // float(10.0)

        $pi = 3.99;
        $truncated = (int) $pi; // 浮点转整数：向零截断
        var_dump($truncated);   // int(3)

        $rounded = (int) round($pi); // 四舍五入后再转
        var_dump($rounded);          // int(4)

        // 字符串转数值
        $i = (int) '666';
        var_dump($i); // int(666)

        // --- 3. 类型判断与严格比较 ---
        var_dump(is_int(666));    // true
        var_dump(is_float(88.8)); // true
        // PHP 8 之前 == 是宽松比较，PHP 8 之后建议使用 ===
        var_dump(1 == '1');  // true（宽松比较，自动转换）
        var_dump(1 === '1'); // false（严格比较，类型也相同才相等）

        // --- 4. 字符串解析（对应 Rust string_parse）---
        $i2 = intval('666');
        echo "intval('666') = {$i2}", PHP_EOL; // 666
        $f2 = floatval('88.88');
        echo "floatval('88.88') = {$f2}", PHP_EOL; // 88.88
        // 解析失败：PHP 不会抛异常，而是返回 0（注意！）
        $bad = intval('not_a_number');
        echo "intval('not_a_number') = {$bad}（解析失败返回 0，不抛异常）", PHP_EOL;
        // 需要严格校验时用 filter_var
        $checked = filter_var('666', FILTER_VALIDATE_INT);
        var_dump($checked); // int(666)
        var_dump(filter_var('not_a_number', FILTER_VALIDATE_INT)); // false
    }
}
