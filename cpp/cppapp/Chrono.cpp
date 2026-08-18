#include "Chrono.h"

#include <format>   // std::format [C++20]
#include <iostream>

// 注：MinGW 的 libstdc++ 缺少 <print> 的终端符号（__open_terminal），直接使用
// std::print / std::println 会链接失败；因此统一用 std::format + std::cout 输出（功能等价）。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

Chrono::Chrono()
{
    // ===== 1. 时钟 clock：三种常用时钟 =====
    std::cout << "--- 时钟 ---" << std::endl;

    // system_clock：墙钟时间（可转日历/时区，对应 Go time.Now / Rust SystemTime）
    // {:%F %T}：%F = 年月日，%T = 时分秒（C++20 时间格式化说明符）
    PRINT("system_clock 已支持格式化: {:%F %T}",
          std::chrono::system_clock::now()); // 2026-03-04 12:00:00

    // steady_clock：单调时钟（只增不减，专用于测量耗时，对应 Go time.Now 差值 / Rust Instant）
    PRINT("steady_clock 是单调时钟: {}", std::chrono::steady_clock::is_steady); // true

    // ===== 2. 时间点 time_point 运算（对应 Go time.Time / Rust SystemTime）=====
    std::cout << "--- 时间点 ---" << std::endl;

    auto t1 = std::chrono::system_clock::now();
    // 时间点 + 时长 = 时间点（对应 Go Add / Rust + Duration）
    auto t2 = t1 + std::chrono::seconds{30};
    // 时间点 - 时间点 = 时长（对应 Go Sub / Rust duration_since）
    auto diff = t2 - t1;
    // system_clock 精度是纳秒，需 duration_cast 换到秒再取 count()
    PRINT("时间点相差 {} 秒", std::chrono::duration_cast<std::chrono::seconds>(diff).count()); // 30

    // ===== 3. 时长 duration：时间段（对应 Go time.Duration / Rust Duration）=====
    std::cout << "--- 时长 ---" << std::endl;

    // 时长由 数值 + 单位 组成，可加减运算
    auto d = std::chrono::minutes{2} + std::chrono::seconds{30}; // 2 分 30 秒
    // duration_cast 显式换算单位（对应 Go Duration 整除换算 / Rust as_secs）
    PRINT("2 分 30 秒 = {} 秒", std::chrono::duration_cast<std::chrono::seconds>(d).count());   // 150
    PRINT("         = {} 毫秒", std::chrono::duration_cast<std::chrono::milliseconds>(d).count()); // 150000

    // ===== 4. 耗时测量（对应 Go time.Since / Rust Instant::elapsed）=====
    std::cout << "--- 耗时测量 ---" << std::endl;

    auto start = std::chrono::steady_clock::now();
    volatile long long sum = 0; // volatile：禁止编译器把累加循环优化掉（若被优化，耗时≈0 测不出差异）
    for (int i = 0; i < 1'000'000; ++i) {
        sum += i;
    }
    auto cost = std::chrono::steady_clock::now() - start;
    long long result = sum; // volatile 类型没有 std::formatter 支持，需复制到普通变量才能格式化输出
    PRINT("100 万次累加耗时: {} 微秒 (sum={})",
          std::chrono::duration_cast<std::chrono::microseconds>(cost).count(), result);

    // ===== 5. 日历类型（C++20）：year / month / day =====
    std::cout << "--- 日历 ---" << std::endl;

    // 运算符构造日期：year{2026} / 3 / 4 → 2026-03-04（对应 Rust NaiveDate::from_ymd）
    auto today = std::chrono::year{2026} / 3 / 4;
    PRINT("日期: {:%Y-%m-%d} 星期: {:%A}", today, today); // 2026-03-04 Wednesday
    // 注：%A 按 C locale 输出星期名，中文系统下默认仍显示英文（如 Wednesday）

    // 取出年/月/日分量（对应 Go Year/Month/Day / Rust .year()/.month()）
    PRINT("年={} 月={} 日={}", today.year(),
          static_cast<unsigned>(today.month()), static_cast<unsigned>(today.day())); // 2026 3 4

    // 日历运算：+ months / + days（对应 Go AddDate(y,m,d) / Rust + Duration）
    // 平台坑：标准规定 year_month_day 可直接 + days，但 MinGW 的 libstdc++ 尚未实现；
    // 故先经 sys_days 转成时间点加天数，再转回日历（MSVC / Clang libc++ 可直接写 ymd + days）
    PRINT("+1 月: {:%Y-%m-%d}", today + std::chrono::months{1}); // 2026-04-04
    auto plus30 = std::chrono::year_month_day{std::chrono::sys_days{today} + std::chrono::days{30}};
    PRINT("+30 天: {:%Y-%m-%d}", plus30); // 2026-04-03

    // 星期判断：weekday 可比较（对应 Go Weekday / Rust weekday()）
    auto wd = std::chrono::weekday{today};
    PRINT("是否周六/周日: {}", wd == std::chrono::Saturday || wd == std::chrono::Sunday); // false

    // ===== 6. 时间点 ↔ 日历互转（对应 Go time.Date 往返 / Rust 转换）=====
    std::cout << "--- 时间点与日历 ---" << std::endl;

    // 日历 → 时间点：sys_days 是 days 精度的时间点
    auto tp = std::chrono::sys_days{today};
    PRINT("2026-03-04 的 Unix 时间戳: {} 秒", std::chrono::system_clock::to_time_t(tp));

    // 时间点 → 日历：floor<days> 截断到天，再转 year_month_day
    auto now = std::chrono::system_clock::now();
    auto today_now = std::chrono::floor<std::chrono::days>(now);
    PRINT("现在的日期: {:%Y-%m-%d %A}", std::chrono::year_month_day{today_now});

    // ===== 7. 时区 zoned_time（对应 Go time.LoadLocation / Rust chrono-tz）=====
    std::cout << "--- 时区 ---" << std::endl;

    try {
        // current_zone() 获取系统时区，再把时间点绑定到该时区
        auto zt = std::chrono::zoned_time{std::chrono::current_zone(), std::chrono::system_clock::now()};
        PRINT("本地时区时间: {:%F %T}", zt);
    } catch (const std::exception& e) {
        // 平台坑：libstdc++ 的时区功能依赖 tzdata 时区数据库，MinGW 默认不带；
        // 获取不到时 current_zone() 会抛 std::runtime_error（Linux/macOS 一般自带）。
        std::cout << "时区数据不可用: " << e.what() << std::endl;
    }
}
