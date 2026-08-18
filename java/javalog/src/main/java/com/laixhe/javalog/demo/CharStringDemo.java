package com.laixhe.javalog.demo;

import java.util.LinkedHashMap;
import java.util.Map;
import java.util.StringJoiner;

/**
 * 字符与字符串示例：char、String 的常用操作、UTF-16 与 Unicode 码点的区别。
 * 对应 Rust rustlog/src/char_string.rs 与 Go golog/string_test.go。
 *
 * 前置知识：
 * - Java 的 char 是 16 位 UTF-16 码元（不是 Unicode 码点！）。
 *   「中」等 BMP 字符占 1 个 char，「😀」等增补平面字符占 2 个 char（代理对）。
 * - String.length() 返回 UTF-16 码元个数；codePointCount() 返回码点个数；
 *   getBytes(UTF_8).length 返回 UTF-8 字节数（对应 Go len() / utf8.RuneCountInString）。
 */
public final class CharStringDemo {

    private CharStringDemo() {
    }

    // ============ 字符类型 char ============
    public static void stdChar() {
        char c1 = 'A';    // 英文字母
        char c2 = '中';   // 中文字符
        // 😀 表情符号位于增补平面（U+1F600），占用 2 个 UTF-16 码元，
        // Java 的 char 是 16 位无法直接表示，用码点 int 表示
        int c3 = 0x1F600; // 😀 Emoji 的 Unicode 码点
        char c4 = '1';    // 数字字符
        char c5 = '!';    // 标点符号

        System.out.println("c1 = " + c1 + " c2 = " + c2 + " c3 = "
                + new String(Character.toChars(c3)) + " c4 = " + c4 + " c5 = " + c5);

        // char 的常用判断方法
        System.out.println("'A' 是字母？" + Character.isLetter(c1));   // true
        System.out.println("'1' 是数字？" + Character.isDigit(c4));    // true
        System.out.println("' ' 是空白？" + Character.isWhitespace(' ')); // true
        System.out.println("'A' 转小写：" + Character.toLowerCase(c1)); // a
        System.out.println("'a' 转大写：" + Character.toUpperCase('a')); // A

        // 判断中文字符：CJK 统一汉字区间 U+4E00 ~ U+9FFF（对应 Rust matches! 练习）
        char han = '中';
        System.out.println("'中' 是汉字？" + (han >= 0x4E00 && han <= 0x9FFF)); // true
        System.out.println("'A' 是汉字？" + (c1 >= 0x4E00 && c1 <= 0x9FFF));    // false
    }

    // ============ String 常用操作 ============
    public static void stdString() {
        // ---------- 创建 String 的多种方式 ----------
        // 1. 字面量（对应 &str，不可变）
        String s1 = "hello";
        System.out.println("s1 = " + s1); // hello

        // 2. 字符数组（对应 from_iter(['h','e','l','l','o'])）
        String s2 = new String(new char[]{'h', 'e', 'l', 'l', 'o'});
        System.out.println("s2 = " + s2); // hello

        // 3. 拼接（对应 String::from + push + push_str）
        StringBuilder sb = new StringBuilder("hello");
        sb.append(' ');       // 追加一个字符
        sb.append("world");   // 追加字符串
        System.out.println("s3 = " + sb); // hello world

        // 4. 格式化拼接（对应 format!，不改变原字符串）
        String s4 = String.format("%s...", sb);
        System.out.println("s4 = " + s4); // hello world...
        System.out.println("sb still alive: " + sb);

        // 5. StringJoiner / String.join（对应 join）
        String joined = String.join(",", "a", "b", "c");
        System.out.println("String.join = " + joined); // a,b,c
        StringJoiner sj = new StringJoiner("-", "[", "]");
        sj.add("x").add("y");
        System.out.println("StringJoiner = " + sj); // [x-y]

        // ---------- 长度：字节 / 码元 / 码点 ----------
        String text = "你好😀";
        System.out.println("len(UTF-16码元)=" + text.length());                 // 4（你、好、😀 的代理对）
        System.out.println("codePointCount(码点)=" + text.codePointCount(0, text.length())); // 3
        System.out.println("getBytes(UTF-8字节)=" + text.getBytes(java.nio.charset.StandardCharsets.UTF_8).length); // 10

        // 遍历每个码点（对应 Rust chars()）
        System.out.print("遍历码点: ");
        text.codePoints().forEach(cp -> System.out.print(new String(Character.toChars(cp)) + " "));
        System.out.println();

        // ---------- 常用方法（对应 Go strings 包 / Rust String）----------
        String s = "   Hello, Rust! I love Rust.   ";
        System.out.println("包含 'Rust'？" + s.contains("Rust"));            // true
        System.out.println("以 '   He' 开头？" + s.startsWith("   He"));     // true
        System.out.println("以 '.   ' 结尾？" + s.endsWith(".   "));        // true
        System.out.println("trim()      = |" + s.trim() + "|");             // 去首尾空白
        System.out.println("strip()     = |" + s.strip() + "|");            // 去首尾空白（含 Unicode）
        String replaced = s.replace("Rust", "🦀 Rust");
        System.out.println("replace: " + replaced);                          // Hello, 🦀 Rust! I love 🦀 Rust.
        String[] fruits = "apple,banana,cherry,date".split(",");
        System.out.println("split: " + java.util.Arrays.toString(fruits));  // [apple, banana, cherry, date]

        // 对应 Go strings.Fields（按任意空白切分）
        String[] fields = "1 2\t3 \t4".split("\\s+");
        System.out.println("split(空白): " + java.util.Arrays.toString(fields)); // [1, 2, 3, 4]

        // 查找位置（对应 Index / LastIndex，-1 表示没找到）
        System.out.println("indexOf('出现'): " + "查找到第一次出现的位置".indexOf("出现"));     // 6
        System.out.println("lastIndexOf('出现'): " + "查找到最后出现的位置".lastIndexOf("出现")); // 6
        System.out.println("indexOf 未找到: " + "abc".indexOf("z"));                          // -1

        // 大小写 / 相等
        System.out.println("equalsIgnoreCase: " + "AB大".equalsIgnoreCase("ab大")); // true
        System.out.println("toUpperCase: " + "rust".toUpperCase());                 // RUST

        // 首尾截取
        System.out.println("substring(0,2): " + "查找到第一次出现的位置".substring(0, 2)); // 查找

        // ---------- 单词频率统计（综合实战：merge + 流）----------
        Map<String, Integer> counts = new LinkedHashMap<>();
        for (String word : "rust go rust php rust go python js".split("\\s+")) {
            // merge：key 不存在则插入 1，存在则旧值 + 1（对应 entry().or_insert(0) += 1）
            counts.merge(word, 1, Integer::sum);
        }
        System.out.println("单词出现次数统计: " + counts);
        // 结果：{rust=3, go=2, php=1, python=1, js=1}
    }

    // ============ 字符串解析为数值类型 ============
    public static void stringParse() {
        // 转整数（对应 parse::<i32>）
        int i = Integer.parseInt("666");
        System.out.println("i=" + i); // 666

        // 转浮点
        double f = Double.parseDouble("88.88");
        System.out.println("f=" + f); // 88.88

        // 解析失败演示
        try {
            Integer.parseInt("not_a_number");
        } catch (NumberFormatException e) {
            System.out.println("解析失败演示: 'not_a_number' → NumberFormatException");
        }

        // 进制解析：radix 指定进制
        System.out.println("parseInt(0x29A, 16) = " + Integer.parseInt("29A", 16)); // 666
        System.out.println("toHexString(666) = " + Integer.toHexString(666));        // 29a
    }
}
