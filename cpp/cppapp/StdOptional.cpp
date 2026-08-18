#include "StdOptional.h"

#include <format>     // std::format [C++20]
#include <iostream>
#include <optional>   // std::optional [C++17]
#include <string>
#include <tuple>      // std::tuple [C++11]
#include <variant>    // std::variant [C++17]

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 返回 optional：可能没有结果（对应 Rust Option / Go (value, error)）
static std::optional<int> safeDivide(int a, int b)
{
    if (b == 0) {
        return std::nullopt; // 无值（对应 Rust None / Go nil error）
    }
    return a / b; // 有值（对应 Rust Some）
}

StdOptional::StdOptional()
{
    // ===== 1. std::optional：可选值（对应 Rust Option<T> / Java Optional）=====
    std::cout << "--- optional ---" << std::endl;

    auto ok = safeDivide(10, 2);
    auto fail = safeDivide(10, 0);

    PRINT("has_value: {} {}", ok.has_value(), fail.has_value()); // true false
    PRINT("value: {}, 默认值: {}", *ok, fail.value_or(-1));      // 5 -1

    // 有值才处理（对应 Rust if let Some / Python 判断 None）
    if (ok) {
        PRINT("if 判断有值: {}", *ok); // 5
    }

    // ===== 2. std::variant：多类型联合（对应 Rust enum / Go interface）=====
    std::cout << "--- variant ---" << std::endl;

    std::variant<int, double, std::string> v = 42;   // 当前持有 int
    PRINT("当前类型 index: {}", v.index());           // 0

    v = "hello";                                     // 改为持有 string
    // holds_alternative：判断当前类型（对应 Rust match / Go 类型断言）
    PRINT("是 string: {}", std::holds_alternative<std::string>(v)); // true

    // get 获取值（类型错误会抛 bad_variant_access）
    PRINT("get string: {}", std::get<std::string>(v)); // hello

    // get_if：安全获取，返回指针（对应 Rust match 安全解构）
    if (const auto* d = std::get_if<double>(&v)) {
        PRINT("double: {}", *d);
    } else {
        PRINT("当前不是 double");
    }

    // ===== 3. std::tuple：元组（对应 Rust tuple / Go 多返回值 / Python tuple）=====
    std::cout << "--- tuple ---" << std::endl;

    auto tup = std::make_tuple("laixhe", 18, 88.8);
    PRINT("get<0>: {}, get<1>: {}", std::get<0>(tup), std::get<1>(tup)); // laixhe 18

    // 结构化绑定（C++17，对应 Rust 解构 / Go 多返回值解构）
    auto [name, age, score] = tup;
    PRINT("解构: {} {} {}", name, age, score);

    // 函数返回多值（对应 Go 多返回值）
    auto divMod = [](int a, int b) { return std::make_tuple(a / b, a % b); };
    auto [q, r] = divMod(10, 3);
    PRINT("10/3 商 {} 余 {}", q, r); // 3 1
}
