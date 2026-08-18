#include "StdIter.h"

#include <algorithm> // std::copy
#include <format>   // std::format [C++20]
#include <iostream>
#include <iterator> // std::advance / distance / next / prev / back_inserter / inserter
#include <list>     // std::list（双向迭代器）
#include <vector>   // std::vector（随机访问迭代器）

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 打印容器内容（用迭代器遍历，对应 Go for range / Rust iter().for_each）
static void Show(const std::vector<int>& v, const char* label)
{
    std::cout << label << ": [";
    for (auto it = v.begin(); it != v.end(); ++it) {
        std::cout << *it << (std::next(it) != v.end() ? ", " : "");
    }
    std::cout << "]" << std::endl;
}

StdIter::StdIter()
{
    // ===== 1. 迭代器基本操作：begin/end、解引用 *、自增 ++ =====
    std::cout << "--- 基本操作 ---" << std::endl;

    std::vector<int> nums{10, 20, 30, 40, 50};

    // 迭代器类似"指针"：*it 取元素，++it 前进，!= 判断是否到末尾
    auto it = nums.begin();
    PRINT("begin() = {}", *it);       // 10
    ++it;
    PRINT("++ 后 = {}", *it);          // 20
    it += 2;                          // 随机访问迭代器支持跳跃
    PRINT("+= 2 后 = {}", *it);        // 40
    auto end = nums.end();
    PRINT("it != end: {}", it != end); // true

    // ===== 2. 迭代器分类：不同容器提供不同能力的迭代器 =====
    std::cout << "--- 迭代器分类 ---" << std::endl;

    // vector：随机访问迭代器（支持 +n / [] / --）
    PRINT("vector 随机访问: nums[3] = {}", *std::next(nums.begin(), 3)); // 40

    // list：双向迭代器（只能 ++ / --，不能 +n）
    std::list<int> lst{1, 2, 3, 4};
    auto lit = lst.begin();
    std::advance(lit, 2);             // 双向迭代器用 advance 走 n 步
    PRINT("list 双向访问: {}", *lit); // 3

    // ===== 3. 反向迭代器：rbegin/rend（对应 Rust .rev() / Go 逆序遍历）=====
    std::cout << "--- 反向迭代器 ---" << std::endl;

    std::cout << "反向: [";
    for (auto rit = nums.rbegin(); rit != nums.rend(); ++rit) {
        std::cout << *rit << (std::next(rit) != nums.rend() ? ", " : "");
    }
    std::cout << "]" << std::endl; // [50, 40, 30, 20, 10]

    // ===== 4. 迭代器工具：distance / advance / next / prev =====
    std::cout << "--- 迭代器工具 ---" << std::endl;

    PRINT("distance(begin, end) = {}", std::distance(nums.begin(), nums.end())); // 5
    PRINT("*prev(end) = {}", *std::prev(nums.end())); // 50（最后一个元素）

    // ===== 5. 插入迭代器：向容器自动插入（对应 Go append / Rust collect）=====
    std::cout << "--- 插入迭代器 ---" << std::endl;

    // back_inserter：push_back 包装，与 std::copy 配合最常用
    std::vector<int> copy;
    std::copy(nums.begin(), nums.end(), std::back_inserter(copy));
    Show(copy, "back_inserter 复制"); // [10, 20, 30, 40, 50]

    // inserter：在指定位置插入（每次插入后迭代器自动推进，保持顺序）
    std::vector<int> mid;
    std::copy(nums.begin(), nums.end(), std::inserter(mid, mid.begin()));
    Show(mid, "inserter 头部插入"); // [10, 20, 30, 40, 50]

    // ===== 6. 遍历方式对比：下标 / 迭代器 / range-for =====
    std::cout << "--- 遍历方式 ---" << std::endl;

    // 方式 1：下标（仅随机访问容器可用）
    int sumIndex = 0;
    for (size_t i = 0; i < nums.size(); ++i) sumIndex += nums[i];

    // 方式 2：迭代器（所有容器通用）
    int sumIter = 0;
    for (auto i = nums.begin(); i != nums.end(); ++i) sumIter += *i;

    // 方式 3：range-for（C++11，编译期展开为迭代器，推荐写法）
    int sumRange = 0;
    for (int n : nums) sumRange += n;

    PRINT("下标={} 迭代器={} range-for={}", sumIndex, sumIter, sumRange); // 150 150 150
}
