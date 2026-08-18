#include "StdFunction.h"

#include <algorithm>  // std::for_each
#include <format>     // std::format [C++20]
#include <iostream>
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// ===== 函数重载（对应 Rust 无重载 / Java 方法重载）=====
// 重载规则：参数个数或类型不同，返回类型不算
static int add(int a, int b)
{
    return a + b;
}

static double add(double a, double b) // 重载：参数类型不同
{
    return a + b;
}

// ===== 默认参数（对应 Python 默认参数 / C# 可选参数）=====
static int multiply(int a, int b = 2) // b 有默认值
{
    return a * b;
}

// ===== 引用参数：函数内修改外部变量（对应 Rust &mut 借用 / Go 指针）=====
static void doubleValue(int& n) // 引用参数，不复制
{
    n *= 2;
}

StdFunction::StdFunction()
{
    // ===== 1. 函数重载 =====
    std::cout << "--- 重载 ---" << std::endl;

    PRINT("add(int): {}", add(1, 2));     // 3（调用 int 版本）
    PRINT("add(double): {}", add(1.5, 2.5)); // 4（调用 double 版本）

    // ===== 2. 默认参数 =====
    std::cout << "--- 默认参数 ---" << std::endl;

    PRINT("multiply(3) = {}", multiply(3));   // 6（b 用默认值 2）
    PRINT("multiply(3, 5) = {}", multiply(3, 5)); // 15

    // ===== 3. lambda 表达式（对应 Rust 闭包 / Go 匿名函数，C++11）=====
    std::cout << "--- lambda ---" << std::endl;

    // 基本 lambda：[捕获](参数) -> 返回类型 { 体 }
    auto add_one = [](int x) { return x + 1; };
    PRINT("lambda: {}", add_one(5)); // 6

    // 捕获外部变量（[=] 按值捕获，[&] 按引用捕获，对应 Rust move/借用）
    int factor = 3;
    auto multiply_by = [=](int x) { return x * factor; }; // 按值捕获 factor
    PRINT("lambda 捕获: {} * {} = {}", factor, 4, multiply_by(4)); // 12

    // lambda 作为参数：配合标准库算法（对应 Rust 迭代器闭包 / C# LINQ）
    std::vector<int> nums{1, 2, 3, 4, 5};
    std::cout << "for_each 平方: ";
    std::for_each(nums.begin(), nums.end(), [](int& n) { n *= n; });
    for (int n : nums) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 1 4 9 16 25

    // ===== 4. 引用参数 =====
    std::cout << "--- 引用参数 ---" << std::endl;

    int value = 10;
    doubleValue(value); // 传入引用，函数内修改
    PRINT("doubleValue(10) 后: {}", value); // 20
}
