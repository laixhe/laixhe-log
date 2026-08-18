//! 日志与追踪示例（log + tracing + tracing-subscriber）。
//!
//! ## 前置知识
//! - **log**：Rust 生态最基础的日志门面（facade），只定义宏，不负责输出
//! - **tracing**：新一代结构化日志 / 追踪框架，支持 span（跨度）、上下文关联
//! - **tracing-subscriber**：tracing 的后端实现，负责格式化输出到终端 / 文件
//! - **env_logger**：log 的后端实现，通过环境变量 `RUST_LOG` 控制日志级别
//!
//! ## log vs tracing 选型
//! | 场景 | 推荐 | 原因 |
//! |---|---|---|
//! | 简单日志输出 | log + env_logger | 轻量，零学习成本 |
//! | 异步 / 多线程追踪 | tracing | span 关联上下文，不丢链路 |
//! | 生产可观测性 | tracing | 可接入 OpenTelemetry / Jaeger |
//!
//! ## 日志级别（从低到高）
//! `ERROR` < `WARN` < `INFO` < `DEBUG` < `TRACE`
//! 设置某个级别后，只输出该级别及更高级别的日志。
//!
//! ## 练习题
//! 1. 把 `RUST_LOG=trace` 环境变量设上再运行，观察 TRACE 级别输出。
//! 2. 试着把 `tracing::info!("处理订单中...")` 改成 `tracing::warn!`，观察级别变化。
//! 3. 试着用 `tracing::span!` 手动创建一个 span，在内部记录日志。

// ============ log crate 演示 ============

pub fn log_demo() {
    // 初始化 log 后端（env_logger）
    // 通过环境变量 RUST_LOG 控制级别，默认 info
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug")
    )
    .format_timestamp_millis()
    .try_init();
    // try_init：若已被初始化则忽略错误（不 panic）

    println!("\n--- log crate 宏演示 ---");
    log::error!("❌ error!：严重错误，程序可能无法继续");
    log::warn!("⚠️  warn!：警告，潜在问题");
    log::info!("ℹ️  info!：一般信息");
    log::debug!("🐛 debug!：调试信息");
    log::trace!("📍 trace!：最详细的跟踪信息");
}

// ============ tracing crate 演示 ============

pub fn tracing_demo() {
    // 初始化 tracing 后端（tracing-subscriber）
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)  // 不输出模块名
        .compact()
        .try_init()
        .ok();
    // try_init：若已被初始化则忽略（和 log 的 try_init 一样）

    println!("\n--- tracing crate 宏演示 ---");
    tracing::error!("❌ error!");
    tracing::warn!("⚠️  warn!");
    tracing::info!("ℹ️  info!");
    tracing::debug!("🐛 debug!");
}

// ============ tracing span（跨度）演示 ============

// span 是 tracing 的核心概念：标记一段代码的「上下文」
// 所有 span 内的日志都会自动带上 span 的信息

pub fn span_demo() {
    println!("\n--- tracing span 演示 ---");

    // 方式 1：手动创建 span
    let span = tracing::span!(tracing::Level::INFO, "checkout", user_id = 42);
    let _enter = span.enter();  // 进入 span（离开作用域自动退出）

    tracing::info!("开始结算");
    tracing::debug!("检查库存...");
    tracing::debug!("计算运费...");
    tracing::info!("结算完成");
    // 以上所有日志都会带上 [checkout user_id=42] 的上下文
}

// 方式 2：用 #[tracing::instrument] 宏自动创建 span
// 函数被调用时自动创建一个以函数名命名的 span，参数自动记录
#[tracing::instrument]
fn process_order(order_id: u32, amount: f64) {
    tracing::info!("处理订单中...");
    tracing::debug!(amount, "订单金额确认");
    // 日志会自动带上 span: [process_order order_id=.. amount=..]
}

pub fn instrument_demo() {
    println!("\n--- #[tracing::instrument] 演示 ---");
    process_order(1024, 99.5);
}

fn main() {
    log_demo();
    tracing_demo();
    span_demo();
    instrument_demo();
}
