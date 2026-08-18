#include "StdRandom.h"

#include <algorithm>  // std::shuffle
#include <format>     // std::format [C++20]
#include <iostream>
#include <random>     // std::random_device / mt19937 / 分布
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdRandom::StdRandom()
{
    // ===== 1. 随机源：random_device（硬件随机数，每次不同）=====
    std::cout << "--- 随机源与生成器 ---" << std::endl;

    std::random_device rd; // 真随机种子源（对应 Python os.urandom）
    std::mt19937 gen(rd()); // 梅森旋转生成器（标准建议，代替 rand()）

    // ===== 2. 整数分布：uniform_int_distribution（对应 Python randint）=====
    std::cout << "--- 整数分布 ---" << std::endl;

    std::uniform_int_distribution<int> dice(1, 6); // 1..6
    std::cout << "掷骰子 5 次: ";
    for (int i = 0; i < 5; i++) {
        std::cout << dice(gen) << " ";
    }
    std::cout << std::endl; // 每次运行不同

    // ===== 3. 浮点分布：uniform_real_distribution（对应 Python uniform）=====
    std::cout << "--- 浮点分布 ---" << std::endl;

    std::uniform_real_distribution<double> real(0.0, 1.0); // 0..1
    PRINT("随机小数: {:.3f}", real(gen)); // 0.xxx

    // ===== 4. 正态分布（对应 numpy random.normal）=====
    std::cout << "--- 正态分布 ---" << std::endl;

    std::normal_distribution<double> normal(0.0, 1.0); // 均值 0 标准差 1
    PRINT("正态采样: {:.3f}", normal(gen));

    // ===== 5. 打乱：std::shuffle（对应 Python random.shuffle / Rust rand shuffle）=====
    std::cout << "--- 打乱 shuffle ---" << std::endl;

    std::vector<int> nums{1, 2, 3, 4, 5};
    std::shuffle(nums.begin(), nums.end(), gen);
    std::cout << "打乱后: ";
    for (int n : nums) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 顺序随机

    // ===== 6. 从容器随机选一个（对应 Python random.choice）=====
    std::cout << "--- 随机选择 ---" << std::endl;

    std::vector<std::string> fruits{"苹果", "香蕉", "橘子"};
    std::uniform_int_distribution<std::size_t> pick(0, fruits.size() - 1);
    PRINT("随机水果: {}", fruits[pick(gen)]);
}
