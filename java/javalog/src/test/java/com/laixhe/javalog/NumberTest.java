package com.laixhe.javalog;

import com.laixhe.javalog.demo.NumberDemo;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

/**
 * 数值类型测试（对应 Rust number.rs 的 #[cfg(test)] 练习题）。
 */
class NumberTest {

    // 练习 1：格式化输出 666 的 8 位十六进制（前面补零）
    @Test
    void exercise1_hex_leading_zeros() {
        assertEquals("0000029A", String.format("%08X", 666));
    }

    // 练习 2：精确乘法溢出检测（对应 saturating_mul：MAX * 3 不溢出/饱和）
    @Test
    void exercise2_saturating_mul() {
        long result = Math.min((long) Integer.MAX_VALUE * 3, Integer.MAX_VALUE);
        assertEquals(Integer.MAX_VALUE, result);
    }

    // 练习 3：类型转换溢出检测（对应 TryFrom u32::MAX -> u8 返回 Err）
    @Test
    void exercise3_to_int_exact_overflow() {
        assertThrows(ArithmeticException.class, () -> Math.toIntExact(3_000_000_000L));
        assertEquals(200, Math.toIntExact(200L));
    }

    // 练习 4：Math.addExact 溢出抛异常（对应 checked_add）
    @Test
    void exercise4_checked_add() {
        assertThrows(ArithmeticException.class, () -> Math.addExact(Integer.MAX_VALUE, 1));
        assertEquals(300, Math.addExact(100, 200));
    }

    // 运行完整 Demo（对应 cargo run 输出）
    @Test
    void runNumberDemo() {
        NumberDemo.numberToString();
        NumberDemo.overflow();
        NumberDemo.typeConversion();
    }
}
