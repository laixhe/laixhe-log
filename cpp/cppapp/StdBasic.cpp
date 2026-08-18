#include "StdBasic.h"

#include <cstdint>   // int8_t 等定宽整型
#include <format>    // std::format [C++20]
#include <iostream>
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdBasic::StdBasic()
{
    // ===== 1. 基础数据类型（对应 Go 基本类型 / Rust 基础类型）=====
    std::cout << "--- 基础类型 ---" << std::endl;

    // 整型：int / short / long / long long（定宽整型见 cstdint）
    int age = 18;
    std::cout << "int: " << age << std::endl;

    // 定宽整型：int8_t / uint8_t / int32_t / uint64_t
    int8_t small = 100;
    uint64_t big = 18446744073709551615ULL;
    // 注意：int8_t 是 signed char 的别名，直接输出会显示为字符而非数字，
    // 所以打印时用 static_cast<int> 转成数字形式
    std::cout << "int8_t: " << static_cast<int>(small) << "  uint64_t: " << big << std::endl;

    // 浮点：float（单精度 4 字节）/ double（双精度 8 字节）
    float f = 3.14f;
    double d = 3.14159;
    std::cout << "float: " << f << "  double: " << d << std::endl;

    // 布尔 / 字符
    bool ok = true;
    char c = 'A';
    std::cout << "bool: " << ok << "  char: " << c << std::endl;

    // ===== 2. 变量声明方式 =====
    std::cout << "--- 变量声明 ---" << std::endl;

    // 方式1：传统声明
    int x = 10;
    // 方式2：auto 类型推断（对应 Go := / C# var，C++11 引入）
    auto y = 20;        // 推断为 int
    auto name = "laixhe"; // 推断为 const char*
    // 方式3：列表初始化（C++11，防止窄化转换）
    int z{30};
    std::cout << "x=" << x << " y=" << y << " z=" << z << " name=" << name << std::endl;

    // const 常量（对应 Go const / Rust const）
    const int MAX_POINTS = 100000;
    // MAX_POINTS = 1; // ❌ 常量不可修改
    std::cout << "const: " << MAX_POINTS << std::endl;

    // ===== 3. 类型转换（对应 Rust as / Go 强转）=====
    std::cout << "--- 类型转换 ---" << std::endl;

    double pi = 3.99;
    // 静态转换（编译期检查，推荐）：浮点转整型向零截断
    int truncated = static_cast<int>(pi);
    std::cout << "static_cast: " << truncated << std::endl; // 3

    // 隐式窄化（不推荐）：double → int 自动截断，可能丢失精度
    int implicit = pi; // 真正意义上的隐式转换（与上面的 static_cast 对比）
    std::cout << "隐式: " << implicit << std::endl;

    // ===== 4. 引用 &（对应 Rust 借用 / Go 引用）=====
    std::cout << "--- 引用 ---" << std::endl;

    int value = 42;
    int& ref = value; // 引用：value 的别名，不复制
    ref = 99;          // 通过引用修改原值
    std::cout << "引用修改: " << value << std::endl; // 99

    // const 引用：只读，不修改（对应 Rust & 只读借用）
    const int& cref = value;
    std::cout << "const 引用: " << cref << std::endl; // 99
}
