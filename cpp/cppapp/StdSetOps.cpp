#include "StdSetOps.h"

#include <algorithm>  // std::unique / set_intersection / set_union / set_difference
#include <format>     // std::format [C++20]
#include <iostream>
#include <iterator>   // std::back_inserter
#include <set>
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdSetOps::StdSetOps()
{
    // ===== 1. 去重：std::unique（相邻去重，对应 Go slices.Compact）=====
    std::cout << "--- 相邻去重 unique ---" << std::endl;

    std::vector<int> nums{1, 1, 2, 3, 3, 3, 4};
    auto last = std::unique(nums.begin(), nums.end()); // 相邻重复合并，返回新末尾
    nums.erase(last, nums.end());                      // 删除多余元素
    std::cout << "相邻去重: ";
    for (int n : nums) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 1 2 3 4

    // 全部去重（先排序再 unique，对应 Python set()）
    std::vector<int> unsorted{3, 1, 2, 1, 3, 4};
    std::sort(unsorted.begin(), unsorted.end());
    unsorted.erase(std::unique(unsorted.begin(), unsorted.end()), unsorted.end());
    std::cout << "排序后去重: ";
    for (int n : unsorted) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 1 2 3 4

    // ===== 2. 用 std::set 自动去重且有序（对应 Python set / Rust HashSet）=====
    std::cout << "--- std::set 去重 ---" << std::endl;

    std::set<int> s{3, 1, 2, 3, 1};
    std::cout << "set 内容: ";
    for (int n : s) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 1 2 3（自动去重 + 升序）

    // ===== 3. 交集 / 并集 / 差集（对应 Python & | - / Rust 集合运算）=====
    std::cout << "--- 集合运算 ---" << std::endl;

    std::set<int> a{1, 2, 3, 4};
    std::set<int> b{3, 4, 5, 6};

    // 交集：同时属于 a 和 b（对应 Python a & b）
    std::vector<int> inter;
    std::set_intersection(a.begin(), a.end(), b.begin(), b.end(), std::back_inserter(inter));
    std::cout << "交集: ";
    for (int n : inter) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 3 4

    // 并集：a 和 b 的合并（对应 Python a | b）
    std::vector<int> uni;
    std::set_union(a.begin(), a.end(), b.begin(), b.end(), std::back_inserter(uni));
    std::cout << "并集: ";
    for (int n : uni) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 1 2 3 4 5 6

    // 差集：属于 a 但不属于 b（对应 Python a - b）
    std::vector<int> diff;
    std::set_difference(a.begin(), a.end(), b.begin(), b.end(), std::back_inserter(diff));
    std::cout << "差集(a-b): ";
    for (int n : diff) {
        std::cout << n << " ";
    }
    std::cout << std::endl; // 1 2
}
