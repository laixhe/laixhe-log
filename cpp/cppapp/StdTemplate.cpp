#include "StdTemplate.h"

#include <concepts>   // std::integral 概念 [C++20]
#include <format>     // std::format [C++20]
#include <iostream>
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// ===== 函数模板（对应 Rust 泛型函数 / Go [T] / Java <T>）=====
// 一个函数适用于多种类型
template <typename T>
T max_of_two(T a, T b)
{
    return a > b ? a : b;
}

// ===== 类模板（对应 Rust 泛型结构体 / Java 泛型类）=====
template <typename K, typename V>
struct Pair
{
    K key;
    V value;
};

// ===== 概念约束（C++20，对应 Rust trait bound / Java extends）=====
// 模板参数限定为整型
template <std::integral T>
T square(T x)
{
    return x * x;
}

StdTemplate::StdTemplate()
{
    // ===== 1. 函数模板 =====
    std::cout << "--- 函数模板 ---" << std::endl;

    // 类型推断：自动选择 int / double 版本（对应 Rust 泛型推断）
    PRINT("max(3, 7) = {}", max_of_two(3, 7));       // 7
    PRINT("max(1.5, 2.5) = {}", max_of_two(1.5, 2.5)); // 2.5
    PRINT("max('a', 'z') = {}", max_of_two('a', 'z')); // z

    // ===== 2. 类模板 =====
    std::cout << "--- 类模板 ---" << std::endl;

    Pair<std::string, int> p1{"age", 18};
    Pair<int, std::string> p2{1, "one"};
    PRINT("Pair: {}={}", p1.key, p1.value); // age=18
    PRINT("Pair: {}={}", p2.key, p2.value); // 1=one

    // ===== 3. 概念约束（C++20）=====
    std::cout << "--- 概念约束 ---" << std::endl;

    PRINT("square(5) = {}", square(5)); // 25（int 满足 std::integral）
    // 说明：std::integral 只接受整型；若传浮点会编译报错，下面这行是错误演示
    // PRINT("square(2.5) = {}", square(2.5)); // ❌ double 不满足约束
}
