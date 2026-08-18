//! Bevy 0.19 入门示例：演示日志系统（log 宏与级别过滤）。
//! 演示 trace / debug / info / warn / error 五个级别，以及 RUST_LOG 过滤。
//!
//! 学习重点：
//! - Bevy 内置日志宏：trace! / debug! / info! / warn! / error!（级别从低到高）
//! - 默认只显示 info 及以上（info / warn / error）；debug / trace 需要 RUST_LOG 开启
//! - 运行方式：
//!   cargo run --example example_logging                          # 只显示 info 及以上
//!   RUST_LOG=debug cargo run --example example_logging           # 额外显示 debug
//!   RUST_LOG=bevylog=trace cargo run --example example_logging   # 只对 bevylog 开启 trace
//! - LogPlugin 的 filter 字段可编程设置日志过滤（见 README FAQ）

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, log_startup)
        .add_systems(Update, log_every_second)
        .run()
}

// 启动时五个级别各打一条，观察默认情况下哪些会显示
fn log_startup() {
    trace!("[trace] 最详细，默认不显示");
    debug!("[debug] 调试信息，默认不显示");
    info!("[info] 一般信息，默认显示");
    warn!("[warn] 警告，默认显示");
    error!("[error] 错误，默认显示");
}

// 每秒打一条 info 和 debug，演示运行中的日志
fn log_every_second(time: Res<Time>, mut last: Local<f32>) {
    if time.elapsed_secs() - *last > 1.0 {
        *last = time.elapsed_secs();
        info!("[每秒] 应用已运行 {:.0} 秒", time.elapsed_secs());
        debug!("[每秒 debug] 需要 RUST_LOG=debug 才能看到这一行");
    }
}
