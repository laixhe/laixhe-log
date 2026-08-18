#include "StdAlgorithm.h"

#include <algorithm>  // sort / find / count / transform / reverse / max_element
#include <format>     // std::format [C++20]
#include <iostream>
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 打印 vector 的辅助 lambda
static void printVec(const std::vector<int>& v)
{
    for (int n : v) {
        std::cout << n << " ";
    }
    std::cout << std::endl;
}

StdAlgorithm::StdAlgorithm()
{
    // ===== 1. 查找：find / find_if（对应 Go slices.Contains / Rust iter().find）=====
    std::cout << "--- 查找 ---" << std::endl;

    std::vector<int> nums{1, 2, 3, 4, 5};
    auto it = std::find(nums.begin(), nums.end(), 3);
    PRINT("find(3) 找到: {}", it != nums.end()); // true

    // find_if：按条件查找（对应 Rust find + 闭包）
    auto it2 = std::find_if(nums.begin(), nums.end(), [](int n) { return n > 4; });
    PRINT("find_if(>4): {}", *it2); // 5

    // ===== 2. 计数：count / count_if（对应 Rust count / Python list.count）=====
    std::cout << "--- 计数 ---" << std::endl;

    std::vector<int> dup{1, 2, 2, 3, 2};
    PRINT("count(2): {}", std::count(dup.begin(), dup.end(), 2)); // 3
    PRINT("count_if(偶数): {}", std::count_if(dup.begin(), dup.end(), [](int n) { return n % 2 == 0; })); // 4

    // ===== 3. 排序与反转（对应 Go slices.Sort/Reverse / Rust sort）=====
    std::cout << "--- 排序 ---" << std::endl;

    std::vector<int> order{3, 1, 2};
    std::sort(order.begin(), order.end()); // 升序
    std::cout << "排序后: ";
    printVec(order); // 1 2 3

    std::reverse(order.begin(), order.end()); // 反转
    std::cout << "反转后: ";
    printVec(order); // 3 2 1

    // sort 按自定义规则（lambda，对应 Go slices.SortFunc）
    std::vector<int> custom{5, 2, 8, 1};
    std::sort(custom.begin(), custom.end(), [](int a, int b) { return a > b; }); // 降序
    std::cout << "降序: ";
    printVec(custom); // 8 5 2 1

    // ===== 4. 转换：transform（对应 Rust map / C# Select）=====
    std::cout << "--- transform ---" << std::endl;

    std::vector<int> src{1, 2, 3, 4, 5};
    std::vector<int> squares(src.size());
    std::transform(src.begin(), src.end(), squares.begin(), [](int n) { return n * n; });
    std::cout << "平方: ";
    printVec(squares); // 1 4 9 16 25

    // ===== 5. 极值：max_element / min_element（对应 Rust max / C# Max）=====
    std::cout << "--- 极值 ---" << std::endl;

    auto maxIt = std::max_element(src.begin(), src.end());
    auto minIt = std::min_element(src.begin(), src.end());
    PRINT("max: {}, min: {}", *maxIt, *minIt); // 5 1
}
