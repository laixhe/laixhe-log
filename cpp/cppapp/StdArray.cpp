#include "StdArray.h"

#include <array>      // std::array [C++11]
#include <format>     // std::format [C++20]
#include <iostream>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdArray::StdArray()
{
    // ===== 1. C 风格数组（定长，大小编译期确定）=====
    std::cout << "--- C 数组 ---" << std::endl;

    // 声明 + 初始化（对应 Go [3]int / Rust [i32; 3]）
    int arr[3] = {1, 2, 3};
    // 省略大小：编译器推断
    int arr2[] = {10, 20, 30, 40};
    PRINT("arr2 大小: {}，元素: {} {} {}", sizeof(arr2) / sizeof(arr2[0]), arr2[0], arr2[1], arr2[3]);

    // 遍历（范围 for，C++11）
    std::cout << "范围 for: ";
    for (int v : arr) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    // 数组传参退化为指针（与 Go/Rust 不同，需传大小）
    // 注意：C++ 数组默认按"指针"传递，不是值拷贝

    // ===== 2. std::array（C++11，封装数组，可传值、有 size 等）=====
    std::cout << "--- std::array ---" << std::endl;

    std::array<int, 3> a1 = {1, 2, 3};
    PRINT("size: {}, 首元素: {}", a1.size(), a1.front()); // 3 1

    // 越界安全：at() 抛异常（对应 Rust 越界 panic / Go 越界 panic）
    try {
        (void)a1.at(5);
    } catch (const std::out_of_range&) {
        PRINT("at(5) 越界抛异常");
    }

    // 值语义：赋值是整体拷贝（区别于 C 数组，对应 Rust Copy）
    std::array<int, 3> a2 = a1;
    a2[0] = 99;
    PRINT("拷贝后原数组不变: {}", a1[0]); // 1

    // ===== 3. 多维数组（对应 Rust 多维数组 / Python 嵌套 list）=====
    std::cout << "--- 多维数组 ---" << std::endl;

    // 2 行 3 列
    int matrix[2][3] = {{1, 2, 3}, {4, 5, 6}};
    PRINT("matrix[1][2] = {}", matrix[1][2]); // 6

    // 嵌套 std::array
    std::array<std::array<int, 2>, 2> m2 = {{{1, 2}, {3, 4}}};
    PRINT("std::array 嵌套: {}", m2[1][0]); // 3
}
