//! 随机数示例（rand）。
//!
//! ## 前置知识
//! - **rand**：Rust 标准随机数库，提供线程安全 RNG、各种分布
//! - `rand::rng()`：获取线程局部随机数生成器（等价于旧版 `thread_rng()`）
//! - `random_range()`：生成指定范围内的随机数
//! - `random()`：生成随机值（类型由编译器推断）
//!
//! ## 练习题
//! 1. 生成一个 6 位随机验证码（纯数字）。
//! 2. 模拟掷两个骰子，打印点数之和（1~6 + 1~6）。
//! 3. 生成一个 16 字节的随机 UUID（提示：用 `fill_bytes` 填充数组）。

use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, RngExt};

// ============ 基础随机数 ============

pub fn basic_random() {
    let mut rng = rand::rng();

    // 随机整数（闭区间 1..=100）
    let n: i32 = rng.random_range(1..=100);
    println!("✅ 随机整数 1..=100: {n}");

    // 随机浮点数（半开区间 0.0..1.0）
    let f: f64 = rng.random_range(0.0..1.0);
    println!("✅ 随机浮点 0.0..1.0: {f:.4}");

    // 随机布尔值
    let b: bool = rng.random();
    println!("✅ 随机布尔: {b}");

    // 随机字符（用 random() + 类型标注）
    let c: char = rng.random();
    println!("✅ 随机 char: {c}");
}

// ============ 从集合中随机选择 ============

pub fn random_choice() {
    let mut rng = rand::rng();
    let fruits = ["苹果", "香蕉", "橙子", "葡萄", "西瓜"];

    // choose：从切片中随机选一个元素
    let pick = fruits.choose(&mut rng).expect("非空切片"); // 免责：数组非空
    println!("✅ 随机选一个水果: {pick}");

    // 加权随机：简单做法——在数组中重复元素来模拟权重
    // 例如 A 出现 3 次、B 出现 1 次、C 出现 1 次 → A 的概率 = 3/5 = 60%
    let weighted = ["A", "A", "A", "B", "C"]; // A 概率 3/5
    let weighted_pick = weighted.choose(&mut rng).unwrap(); // 免责：非空
    println!("✅ 加权随机 (A:60% B:20% C:20%): {weighted_pick}");

    // shuffle：原地打乱
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("✅ shuffle 打乱: {nums:?}");

    // partial_shuffle：只取前 N 个（不需要打乱全部）
    let mut deck: Vec<u32> = (1..=54).collect();
    let (hand, _rest) = deck.partial_shuffle(&mut rng, 5);
    println!("✅ partial_shuffle (抽 5 张): {hand:?}");
}

// ============ 随机字节 ============

pub fn random_bytes() {
    let mut rng = rand::rng();

    // fill_bytes：填充字节切片
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    println!("✅ 随机字节 [0u8; 16]: {bytes:?}");

    // 把随机字节转成十六进制字符串
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    println!("✅ 十六进制: {hex}");
}

// ============ 随机密码生成器（实战）============

pub fn generate_password(length: usize) {
    let mut rng = rand::rng();
    // 字符池：大小写字母 + 数字 + 特殊符号
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@#$%^&*()_+-=[]{}";

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("✅ 随机密码 (长度 {length}): {password}");
}

fn main() {
    println!("============= basic_random =============");
    basic_random();
    println!("\n============= random_choice =============");
    random_choice();
    println!("\n============= random_bytes =============");
    random_bytes();
    println!("\n============= generate_password =============");
    generate_password(16);
}
