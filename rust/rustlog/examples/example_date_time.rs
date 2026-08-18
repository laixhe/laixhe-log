//! 日期时间处理：标准库 std::time + 更强大的 jiff 库。
//!
//! ## 前置知识
//! - `std::time::Duration`  —— 时间间隔（秒 / 毫秒 / 微秒 / 纳秒），**与日期无关**
//! - `std::time::Instant`   —— 单调递增时间点，适合**耗时统计 / 基准测试**（不受系统时间回拨影响）
//! - `std::time::SystemTime`—— 系统墙上时钟（wall clock），可能被 NTP 回拨，适合「实际年月日时分秒」
//! - **jiff** crate（本示例重点）—— 标准库没有日期时区格式化，jiff 提供了完整的：
//!     - 时区支持（`Zoned` / IANA 时区数据库如 `Asia/Shanghai`）
//!     - 格式化输出（strftime 风格）
//!     - 日期时间计算（加减天数、月份差、本地日历）
//!
//! ## 练习题
//! 1. 用 jiff 计算「2024-02-29 加 1 年」会得到什么日期（注意闰年）。
//! 2. 用 `Instant` 统计一个 1 千万次循环的耗时。
//! 3. jiff 不支持农历。想想为什么一个通用日期库不做农历？（提示：农历规则复杂、
//!    各地区历法不统一、需求小众，通常由专门的农历 crate 如 `lunar-calendar` 补充。）

use jiff::{ToSpan, Zoned};
use std::thread;

// ============ 标准库部分 ============

pub fn std_time() {
    // Instant：单调时间点，适合测耗时（推荐所有性能统计都用它！）
    let start = std::time::Instant::now();

    // SystemTime：系统墙上时钟，能和「真实日期」转换，但可能跳变
    let now_sys = std::time::SystemTime::now();
    println!("SystemTime 当前: {:?}", now_sys);
    // SystemTime 本身不好直接显示年月日——这也是我们需要 jiff / chrono 的原因

    println!("Instant 距离 start 耗时：{:?}", start.elapsed());
}

/// 休眠示例：Duration 多种构造方式
pub fn sleep_demo() {
    // Duration 构造（都是零成本抽象，编译期就是整数）
    let _s  = std::time::Duration::from_secs(3);          // 3 秒
    let _ms = std::time::Duration::from_millis(300);      // 300 毫秒
    let _us = std::time::Duration::from_micros(500);      // 500 微秒
    let _ns = std::time::Duration::from_nanos(1000);      // 1000 纳秒 = 1 微秒

    // 实际 sleep 1 秒
    println!("(休眠 1 秒演示 Duration...)");
    thread::sleep(std::time::Duration::from_secs(1));
}

// ============ jiff 库部分（重点！）============

pub fn jiff_demo() -> anyhow::Result<()> {
    println!("\n======= jiff 日期时间处理 =======");
    use jiff::Timestamp;

    // ---- 1. 获取当前时间 + 时区 ----
    // Zoned::now() 直接返回 Zoned（不是 Result），会读取 OS 本地时区
    let now_local = Zoned::now();
    println!("本地时区当前时间: {}", now_local);

    // 显式指定上海时区：Timestamp::now().in_tz("IANA 名") —— 最推荐的写法
    // （jiff 0.2 里没有 jiff::tz! 宏（需要 static feature），也没有 Zoned::now_utc()）
    let shanghai = Timestamp::now().in_tz("Asia/Shanghai")?;
    println!("上海时区(显式):   {}", shanghai);

    // UTC 同理
    let utc = Timestamp::now().in_tz("UTC")?;
    println!("UTC 时间:         {}", utc);

    // ---- 2. 格式化输出 ----
    // strftime 风格占位符：
    //   %F = %Y-%m-%d  ， %T = %H:%M:%S  ， %z = 时区偏移
    let fmt = shanghai.strftime("%Y年%m月%d日 %H:%M:%S 星期%u (%Z %z)");
    println!("格式化（中文友好）: {}", fmt);
    // 示例输出：2024年08月12日 15:30:45 星期1 (CST +08:00)

    // ---- 3. 构造指定日期 / 时间 ----
    // civil::Date 只有年月日（无时区）
    let d = jiff::civil::date(2024, 2, 29); // 2024 闰年
    println!("\n构造日期: {}", d);

    // civil::DateTime 加时分秒 + **亚秒纳秒**（jiff 0.2 需要 7 个参数）
    let dt = jiff::civil::datetime(2024, 8, 8, 20, 0, 0, 0);
    println!("构造 datetime: {}", dt);

    // ---- 4. 日期时间计算（加 / 减跨度 Span）----
    // ToSpan trait 提供了非常直观的语法：1.year()、3.days()、5.hours()
    let start_date = jiff::civil::date(2024, 2, 29); // 闰年 2 月 29 日
    let plus_1y = start_date.checked_add(1.year()).unwrap();  // 免责：示例方便
    let plus_1d = start_date.checked_add(1.day()).unwrap();
    println!("\n日期计算:");
    println!("  {} + 1 年 = {}（注意 2025 不是闰年，回退到 2 月 28 日）", start_date, plus_1y);
    println!("  {} + 1 天 = {}", start_date, plus_1d);

    // 也可以用 Zoned 做带时区的时间加减
    let in_3h = shanghai.checked_add(3.hours()).unwrap(); // 免责：示例方便
    println!("  上海时间 + 3 小时 = {}", in_3h.strftime("%F %T"));

    // ---- 5. 两个时间点的差值（Span）----
    let d1 = jiff::civil::date(2024, 1, 1);
    let d2 = jiff::civil::date(2024, 8, 12);
    let span_between = d1.until(d2)?;
    println!("\n{} 到 {} 相差 {} 天", d1, d2, span_between.get_days());

    Ok(())
}

/// 时间戳与格式化/解析/比较：覆盖日常最常见的时间需求。
pub fn timestamp_demo() -> anyhow::Result<()> {
    use jiff::{civil, Timestamp};

    println!("\n======= 时间戳与格式化/解析/比较 =======");

    // ---- 1. 获取当前时间戳（秒级）----
    // Timestamp 表示「时间点」，as_second() 返回自 1970-01-01 起的整秒数
    let now = Timestamp::now();
    let seconds = now.as_second();
    println!("1. 当前时间戳(秒): {seconds}");

    // ---- 2. 当前时间格式化 ----
    // %F = %Y-%m-%d（2026-08-13），%T = %H:%M:%S（12:13:14）
    let now_local = now.in_tz("Asia/Shanghai")?;
    println!("2. 当前时间格式化: {}", now_local.strftime("%F %T"));

    // ---- 3. 时间戳转时间对象 ----
    // from_second 把「秒级时间戳」转回 Timestamp，再转时区格式化
    let back = Timestamp::from_second(seconds)?;
    let back_local = back.in_tz("Asia/Shanghai")?;
    println!("3. 时间戳 {seconds} → 时间对象: {}", back_local.strftime("%F %T"));

    // ---- 4. 时间字符串解析 ----
    // civil::DateTime::strptime 解析「无时区」的日期时间字符串（最直观）
    // 注意：Timestamp::strptime 需要时区偏移，故无时区字符串要用 civil::DateTime
    let dt = civil::DateTime::strptime("%F %T", "2026-08-13 12:13:14")?;
    println!("4. 解析字符串 → 日期时间: {}", dt.strftime("%F %T"));

    // ---- 5. 时间比较 ----
    // civil::DateTime 实现了 Ord，可直接用 < / == 比较
    let t1 = civil::DateTime::strptime("%F %T", "2026-08-13 12:13:14")?;
    let t2 = civil::DateTime::strptime("%F %T", "2026-08-13 12:13:15")?;
    println!("5. 时间比较: t1 < t2 = {}，t1 == t1 = {}", t1 < t2, t1 == t1);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("============= std_time (Instant/SystemTime) =============");
    std_time();

    println!("\n============= sleep_demo (Duration) =============");
    sleep_demo();

    println!("\n============= jiff_demo (日期时区格式化) =============");
    jiff_demo()?;

    println!("\n============= timestamp_demo (时间戳/格式化/解析/比较) =============");
    timestamp_demo()?;

    Ok(())
}
