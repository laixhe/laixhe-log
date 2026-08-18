//! RON 序列化 / 反序列化示例（ron + serde）。
//!
//! ## 前置知识
//! - **RON** (Rusty Object Notation)：Rust 风格的对象序列化格式，语法接近 Rust 字面量
//! - 对比 JSON：RON 支持注释、尾部逗号、原始字符串、枚举变体，可读性更好
//! - 同样基于 serde，只需换掉序列化器即可
//!
//! ## RON vs JSON 对比
//! | 特性 | JSON | RON |
//! |---|---|---|
//! | 注释 | ❌ | ✅ `// line` `/* block */` |
//! | 尾部逗号 | ❌ | ✅ |
//! | 枚举 | 只能用字符串 | ✅ 原生 `Debug` |
//! | 原始字符串 | ❌ | ✅ `r#"..."#` |
//! | 可读性 | 一般 | 优秀（接近 Rust 代码） |
//!
//! ## 练习题
//! 1. 把 Config 结构体加上 `#[serde(default)]`，观察缺省字段时的行为。
//! 2. 尝试用 RON 表示一个 `Vec<Vec<i32>>` 嵌套数组。
//! 3. 对比同样结构体的 JSON pretty 输出和 RON pretty 输出，哪个更易读？

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    name: String,
    timeout: u32,
    // RON 原生支持枚举——JSON 里只能序列化成字符串
    mode: Mode,
    // 嵌套结构体
    database: Database,
    // 可选字段
    retries: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Mode {
    Debug,
    Release,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Database {
    host: String,
    port: u16,
}

// ============ 序列化 ============

pub fn serialize() {
    let config = Config {
        name:     "my_app".to_string(),
        timeout:  30,
        mode:     Mode::Debug,
        database: Database {
            host: "127.0.0.1".to_string(),
            port: 5432,
        },
        retries:  Some(3),
    };

    // 紧凑输出（一行）
    let ron_str = ron::to_string(&config).expect("RON 序列化失败");
    // 免责：固定结构体，序列化不会失败
    println!("✅ RON 序列化 (紧凑): {ron_str}");
    // 期望：(name:"my_app",timeout:30,mode:Debug,database:(host:"127.0.0.1",port:5432),retries:Some(3))

    // 美化输出（带缩进）
    let pretty = ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default())
        .expect("RON 序列化失败");
    println!("✅ RON 序列化 (美化):\n{pretty}");
}

// ============ 反序列化 ============

pub fn deserialize() {
    // RON 格式：注意它支持注释、尾部逗号、枚举无需引号
    let ron_str = r#"
        // RON 支持行注释
        Config(
            name: "my_app",
            timeout: 30,
            mode: Debug,    // 枚举变体直接写名字，不需要引号
            database: Database(
                host: "127.0.0.1",
                port: 5432, // 尾部逗号 OK
            ),
            retries: None,  // Option::None
        )
    "#;

    let config: Config = ron::from_str(ron_str).expect("RON 反序列化失败");
    println!("✅ RON 反序列化: {config:?}");
}

// ============ 错误处理 ============

pub fn error_handling() {
    // 故意少一个字段（database 缺失）
    let bad_ron = r#"Config(name: "test", timeout: 10, mode: Debug)"#;

    match ron::from_str::<Config>(bad_ron) {
        Ok(_) => println!("❓ 理论上不会成功"),
        Err(e) => println!("✅ 如预期失败（缺字段）: {e}"),
    }
}

fn main() {
    println!("============= serialize =============");
    serialize();
    println!("\n============= deserialize =============");
    deserialize();
    println!("\n============= error_handling =============");
    error_handling();
}
