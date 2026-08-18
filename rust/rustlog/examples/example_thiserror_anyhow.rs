//! 错误处理示例（thiserror + anyhow）。
//!
//! ## 前置知识
//! - **thiserror**：为库定义自定义错误类型的利器，通过 derive 宏自动实现 Error trait
//! - **anyhow**：为应用层提供简单的错误链式传播，不关心错误类型时使用
//! - `?` 运算符：遇到错误自动提前返回，是 Rust 错误处理的核心语法
//!
//! ## thiserror vs anyhow 选型
//! | 场景 | 推荐 | 原因 |
//! |---|---|---|
//! | 库 (library) | thiserror | 调用者需要 match 错误类型，必须精确 |
//! | 应用 (application) | anyhow | 只需打印/记录错误，不需要 match |
//!
//! ## 练习题
//! 1. 给 AppError 加一个 `NotFound { key: String }` 变体，并在函数中触发。
//! 2. 用 `anyhow::Context` 给 `?` 添加上下文信息：`file.read().context("读取配置失败")?`。
//! 3. 试着用 `downcast_ref` 从 `anyhow::Error` 中提取具体的 `AppError`。

use thiserror::Error;
use anyhow::Context;

// ============ thiserror：自定义错误类型 ============
//
// #[derive(Error)] 会自动实现 std::error::Error + Display
// #[error("...")] 定义 Display 的输出格式
// #[from] 自动实现 From<源错误> → Self，让 ? 运算符自动转换

#[derive(Debug, Error)]
enum AppError {
    // #[from]：自动从 std::io::Error 转换
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    // #[from]：自动从 ParseIntError 转换
    #[error("解析错误: {0}")]
    Parse(#[from] std::num::ParseIntError),

    // 结构体变体：命名字段，可在 Display 中引用
    #[error("自定义错误: {message}")]
    Custom { message: String },
}

// ============ thiserror 使用示例 ============

/// 模拟读取配置并解析数字，可能失败
fn read_config() -> Result<i32, AppError> {
    let s = "not_a_number";
    // ? 运算符：parse() 返回 Err(ParseIntError) 时，
    // 通过 #[from] 自动转换为 AppError::Parse 并提前返回
    let n: i32 = s.parse()?;
    Ok(n)
}

fn trigger_custom_error() -> Result<(), AppError> {
    Err(AppError::Custom {
        message: "用户名不能为空".to_string(),
    })
}

// ============ anyhow：应用层错误传播 ============
//
// anyhow::Result<T> = Result<T, anyhow::Error>
// anyhow::Error 可以包装任何实现了 std::error::Error 的错误
// 不需要定义错误类型，直接用 ? 传播，适合应用代码

fn do_something() -> anyhow::Result<()> {
    // read_config 返回 Result<_, AppError>
    // AppError 实现了 Error，所以可以被 anyhow::Error 包装
    let _n = read_config()?;
    Ok(())
}

fn with_context_demo() -> anyhow::Result<String> {
    // .context()：anyhow 的核心方法，给 ? 添加上下文信息
    // 出错时能看到「在哪一步出错的」，错误链会多一层
    let content = std::fs::read_to_string("nonexistent.txt")
        .context("读取配置文件失败")?;
    Ok(content)
}

// ============ 演示 ============

pub fn thiserror_demo() {
    println!("--- thiserror 演示 ---");

    // 情况 1：解析错误
    match read_config() {
        Ok(n) => println!("✅ 解析成功: {n}"),
        Err(e) => {
            println!("❌ thiserror 捕获: {e}");
            // thiserror 的优势：可以 match 具体错误类型
            match &e {
                AppError::Parse(p) => println!("   → 具体类型: ParseIntError({p})"),
                AppError::Io(_) => println!("   → 具体类型: Io"),
                AppError::Custom { message } => println!("   → 具体类型: Custom({message})"),
            }
        }
    }

    // 情况 2：自定义错误
    match trigger_custom_error() {
        Ok(()) => println!("❓ 不会成功"),
        Err(e) => println!("❌ thiserror 捕获: {e}"),
    }
}

pub fn anyhow_demo() {
    println!("\n--- anyhow 演示 ---");

    match do_something() {
        Ok(()) => println!("✅ anyhow 成功"),
        Err(e) => {
            println!("❌ anyhow 捕获: {e}");
            // anyhow 支持错误链（cause chain）
            println!("   错误链:");
            for (i, cause) in e.chain().enumerate() {
                println!("     [{i}] {cause}");
            }
        }
    }

    // anyhow! 宏：快速创建错误
    let err = anyhow::anyhow!("手动创建的错误");
    println!("\nanyhow! 宏: {err}");

    // with_context 演示
    match with_context_demo() {
        Ok(s) => println!("✅ 文件内容: {s}"),
        Err(e) => println!("❌ with_context: {e}"),
    }
}

fn main() {
    thiserror_demo();
    anyhow_demo();
}
