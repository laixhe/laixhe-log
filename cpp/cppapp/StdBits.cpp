#include "StdBits.h"

#include <bitset>     // std::bitset [C++11]
#include <cstdint>    // uint8_t
#include <format>     // std::format [C++20]
#include <iostream>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdBits::StdBits()
{
    // ===== 1. 移位运算 =====
    std::cout << "--- 移位 ---" << std::endl;

    int n = 1;
    PRINT("1 << 3 = {}", n << 3);   // 8（左移 = 乘以 2^n）
    PRINT("16 >> 2 = {}", 16 >> 2); // 4（右移 = 除以 2^n）

    // 位移实现乘除 2（对应 Go/Rust 位运算优化）
    PRINT("5 * 2 = {}, 5 / 2 = {}", 5 << 1, 5 >> 1); // 10 2

    // ===== 2. 位逻辑：与 / 或 / 异或 / 取反 =====
    std::cout << "--- 位逻辑 ---" << std::endl;

    uint8_t a = 0b1100;
    uint8_t b = 0b1010;

    PRINT("a & b  = {:04b}", a & b);  // 1000（与：权限判断）
    PRINT("a | b  = {:04b}", a | b);  // 1110（或：合并权限）
    PRINT("a ^ b  = {:04b}", a ^ b);  // 0110（异或：翻转/去重）
    PRINT("~a     = {:08b}", static_cast<uint8_t>(~a)); // 11110011（取反）

    // ===== 3. 掩码操作：设置 / 清除 / 检查某一位（对应位标志位）=====
    std::cout << "--- 掩码 ---" << std::endl;

    uint8_t flags = 0b0000;
    constexpr uint8_t kRead = 1 << 0;  // 0b0001
    constexpr uint8_t kWrite = 1 << 1; // 0b0010
    constexpr uint8_t kExec = 1 << 2;  // 0b0100

    flags |= kRead | kWrite;  // 设置读+写权限
    PRINT("设置后: {:04b}", flags); // 0011

    PRINT("可读? {}", (flags & kRead) != 0);   // true
    PRINT("可执行? {}", (flags & kExec) != 0); // false

    flags &= ~kWrite; // 清除写权限
    PRINT("清除写后: {:04b}", flags); // 0001

    // 异或翻转某位
    flags ^= kExec; // 翻转执行位
    PRINT("翻转后: {:04b}", flags); // 0101

    // ===== 4. std::bitset：位集合（对应 Go math/bits / Python 位操作）=====
    std::cout << "--- std::bitset ---" << std::endl;

    std::bitset<8> bits(0b1100);
    PRINT("bitset: {}", bits.to_string());       // 00001100
    PRINT("count(1 的个数): {}", bits.count());  // 2
    PRINT("test(2): {}", bits.test(2));          // true（第 2 位是 1）

    bits.set(0); // 设置第 0 位
    bits.reset(1); // 清除第 1 位
    PRINT("set/reset 后: {}", bits.to_string()); // 00001101

    // 位翻转
    PRINT("flip 后: {}", bits.flip().to_string()); // 11110010
}
