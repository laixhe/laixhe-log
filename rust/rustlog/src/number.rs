//! 数值类型示例：整数溢出的多种安全处理方式、数值类型转换、格式化输出。
//!
//! ## 前置知识
//! Rust 整数类型分为：
//! - 有符号：i8 / i16 / i32 / i64 / i128 / isize
//! - 无符号：u8 / u16 / u32 / u64 / u128 / usize
//!
//! 浮点数：f32（单精度）、f64（双精度，默认推荐）
//!
//! ## 练习题
//! 1. 用 `format!` 输出 `666` 的 8 位十六进制形式（前面补零）。
//! 2. 用 `saturating_mul` 计算 `u8::MAX * 3`，观察结果。
//! 3. 用 `TryFrom` 把 `u32::MAX` 转成 `u8`，看返回什么错误。

// ============ 数值转字符串与格式化输出 ============
pub fn number_to_string() {
    let i = 666;
    let f1 = 88.888;
    let f2 = 88.0;

    // 基础转字符串
    println!("i={}", i.to_string()); // 结果：i=666

    // 精度控制（四舍五入）
    println!("f1={}", format!("{:.2}", f1)); // 指定精度为两位小数，结果 f1=88.89
    println!("f2={}", format!("{:.2}", f2)); // 结果 f2=88.00

    // ===== 更多格式化方式（新手学习重点）=====

    // 十六进制 / 八进制 / 二进制
    println!("666 hex=0x{:X}  octal=0o{:o}  binary=0b{:b}", i, i, i);
    // 结果：666 hex=0x29A  octal=0o1232  binary=0b1010011010

    // 前导零填充 + 宽度控制：{:0>8} 表示「右对齐，总宽度 8，不足补 0」
    println!("666 with leading zeros: {:0>8}", i);
    // 结果：666 with leading zeros: 00000666

    // 对齐：< 左对齐，^ 居中，> 右对齐（默认）
    println!("left=|{:<10}| center=|{:^10}| right=|{:>10}|", i, i, i);
    // 结果：left=|666       | center=|   666    | right=|       666|

    // 正负号显式显示
    println!("positive= {:+}  negative= {:+}", 666, -888);
    // 结果：positive= +666  negative= -888
}

// ============ 整数溢出安全处理（四种模式对比）============
//
// Rust Debug 模式下溢出会 panic，但 Release 模式默认 wrapping 行为。
// 业务代码应该**显式选择**一种溢出处理方式，而不是依赖默认行为。
pub fn overflow() {
    let x: u8 = 255;

    // 1) checked_add：溢出返回 None，最安全，推荐默认使用
    match x.checked_add(1) {
        Some(result) => println!("checked_add:   255+1 = {}", result),
        None => println!("checked_add:   255+1 = 溢出了（返回 None）"),
    }

    // 2) saturating_add：饱和运算，溢出时取类型最大/最小值
    //    常用于计数、限流等「不能回绕」的场景
    let sat = x.saturating_add(1);
    println!("saturating_add: 255+1 = {}（饱和，卡在 u8::MAX）", sat); // 255

    let sat_neg: i8 = (-128i8).saturating_sub(1);
    println!("saturating_sub: -128-1 = {}（饱和，卡在 i8::MIN）", sat_neg); // -128

    // 3) wrapping_add：回绕运算，溢出时从另一端重新开始
    //    适合哈希算法、CRC 等算法本身就需要模运算的场景
    let wrap = x.wrapping_add(1);
    println!("wrapping_add:   255+1 = {}（回绕到 0）", wrap); // 0

    // 4) overflowing_add：返回 (结果, 是否溢出) 元组
    //    适合需要自行判断是否溢出并做后续处理的场景
    let (result, overflowed) = x.overflowing_add(1);
    println!("overflowing_add: 255+1 = {}, 溢出？{}", result, overflowed); // 0, true

    // ===== 其他运算也有对应四种方法：sub / mul / div / rem / pow / shl / shr =====
    // 例如 checked_mul、saturating_pow、wrapping_shl 等，命名规则一致
}

// ============ 数值类型转换 ============
//
// 三种转换方式，安全性从高到低：
// 1) From / Into trait      ：编译期保证无损失转换（如 u8 -> u32），优先使用
// 2) TryFrom / TryInto trait：可能失败的转换（如 u32 -> u8），返回 Result
// 3) as 关键字               ：强制转换，截断或未定义行为（指针转整数等），慎用
pub fn type_conversion() {
    // --- 1. From / Into（无损失，编译期通过）---
    let small: u8 = 10;
    let big: u32 = small.into(); // u8 -> u32 一定安全，所以可以用 Into
    println!("u8->u32: {} -> {}", small, big); // 10 -> 10

    // From 与 Into 是互逆的，实现 From 自动获得 Into
    let big2 = u32::from(small);
    assert_eq!(big, big2);

    // --- 2. TryFrom / TryInto（可能失败，返回 Result）---
    let too_big: u32 = 1000;
    // u32 -> u8 可能溢出，所以必须用 TryFrom
    match u8::try_from(too_big) {
        Ok(v) => println!("u32->u8 成功: {}", v),
        Err(e) => println!("u32->u8 失败: 1000 -> 错误类型: {}", e),
        // 提示：生产代码中不要 panic，这里仅演示错误分支
    }

    // 小数值转换成功的情况
    let ok_val: u32 = 200;
    // 使用 turbofish 语法指定目标类型
    let res: Result<u8, _> = ok_val.try_into();
    println!("u32->u8 成功: 200 -> {}", res.unwrap());
    // 免责：unwrap() 仅示例方便，生产请用 match/?

    // --- 3. as 强制转换（截断，需慎用）---
    // as 会默默地截断高位，不会报错，新手容易踩坑
    let a: u32 = 0x1234_ABCD;
    let b: u8 = a as u8; // 只保留最低字节
    println!("u32 as u8 截断: 0x{:08X} -> 0x{:02X}", a, b); // 0x1234ABCD -> 0xCD

    // 浮点数转整数会向零截断
    let pi: f64 = 3.99;
    let truncated: i32 = pi as i32;
    println!("f64 as i32 向零截断: {} -> {}", pi, truncated); // 3

    // 如果需要四舍五入，用 round() 再转
    let rounded: i32 = pi.round() as i32;
    println!("f64 round() 后转: {} -> {}", pi, rounded); // 4
}

// ============ 练习题参考答案（cargo test 可验证）============
#[cfg(test)]
mod tests {
    // 练习 1：用 format! 输出 666 的 8 位十六进制（前面补零）
    #[test]
    fn exercise_1_hex_leading_zeros() {
        assert_eq!(format!("{:08X}", 666), "0000029A");
    }

    // 练习 2：saturating_mul 计算 u8::MAX * 3（饱和，不会溢出）
    #[test]
    fn exercise_2_saturating_mul() {
        assert_eq!(u8::MAX.saturating_mul(3), u8::MAX);
    }

    // 练习 3：TryFrom 把 u32::MAX 转 u8（应返回 Err）
    #[test]
    fn exercise_3_try_from_overflow() {
        assert!(u8::try_from(u32::MAX).is_err());
        assert_eq!(u8::try_from(200u32), Ok(200));
    }
}
