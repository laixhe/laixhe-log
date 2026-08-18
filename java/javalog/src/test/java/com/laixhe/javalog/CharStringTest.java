package com.laixhe.javalog;

import com.laixhe.javalog.demo.CharStringDemo;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;

import static org.junit.jupiter.api.Assertions.*;

/**
 * 字符与字符串测试（对应 Rust char_string.rs 的 #[cfg(test)] 练习题）。
 */
class CharStringTest {

    // 练习 1：trim + 替换逗号为空格 + 按空格切词
    @Test
    void exercise1_trim_replace_split() {
        String replaced = "  Hello,Rust!  ".trim().replace(",", " ");
        assertArrayEquals(new String[]{"Hello", "Rust!"}, replaced.split("\\s+"));
    }

    // 练习 2：判断是否为中文（CJK 统一汉字区间 U+4E00 ~ U+9FFF）
    @Test
    void exercise2_is_chinese_char() {
        char han = '中';
        assertTrue(han >= 0x4E00 && han <= 0x9FFF);
        assertFalse('A' >= 0x4E00 && 'A' <= 0x9FFF);
    }

    // 练习 3：format! 拼接 &str 和 String（不转移所有权）
    @Test
    void exercise3_format_borrows() {
        String slice = "Hello";
        String owned = " Rust";
        String combined = String.format("%s%s", slice, owned);
        assertEquals("Hello Rust", combined);
        // format 只借用，owned 仍可用（Java String 不可变，天然无所有权转移问题）
        assertEquals(" Rust", owned);
    }

    // 练习 4：length / codePointCount / 字节数 的区别
    @Test
    void exercise4_length_bytes_codepoints() {
        String text = "你好😀";
        assertEquals(4, text.length());                            // UTF-16 码元数
        assertEquals(3, text.codePointCount(0, text.length()));    // Unicode 码点数
        assertEquals(10, text.getBytes(StandardCharsets.UTF_8).length); // UTF-8 字节数
    }

    // 练习 5：字符串解析
    @Test
    void exercise5_string_parse() {
        assertEquals(666, Integer.parseInt("666"));
        assertEquals(88.88, Double.parseDouble("88.88"));
        assertThrows(NumberFormatException.class, () -> Integer.parseInt("not_a_number"));
    }

    // 运行完整 Demo
    @Test
    void runCharStringDemo() {
        CharStringDemo.stdChar();
        CharStringDemo.stdString();
        CharStringDemo.stringParse();
    }
}
