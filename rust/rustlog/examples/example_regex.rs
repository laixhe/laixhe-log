//! 正则表达式示例（regex）。
//!
//! ## 前置知识
//! - **regex**：Rust 的正则表达式库，基于 NFA，线性时间保证（不会指数爆炸）
//! - `Regex::new(pattern)`：编译正则（有编译开销，建议 `OnceLock` 缓存）
//! - `is_match()`：是否匹配，`captures()`：提取分组，`replace_all()`：替换
//!
//! ## 练习题
//! 1. 写一个正则匹配中国大陆手机号（1 开头，11 位数字）。
//! 2. 用 `replace_all` 把文本中的所有 URL 替换成 `[链接]`。
//! 3. 用 `RegexBuilder` 设置大小写不敏感，匹配 "Rust" / "rust" / "RUST"。

use regex::Regex;
use std::sync::LazyLock;

// LazyLock：编译一次，全局复用
// Regex 编译有开销，不应每次调用都 new
static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    // 免责：固定模式，编译不会失败
});

// ============ 基础匹配 ============

pub fn basic_match() {
    let re = Regex::new(r"\d+").unwrap(); // 免责：固定模式
    let text = "我有 3 个苹果和 15 个橙子";

    // is_match：是否包含匹配（不提取内容）
    println!("✅ is_match: {}", re.is_match(text));

    // find：找到第一个匹配
    if let Some(m) = re.find(text) {
        println!("✅ find: '{}' at {:?}", m.as_str(), m.range());
    }

    // find_iter：所有匹配
    let nums: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("✅ find_iter: {nums:?}");
}

// ============ 捕获组 ============

pub fn capture_groups() {
    // 用括号 () 创建捕获组
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap(); // 免责：固定模式
    let text = "日期：2024-12-25 和 2025-01-01";

    for caps in re.captures_iter(text) {
        // caps[0] 是完整匹配，caps[1..] 是各捕获组
        println!("✅ 完整匹配: {}", &caps[0]);
        println!("   年: {}  月: {}  日: {}", &caps[1], &caps[2], &caps[3]);
    }
}

// ============ 替换 ============

pub fn replace() {
    let re = Regex::new(r"\d+").unwrap(); // 免责：固定模式
    let text = "苹果 3 个，橙子 15 个";

    // replace_all：把所有数字替换为 N
    let result = re.replace_all(text, "N");
    println!("✅ replace_all: {result}");

    // 用捕获组 + $1 $2 做动态替换
    let re2 = Regex::new(r"(\w+)@(\w+\.\w+)").unwrap();
    let text2 = "联系我：laixhe@example.com";
    let result2 = re2.replace_all(text2, "$1 [at] $2");
    println!("✅ replace_all (捕获组): {result2}");
}

// ============ 分割 ============

pub fn split() {
    // 按逗号、分号、空白分割
    let re = Regex::new(r"[,;\s]+").unwrap(); // 免责：固定模式
    let text = "rust, go; python  javascript";

    let parts: Vec<&str> = re.split(text).collect();
    println!("✅ split: {parts:?}");
}

// ============ 邮箱验证（LazyLock 实战）============

pub fn email_validation() {
    let emails = vec!["user@example.com", "invalid-email", "a@b.cn", "x@y."];

    for email in emails {
        let valid = EMAIL_RE.is_match(email);
        println!("  {email:20} → {}", if valid { "✅ 合法" } else { "❌ 不合法" });
    }
}

// ============ 手机号 + 邮箱大小写不敏感 ============

pub fn phone_and_case_insensitive_email() {
    use regex::RegexBuilder;

    // 中国大陆手机号：1 开头，第二位 3-9，后面 9 位数字，共 11 位
    let phone_re = Regex::new(r"^1[3-9]\d{9}$").unwrap(); // 免责：固定模式
    let phones = ["13812345678", "19912345678", "12812345678", "1381234567"];
    println!("手机号匹配:");
    for p in phones {
        println!("  {p:14} → {}", if phone_re.is_match(p) { "✅ 合法" } else { "❌ 不合法" });
    }

    // 邮箱：大小写不敏感用 RegexBuilder 的 case_insensitive(true)
    // （等价于在模式里写 (?i)，但 RegexBuilder 更直观、可读）
    let email_re = RegexBuilder::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$")
        .case_insensitive(true)
        .build()
        .unwrap(); // 免责：固定模式
    let emails = [
        "laixhe@example.com",
        "LAIXHE@EXAMPLE.COM",
        "LaixHe@Example.com",
        "user.name+tag@mail.example.org",
        "not-an-email",
    ];
    println!("\n邮箱匹配(大小写不敏感):");
    for e in emails {
        println!("  {e:28} → {}", if email_re.is_match(e) { "✅ 合法" } else { "❌ 不合法" });
    }
}

fn main() {
    println!("============= basic_match =============");
    basic_match();
    println!("\n============= capture_groups =============");
    capture_groups();
    println!("\n============= replace =============");
    replace();
    println!("\n============= split =============");
    split();
    println!("\n============= email_validation =============");
    email_validation();
    println!("\n============= phone_and_case_insensitive_email =============");
    phone_and_case_insensitive_email();
}
