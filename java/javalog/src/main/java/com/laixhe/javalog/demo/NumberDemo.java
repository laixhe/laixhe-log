package com.laixhe.javalog.demo;

/**
 * 数值类型示例：格式化输出、整数溢出安全处理、类型转换。
 * 对应 Rust rustlog/src/number.rs 与 Go golog 中的数值示例。
 *
 * 前置知识：
 * - Java 整数类型：byte(8) short(16) int(32) long(64)，均为有符号
 * - 浮点数：float(32) double(64，默认推荐)
 * - 溢出默认行为：Java 中整数溢出会「静默回绕」，
 *   推荐使用 Math.addExact / Math.multiplyExact 等抛出 ArithmeticException，
 *   或使用 Math.clamp / Math.min / Math.max 实现饱和运算。
 */
public final class NumberDemo {

    private NumberDemo() {
    }

    // ============ 数值转字符串与格式化输出 ============
    public static void numberToString() {
        int i = 666;
        double f1 = 88.888;
        double f2 = 88.0;

        // 基础转字符串
        System.out.println("i=" + i); // 结果：i=666

        // 精度控制（四舍五入）
        System.out.println("f1=" + String.format("%.2f", f1)); // 结果 f1=88.89
        System.out.println("f2=" + String.format("%.2f", f2)); // 结果 f2=88.00

        // ===== 更多格式化方式（新手学习重点）=====

        // 十六进制 / 八进制 / 二进制
        System.out.printf("666 hex=0x%X  octal=0o%o  binary=0b%s%n", i, i, Integer.toBinaryString(i));
        // 结果：666 hex=0x29A  octal=0o1232  binary=0b1010011010

        // 前导零填充 + 宽度控制：%08d 表示「右对齐，总宽度 8，不足补 0」
        System.out.printf("666 with leading zeros: %08d%n", i);
        // 结果：666 with leading zeros: 00000666

        // 对齐：%-10d 左对齐，%10d 右对齐（默认）；居中需要手动补空格
        System.out.printf("left=|%-10d| center=|%6d%4s| right=|%10d|%n", i, i, "", i);
        // 结果：left=|666       | center=|   666    | right=|       666|
        System.out.printf("left=|%-10d| right=|%10d|%n", i, i);

        // 正负号显式显示
        System.out.printf("positive= %+d  negative= %+d%n", 666, -888);
        // 结果：positive= +666  negative= -888

        // 千分位分组
        System.out.printf("grouping: %,d%n", 1_234_567); // 结果：1,234,567
    }

    // ============ 整数溢出安全处理（三种模式对比）============
    // Java 默认整数溢出会「静默回绕」，业务代码应显式选择处理方式
    public static void overflow() {
        int x = Integer.MAX_VALUE;

        // 1) addExact：溢出抛 ArithmeticException，最安全，推荐默认使用
        try {
            int r = Math.addExact(x, 1);
            System.out.println("addExact: MAX+1 = " + r);
        } catch (ArithmeticException e) {
            System.out.println("addExact: MAX+1 = 溢出了（抛出 ArithmeticException）");
        }

        // 2) saturating：饱和运算，溢出时取类型最大值
        //    Java 没有内置 saturating_add，可用 Math.clamp（Java 21+）模拟：
        //    先拓宽到 long 计算避免溢出，再 clamp 回 int 范围
        long wide = (long) x + 1; // 拓宽到 long，不会溢出
        int sat = Math.clamp(wide, Integer.MIN_VALUE, Integer.MAX_VALUE);
        System.out.println("saturating_add: MAX+1 = " + sat + "（饱和，卡在 Integer.MAX_VALUE）"); // 2147483647

        // 3) wrapping：回绕运算（Java 默认行为），适合哈希/CRC 等算法
        int wrap = x + 1;
        System.out.println("wrapping_add: MAX+1 = " + wrap + "（回绕到最小值）"); // -2147483648

        // 4) byte 的显式回绕：127 + 1 = -128
        byte b = Byte.MAX_VALUE;
        byte wrapB = (byte) (b + 1);
        System.out.println("byte wrapping: 127+1 = " + wrapB); // -128

        // ===== 其他常用精确运算方法 =====
        // Math.incrementExact / Math.decrementExact / Math.subtractExact /
        // Math.multiplyExact / Math.negateExact / Math.toIntExact
        try {
            System.out.println("multiplyExact: 100000*100000 = " + Math.multiplyExact(100_000, 100_000));
        } catch (ArithmeticException e) {
            System.out.println("multiplyExact: 100000*100000 = 溢出了（10^10 超出 int 范围）");
        }
    }

    // ============ 数值类型转换 ============
    // 三种转换方式，安全性从高到低：
    // 1) 隐式拓宽（自动）：int -> long -> double，无损失，编译期通过
    // 2) 显式检查转换：Math.toIntExact / 先判断范围，溢出会报错
    // 3) 强制窄化（cast）：可能截断或丢失精度，慎用
    public static void typeConversion() {
        // --- 1. 隐式拓宽（无损失）---
        int small = 10;
        long big = small; // int -> long 自动拓宽，一定安全
        System.out.println("int->long: " + small + " -> " + big); // 10 -> 10

        double d = big; // long -> double 自动拓宽（大数可能丢精度）
        System.out.println("long->double: " + big + " -> " + d);

        // --- 2. 显式检查转换（可能失败，会抛异常）---
        long tooBig = 1000L;
        try {
            int v = Math.toIntExact(tooBig); // long -> int，溢出抛 ArithmeticException
            System.out.println("long->int 成功: " + v);
        } catch (ArithmeticException e) {
            System.out.println("long->int 失败: 1000 -> " + e.getMessage());
        }

        try {
            long overflow = 3_000_000_000L; // 超过 int 范围
            int v = Math.toIntExact(overflow);
            System.out.println("long->int 成功: " + v);
        } catch (ArithmeticException e) {
            System.out.println("long->int 失败: 3000000000 -> 溢出异常：" + e.getClass().getSimpleName());
        }

        // --- 3. as 强制窄化（截断，需慎用）---
        // Java 中 int -> byte 只会保留最低字节
        int a = 0x1234ABCD;
        byte b = (byte) a; // 只保留最低字节 0xCD
        System.out.printf("int as byte 截断: 0x%08X -> 0x%02X%n", a, b & 0xFF); // 0x1234ABCD -> 0xCD

        // 浮点数转整数会向零截断
        double pi = 3.99;
        int truncated = (int) pi;
        System.out.println("double as int 向零截断: " + pi + " -> " + truncated); // 3

        // 如果需要四舍五入，用 Math.round() 再转
        int rounded = (int) Math.round(pi);
        System.out.println("Math.round() 后转: " + pi + " -> " + rounded); // 4

        // 字符串解析（对应 Rust string_parse）
        System.out.println("parseInt('666') = " + Integer.parseInt("666")); // 666
        System.out.println("parseDouble('88.88') = " + Double.parseDouble("88.88")); // 88.88
        try {
            Integer.parseInt("not_a_number");
        } catch (NumberFormatException e) {
            System.out.println("解析失败演示: 'not_a_number' -> NumberFormatException");
        }
    }
}
