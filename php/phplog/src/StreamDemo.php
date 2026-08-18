<?php

namespace Laixhe\Phplog;

/**
 * 流/迭代器专题：PHP 数组函数与生成器（Generator）。
 * 对应 Rust rustlog/src/iterators.rs 与 Java StreamDemo。
 *
 * 对应关系速查：
 * - map          → array_map / 生成器
 * - filter       → array_filter
 * - filter_map   → array_filter + array_map / array_map 后过滤 null
 * - take(n)      → array_slice(0, n) / 生成器 break
 * - skip(n)      → array_slice(n)
 * - step_by(n)   → range(start, end, step)
 * - enumerate    → foreach 带 key / array_keys
 * - zip          → array_map 多数组 / array_combine
 * - flatten      → 循环 array_merge
 * - fold/reduce  → array_reduce
 * - partition    → 循环分组 / array_filter + array_diff
 * - collect      → array_values / iterator_to_array
 *
 * ⭐ 生成器（yield）是 PHP 的惰性迭代器，与 Rust 迭代器/Java Stream 一样
 *   在消费时才真正计算，适合处理大数据流。
 */
final class StreamDemo
{
    // ============ 基础：数组的三种遍历方式 ============
    public static function basics(): void
    {
        $v = [10, 20, 30];

        // 1) foreach 遍历（最常用）
        echo 'foreach: ';
        foreach ($v as $x) {
            echo $x, ' ';
        }
        echo PHP_EOL;

        // 2) 索引遍历
        for ($i = 0; $i < count($v); $i++) {
            echo "索引 {$i} = {$v[$i]}", PHP_EOL;
        }

        // 3) 带索引遍历（对应 enumerate）
        foreach ($v as $i => $x) {
            echo "enumerate: [{$i}] = '{$x}'", PHP_EOL;
        }

        // 生成器：手动 next()（展示迭代器底层）
        $it = (function () {
            yield 'a';
            yield 'b';
            yield 'c';
        })();
        echo '手动 next: ', $it->current(), ' '; // a
        $it->next();
        echo $it->current(), ' '; // b
        $it->next();
        echo $it->current(), ' '; // c
        $it->next();
        echo '结束? ', var_export(!$it->valid(), true), PHP_EOL; // true
    }

    // ============ 中间操作（惰性：生成器）============
    public static function adapters(): void
    {
        // ---- map：对每个元素做变换 ----
        $squares = array_map(fn ($x) => $x * $x, range(1, 10));
        echo '1..10 平方: ', implode(',', $squares), PHP_EOL;
        // 1,4,9,16,25,36,49,64,81,100

        // ---- filter：只保留满足条件的元素 ----
        $words = ['rust', 'go', 'python', 'java', 'c++', 'js'];
        $shortWords = array_values(array_filter($words, fn ($w) => strlen($w) <= 3));
        echo '长度≤3 的单词: ', implode(',', $shortWords), PHP_EOL; // go,c++,js

        // ---- filter_map 二合一：把能解析为数字的挑出来（对应 Rust filter_map）----
        $strs = ['123', 'abc', '456', 'not_a_num', '789'];
        $nums = [];
        foreach ($strs as $s) {
            $n = filter_var($s, FILTER_VALIDATE_INT);
            if ($n !== false) {
                $nums[] = $n; // 解析成功才保留
            }
        }
        echo 'filter_map 选出合法数字: ', implode(',', $nums), PHP_EOL; // 123,456,789

        // ---- take(n) / skip(n) ----
        echo 'array_slice take(3): ', implode(',', array_slice(range(1, 10), 0, 3)), PHP_EOL; // 1,2,3
        echo 'array_slice skip(7): ', implode(',', array_slice(range(1, 10), 7)), PHP_EOL;    // 8,9,10

        // ---- step_by(n)：每 n 个取一个 ----
        $stepped = range(0, 20, 5);
        echo 'step_by(5) 0..20: ', implode(',', $stepped), PHP_EOL; // 0,5,10,15,20

        // ---- zip：把两个数组一一配对 ----
        $names = ['Alice', 'Bob', 'Charlie'];
        $scores = [95, 87, 92];
        $pairs = array_map(null, $names, $scores); // array_map(null, ...) 实现 zip
        echo 'zip 配对: ', json_encode($pairs, JSON_UNESCAPED_UNICODE), PHP_EOL;
        // [["Alice",95],["Bob",87],["Charlie",92]]

        // ---- chain：把两个数组首尾相接（对应 chain）----
        $chained = array_merge(range(1, 3), range(10, 12));
        echo 'chain: ', implode(',', $chained), PHP_EOL; // 1,2,3,10,11,12

        // ---- flatten：把嵌套数组展平一层（对应 flatten / flat_map）----
        $nested = [[1, 2], [3, 4, 5], [6]];
        $flat = [];
        foreach ($nested as $sub) {
            array_push($flat, ...$sub); // 展平
        }
        echo 'flatMap 展平: ', implode(',', $flat), PHP_EOL; // 1,2,3,4,5,6

        // 把每个单词的字符展开（对应 flat_map chars）
        $chars = str_split(implode('', ['hello', 'world']));
        echo 'flat_map 展开字符: ', implode(',', $chars), PHP_EOL; // h,e,l,l,o,w,o,r,l,d
    }

    // ============ 终结操作（真正触发计算）============
    public static function consumers(): void
    {
        $v = [3, 1, 4, 1, 5, 9, 2, 6];

        // ---- collect：收集到目标数组 ----
        $doubled = array_map(fn ($x) => $x * 2, $v);
        echo 'collect 到数组: ', implode(',', $doubled), PHP_EOL;

        // ---- sum / count / min / max ----
        echo 'sum=', array_sum($v), ' count=', count($v), PHP_EOL; // sum=31 count=8
        echo 'min=', min($v), ' max=', max($v), PHP_EOL;           // min=1 max=9

        // ---- any / all：是否「有一个」/「全部」满足条件 ----
        $anyGt10 = count(array_filter($v, fn ($x) => $x > 10)) > 0;
        $allGt0 = count(array_filter($v, fn ($x) => $x <= 0)) === 0;
        echo 'any > 10? ', var_export($anyGt10, true), '  all > 0? ', var_export($allGt0, true), PHP_EOL;
        // false true

        // ---- forEach：对每个元素执行副作用 ----
        echo 'forEach: ';
        array_walk($v, fn ($x) => print($x . ' '));
        echo PHP_EOL;

        // ---- reduce：累积聚合（对应 fold / reduce）----
        $foldSum = array_reduce(range(1, 10), fn ($acc, $x) => $acc + $x, 0);
        echo 'array_reduce 累加 1..10 = ', $foldSum, PHP_EOL; // 55

        // ---- partition：按条件分成两组 ----
        $even = [];
        $odd = [];
        foreach ($v as $x) {
            if ($x % 2 === 0) {
                $even[] = $x;
            } else {
                $odd[] = $x;
            }
        }
        echo 'partition 奇偶分: 偶=[', implode(',', $even), ']  奇=[', implode(',', $odd), ']', PHP_EOL;
    }

    // ============ 综合实战：用数组函数处理复杂查询 ============
    // 场景：给定一批员工（部门，年龄，月薪），求出「R&D 部门 30 岁以上员工的平均月薪」。
    public static function practice(): void
    {
        $staff = [
            ['dept' => 'R&D', 'age' => 28, 'salary' => 30000],
            ['dept' => 'R&D', 'age' => 35, 'salary' => 45000],
            ['dept' => 'R&D', 'age' => 42, 'salary' => 60000],
            ['dept' => 'HR', 'age' => 32, 'salary' => 18000],
            ['dept' => 'R&D', 'age' => 25, 'salary' => 22000],
            ['dept' => 'Sale', 'age' => 38, 'salary' => 25000],
        ];

        // 要求：R&D 部门 + 30 岁以上 → 平均月薪
        $filtered = array_filter(
            $staff,
            fn ($e) => $e['dept'] === 'R&D' && $e['age'] >= 30 // 先筛选部门，再筛选年龄
        );
        $salaries = array_column($filtered, 'salary'); // 提取月薪
        $avg = count($salaries) === 0 ? 0 : array_sum($salaries) / count($salaries);
        printf("R&D 30+ 员工平均月薪: %.0f 元/月" . PHP_EOL, $avg); // (45000+60000)/2 = 52500
    }
}
