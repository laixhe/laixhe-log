#include "StdNumber.h"

#include <algorithm>  // std::clamp
#include <charconv>   // std::from_chars [C++17]
#include <cmath>      // std::round
#include <cstdint>    // uint8_t
#include <format>     // std::format [C++20]
#include <iostream>
#include <limits>     // std::numeric_limits
#include <stdexcept>  // std::invalid_argument
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl
#define PRINTF(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__)

StdNumber::StdNumber()
{
    // ===== 1. 数值格式化输出（对应 Rust number_to_string）=====
    std::cout << "--- 数值格式化 ---" << std::endl;

    // 精度控制（四舍五入）
    PRINT("f1={:.2f} f2={:.2f}", 88.888, 88.0); // f1=88.89 f2=88.00

    // 十六进制 / 八进制 / 二进制（{x} 小写，{X} 大写）
    PRINT("666 hex=0x{:X} octal=0o{:o} binary=0b{:b}", 666, 666, 666);
    // 666 hex=0x29A octal=0o1232 binary=0b1010011010

    // 前导零填充 + 宽度控制（{0} 补零，右对齐）
    PRINT("leading zeros: {:08}", 666); // 00000666

    // 对齐：{:<10} 左对齐，{:>10} 右对齐
    PRINT("left=|{:<10}| right=|{:>10}|", 666, 666);
    // left=|666       | right=|       666|

    // 正负号显式显示（{:+d}）
    PRINT("positive={:+d}  negative={:+d}", 666, -888);
    // positive=+666  negative=-888

    // 千分位分组（libstdc++ 的 std::format 暂不支持 {:,}，此处手动拼接演示）
    PRINT("grouping: 1,234,567");

    // ===== 2. 整数溢出（对应 Rust overflow / Go 溢出检查）=====
    std::cout << "--- 整数溢出 ---" << std::endl;

    // 无符号回绕：u8 255 + 1 = 0（无符号溢出是定义行为，会回绕）
    uint8_t b = 255;
    PRINT("u8 255 + 1 = {}（回绕到 0）", static_cast<int>(static_cast<uint8_t>(b + 1)));

    // 有符号溢出是未定义行为！必须先检查边界（对应 Go 的溢出检查）
    int a = std::numeric_limits<int>::max();
    if (a < std::numeric_limits<int>::max() - 1) {
        PRINT("checked_add: {} + 2 = {}", a, a + 2);
    } else {
        PRINT("checked_add: MAX + 2 = 溢出了（已检查）");
    }

    // 饱和（对应 Rust saturating_add）：先用更大类型计算再 clamp
    // 注意：Windows 上 long 是 32 位，必须用 long long（64 位）才不会溢出
    long long wide = static_cast<long long>(a) + 1;
    int sat = static_cast<int>(std::clamp(wide,
        static_cast<long long>(std::numeric_limits<int>::min()),
        static_cast<long long>(std::numeric_limits<int>::max())));
    PRINT("saturating_add: MAX + 1 = {}（饱和）", sat);

    // 浮点精度：0.1 + 0.2 != 0.3（IEEE 754）
    PRINT("0.1 + 0.2 = {}（浮点精度问题）", 0.1 + 0.2);

    // ===== 3. 类型转换（对应 Rust type_conversion）=====
    std::cout << "--- 类型转换 ---" << std::endl;

    // 浮点转整数：static_cast 向零截断
    PRINT("int(3.99) = {}（向零截断）", static_cast<int>(3.99));
    // 四舍五入：std::round 先取整再转换
    PRINT("round(3.99) = {}", static_cast<int>(std::round(3.99)));

    // 字符串解析：std::stoi / std::stod（失败抛异常）
    PRINT("stoi('666') = {}", std::stoi("666"));
    PRINT("stod('88.88') = {}", std::stod("88.88"));

    // 进制解析（对应 Go strconv.ParseInt 指定 base）
    PRINT("stoi('29A', 16) = {}", std::stoi("29A", nullptr, 16)); // 666
    PRINT("stoi('1232', 8) = {}", std::stoi("1232", nullptr, 8)); // 666

    // 解析失败：抛 std::invalid_argument（区别于 PHP 返回 0）
    try {
        std::stoi("not_a_number");
    } catch (const std::invalid_argument&) {
        PRINT("stoi('not_a_number') = 抛 invalid_argument");
    }

    // std::from_chars [C++17]：不抛异常、最快
    int v = 0;
    const std::string str = "666";
    auto [ptr, ec] = std::from_chars(str.data(), str.data() + str.size(), v);
    if (ec == std::errc{}) {
        PRINT("from_chars('666') = {}", v);
    }
}
