#include "StdEnum.h"

#include <cstdint>   // int8_t
#include <format>    // std::format [C++20]
#include <iostream>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// ===== 传统 enum（C++98）：值会泄漏到外层作用域 =====
enum Color { RED, GREEN, BLUE }; // RED=0 GREEN=1 BLUE=2

// ===== 强类型枚举 enum class（C++11，推荐）=====
// 优势：不泄漏名称、不隐式转换、可指定底层类型
enum class Weekday : int8_t { Monday = 1, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday };

StdEnum::StdEnum()
{
    // ===== 1. 传统 enum =====
    std::cout << "--- 传统 enum ---" << std::endl;

    Color c = RED;
    PRINT("RED 值: {}", static_cast<int>(c)); // 0（传统 enum 可隐式转 int）

    // ===== 2. enum class（强类型）=====
    std::cout << "--- enum class ---" << std::endl;

    Weekday day = Weekday::Wednesday;
    // 不能隐式转 int（强类型）：static_cast<int>(day) 才是 3
    PRINT("Wednesday 值: {}", static_cast<int>(day)); // 3

    // 默认从 0 递增，可显式指定起始值
    PRINT("Monday 值: {}", static_cast<int>(Weekday::Monday));   // 1
    PRINT("Sunday 值: {}", static_cast<int>(Weekday::Sunday));   // 7

    // ===== 3. enum class 与 switch（对应 Rust match / Go switch）=====
    std::cout << "--- switch 配合 ---" << std::endl;

    // 注意：C++ 的 switch 必须 break（区别于 Go/Rust）
    switch (day) {
        case Weekday::Monday:
        case Weekday::Tuesday:
        case Weekday::Wednesday:
        case Weekday::Thursday:
        case Weekday::Friday:
            PRINT("工作日（周{}）", static_cast<int>(day));
            break;
        case Weekday::Saturday:
        case Weekday::Sunday:
            PRINT("休息日（周{}）", static_cast<int>(day));
            break;
        default:
            PRINT("未知");
    }
}
