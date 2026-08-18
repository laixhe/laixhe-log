<?php

namespace Laixhe\Phplog;

/**
 * 字符与字符串示例：char（单字符）、字符串常用操作、多字节编码。
 * 对应 Rust rustlog/src/char_string.rs 与 Go golog/string_test.go。
 *
 * 前置知识：
 * - PHP 字符串本质是字节数组，一个中文字符在 UTF-8 下占 3 个字节
 * - strlen() 返回「字节数」（对应 Go len()）；mb_strlen() 返回「字符数」（对应 rune 数）
 * - 操作多字节字符串（中文/Emoji）必须使用 mb_* 系列函数（mbstring 扩展）
 */
final class CharStringDemo
{
    // ============ 字符类型 ============
    public static function stdChar(): void
    {
        $c1 = 'A';   // 英文字母
        $c2 = '中';  // 中文字符
        $c3 = '😀';  // 表情符号（Emoji，UTF-8 占 4 字节）
        $c4 = '1';   // 数字字符
        $c5 = '!';   // 标点符号

        echo "c1 = {$c1} c2 = {$c2} c3 = {$c3} c4 = {$c4} c5 = {$c5}", PHP_EOL;

        // 字符的常用判断方法（PHP 没有 char 类型，用单字符字符串表示）
        var_dump(ctype_alpha($c1));       // 'A' 是字母？true
        var_dump(ctype_digit($c4));       // '1' 是数字？true
        var_dump(ctype_space(' '));       // ' ' 是空白？true
        var_dump(strtolower('A'));        // 'A' 转小写：a
        var_dump(strtoupper('a'));        // 'a' 转大写：A

        // Unicode 码点（对应 Rust char / Java codePoint）
        var_dump(mb_ord($c2)); // '中' 的码点：20013 (U+4E2D)
        var_dump(mb_ord($c3)); // '😀' 的码点：128512 (U+1F600)
        echo '码点 20013 转字符：', mb_chr(20013), PHP_EOL; // 中
    }

    // ============ String 常用操作 ============
    public static function stdString(): void
    {
        // ---------- 长度：字节 / 字符 ----------
        $text = '你好😀';
        echo 'strlen(字节) = ', strlen($text), PHP_EOL;            // 10（中 3+3 + Emoji 4）
        echo 'mb_strlen(字符) = ', mb_strlen($text), PHP_EOL;      // 3
        echo 'mb_strlen(utf8) = ', mb_strlen($text, 'UTF-8'), PHP_EOL; // 3

        // 遍历每个字符（对应 Rust chars()）
        echo '遍历字符: ';
        $chars = mb_str_split($text);
        foreach ($chars as $ch) {
            echo $ch, ' ';
        }
        echo PHP_EOL;

        // ---------- 拼接（对应 String::push / push_str / format!）----------
        $s3 = 'hello';
        $s3 .= ' ';       // 追加一个字符
        $s3 .= 'world';   // 追加字符串
        echo 's3 = ', $s3, PHP_EOL; // hello world

        $s4 = sprintf('%s...', $s3); // 格式化拼接，不改变原字符串
        echo 's4 = ', $s4, PHP_EOL;
        echo 's3 still alive: ', $s3, PHP_EOL;

        $joined = implode(',', ['a', 'b', 'c']); // 对应 Go strings.Join
        echo 'implode = ', $joined, PHP_EOL;     // a,b,c

        // ---------- 常用方法（对应 Go strings 包 / Rust String）----------
        $s = '   Hello, Rust! I love Rust.   ';
        var_dump(str_contains($s, 'Rust'));          // true（PHP 8+）
        var_dump(str_starts_with($s, '   He'));      // true
        var_dump(str_ends_with($s, '.   '));         // true
        echo 'trim = |', trim($s), '|', PHP_EOL;     // 去首尾空白
        echo 'str_replace = ', str_replace('Rust', '🦀 Rust', $s), PHP_EOL;
        // Hello, 🦀 Rust! I love 🦀 Rust.

        $fruits = explode(',', 'apple,banana,cherry,date');
        echo 'explode = ', implode(' ', $fruits), PHP_EOL; // apple banana cherry date

        // 按任意空白切分（对应 Go strings.Fields）
        $fields = preg_split('/\s+/', '1 2	3 	4');
        echo 'preg_split(空白) = ', implode(',', $fields), PHP_EOL; // 1,2,3,4

        // 查找位置（对应 Go strings.Index / LastIndex，false 表示没找到）
        var_dump(strpos('查找到第一次出现的位置', '出现'));   // 6（字节偏移，中文每个 3 字节）
        var_dump(mb_strpos('查找到第一次出现的位置', '出现')); // 2（字符偏移）
        var_dump(strrpos('查找到最后出现的位置', '出现'));     // 15（字节偏移）
        var_dump(strpos('abc', 'z'));                         // false

        // 大小写 / 相等
        var_dump(strcasecmp('AB大', 'ab大') === 0); // 忽略大小写比较 true
        echo 'strtoupper(多字节) = ', mb_strtoupper('rust中文'), PHP_EOL; // RUST中文

        // ---------- 单词频率统计（对应 Rust entry().or_insert(0) += 1）----------
        $words = preg_split('/\s+/', 'rust go rust php rust go python js');
        $counts = array_count_values($words); // 一行统计词频
        ksort($counts);
        echo '单词出现次数统计: ', json_encode($counts, JSON_UNESCAPED_UNICODE), PHP_EOL;
        // {"go":2,"js":1,"php":1,"python":1,"rust":3}
    }

    // ============ 字符串解析为数值类型 ============
    public static function stringParse(): void
    {
        // 转整数
        $i = (int) '666';
        echo "i={$i}", PHP_EOL; // 666

        // 转浮点
        $f = (float) '88.88';
        echo "f={$f}", PHP_EOL; // 88.88

        // 进制解析
        var_dump(hexdec('29A'));     // 666（十六进制字符串转十进制）
        var_dump(dechex(666));       // '29a'
        var_dump(octdec('1232'));    // 666
        var_dump(bindec('1010011010')); // 666

        // 严格校验（避免 PHP 宽松转换的坑）
        var_dump(filter_var('666', FILTER_VALIDATE_INT));        // 666
        var_dump(filter_var('88.88', FILTER_VALIDATE_FLOAT));    // 88.88
        var_dump(filter_var('not_a_number', FILTER_VALIDATE_INT)); // false
    }
}
