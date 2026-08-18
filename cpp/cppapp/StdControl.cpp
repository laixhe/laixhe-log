#include "StdControl.h"

#include <format>    // std::format [C++20]
#include <iostream>
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdControl::StdControl()
{
    // ===== 1. if / else if / else（对应 Go if / Rust if）=====
    std::cout << "--- if ---" << std::endl;

    int score = 85;
    if (score >= 90) {
        std::cout << "优秀" << std::endl;
    } else if (score >= 60) {
        std::cout << "及格" << std::endl;
    } else {
        std::cout << "不及格" << std::endl;
    }

    // ===== 2. for 循环（对应 Go for / Rust for）=====
    std::cout << "--- for ---" << std::endl;

    // 写法1：传统三段式
    std::cout << "1..3: ";
    for (int i = 1; i <= 3; i++) {
        std::cout << i << " ";
    }
    std::cout << std::endl;

    // 写法2：范围 for（C++11，对应 Rust for / C# foreach）
    std::cout << "数组遍历: ";
    std::vector<int> nums{10, 20, 30};
    for (int n : nums) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    // continue / break
    std::cout << "continue 跳过偶数: ";
    for (int i = 1; i <= 6; i++) {
        if (i % 2 == 0) {
            continue;
        }
        std::cout << i << " ";
    }
    std::cout << std::endl;

    // ===== 3. while / do-while（对应 Go while 等价 / Rust while）=====
    std::cout << "--- while ---" << std::endl;

    int n = 3;
    std::cout << "while: ";
    while (n > 0) {
        std::cout << n << " ";
        n--;
    }
    std::cout << std::endl;

    // do-while：至少执行一次（对应 Go do {} while 无直接等价 / Java do-while）
    int m = 0;
    do {
        std::cout << "do-while 至少执行一次: " << m << std::endl;
        m++;
    } while (m < 1);

    // ===== 4. switch（对应 Go switch / Rust match，C++ 需要 break）=====
    std::cout << "--- switch ---" << std::endl;

    int day = 3;
    // 注意：C++ 的 switch 不会自动跳出，必须 break（区别于 Go/Rust）
    switch (day) {
        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
            PRINT("工作日（周{}）", day);
            break;
        case 6:
        case 7:
            PRINT("休息日（周{}）", day);
            break;
        default:
            PRINT("未知（{}）", day);
    }
}
