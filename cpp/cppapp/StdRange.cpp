#include "StdRange.h"

#include <algorithm>  // std::ranges::any_of / partition_copy
#include <cstddef>
#include <format>     // std::format [C++20]
#include <iostream>
#include <numeric>    // std::accumulate
#include <optional>   // std::optional（filter_map 演示）
#include <ranges>     // views [C++20/23]
#include <string>
#include <vector>

namespace views = std::ranges::views;

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl
#define PRINTF(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__)

StdRange::StdRange()
{
    // ===== 1. map / filter：变换与过滤 =====
    std::cout << "--- map / filter ---" << std::endl;

    // map：平方（views::iota(1, 11) 表示 1..10，半开区间）
    auto squares = views::iota(1, 11) | views::transform([](int x) { return x * x; });
    std::cout << "1..10 平方: ";
    for (const int x : squares) PRINTF("{} ", x);
    std::cout << std::endl;

    // filter：长度≤3 的单词（视图不复制元素）
    const std::vector<std::string> words{"rust", "go", "python", "java", "c++", "js"};
    auto short_words = words | views::filter([](const std::string& w) { return w.size() <= 3; });
    std::cout << "长度≤3 的单词: ";
    for (const auto& w : short_words) PRINTF("{} ", w);
    std::cout << std::endl;

    // filter_map：把能解析为数字的挑出来（transform + filter has_value）
    const std::vector<std::string> strs{"123", "abc", "456", "not_a_num", "789"};
    auto nums = strs
        | views::transform([](const std::string& s) -> std::optional<int> {
              try { return std::stoi(s); } catch (...) { return std::nullopt; }
          })
        | views::filter([](const auto& o) { return o.has_value(); })
        | views::transform([](const auto& o) { return *o; });
    std::cout << "filter_map 选出合法数字: ";
    for (const int n : nums) PRINTF("{} ", n);
    std::cout << std::endl;

    // ===== 2. take / drop：取前 n 个 / 跳过前 n 个 =====
    std::cout << "--- take / drop ---" << std::endl;
    auto seq = views::iota(1, 11); // 1..10
    std::cout << "take(3): ";
    for (const int x : seq | views::take(3)) PRINTF("{} ", x);
    std::cout << std::endl;
    std::cout << "drop(7): ";
    for (const int x : seq | views::drop(7)) PRINTF("{} ", x);
    std::cout << std::endl;

    // ===== 3. zip / join：配对与展平 [C++23] =====
    std::cout << "--- zip / join ---" << std::endl;
    const std::vector<std::string> names{"Alice", "Bob", "Charlie"};
    const std::vector<int> scores{95, 87, 92};
    std::cout << "zip 配对: ";
    for (const auto& [name, score] : views::zip(names, scores)) {
        PRINTF("({},{}) ", name, score);
    }
    std::cout << std::endl;

    const std::vector<std::vector<int>> nested{{1, 2}, {3, 4, 5}, {6}};
    std::cout << "join 展平: ";
    for (const int x : nested | views::join) PRINTF("{} ", x);
    std::cout << std::endl;

    // ===== 4. reduce / any_of / all_of（终结操作）=====
    std::cout << "--- reduce / any / all ---" << std::endl;
    const std::vector<int> v{3, 1, 4, 1, 5, 9, 2, 6};

    // reduce 累加（对应 fold）
    auto range = views::iota(1, 11); // 先保存视图再取迭代器（避免重复构造临时对象）
    const int sum = std::accumulate(range.begin(), range.end(), 0);
    PRINT("accumulate 1..10 = {}", sum); // 55

    PRINT("any > 10? {}", std::ranges::any_of(v, [](int x) { return x > 10; }));
    PRINT("all > 0?  {}", std::ranges::all_of(v, [](int x) { return x > 0; }));

    // ===== 5. partition：按条件分成两组（对应 partition）=====
    std::cout << "--- partition ---" << std::endl;
    std::vector<int> even;
    std::vector<int> odd;
    std::ranges::partition_copy(v, std::back_inserter(even), std::back_inserter(odd),
        [](int x) { return x % 2 == 0; });
    std::cout << "偶=";
    for (const int x : even) PRINTF("{} ", x);
    std::cout << " 奇=";
    for (const int x : odd) PRINTF("{} ", x);
    std::cout << std::endl;

    // ===== 6. 综合实战：R&D 部门 30 岁以上平均月薪 =====
    std::cout << "--- 综合实战：平均月薪 ---" << std::endl;
    struct Employee {
        std::string dept;
        int age;
        int salary;
    };
    const std::vector<Employee> staff{
        {"R&D", 28, 30000}, {"R&D", 35, 45000}, {"R&D", 42, 60000},
        {"HR", 32, 18000},  {"R&D", 25, 22000}, {"Sale", 38, 25000},
    };

    auto query = staff
        | views::filter([](const Employee& e) { return e.dept == "R&D"; })   // 先筛选部门
        | views::filter([](const Employee& e) { return e.age >= 30; })       // 再筛选年龄
        | views::transform(&Employee::salary);                                // 提取月薪

    int total = 0;
    std::size_t count = 0;
    for (const int s : query) {
        total += s;
        count++;
    }
    if (count > 0) {
        PRINT("R&D 30+ 员工平均月薪: {} 元/月", total / count); // (45000+60000)/2 = 52500
    }
}
