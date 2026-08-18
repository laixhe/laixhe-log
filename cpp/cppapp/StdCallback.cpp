#include "StdCallback.h"

#include <format>     // std::format [C++20]
#include <functional> // std::function / std::bind [C++11]
#include <iostream>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 普通函数（可被 std::function 包装）
static int Add(int a, int b)
{
    return a + b;
}

// 回调参数：类似 Go 的 func 参数 / Rust 的 Fn 闭包参数 / C 的 qsort 比较函数
static void Process(int x, const std::function<int(int, int)>& fn)
{
    PRINT("Process({}) -> {}", x, fn(x, 10));
}

StdCallback::StdCallback()
{
    // ===== 1. std::function：可调用对象的统一包装（对应 Go 函数值 / Rust 闭包）=====
    std::cout << "--- std::function ---" << std::endl;

    // 包装普通函数
    std::function<int(int, int)> f = Add;
    PRINT("函数指针: {}", f(3, 4)); // 7

    // 重新赋值为 lambda（对应 Go 闭包 / Rust 闭包捕获）
    f = [](int a, int b) { return a * b; };
    PRINT("lambda: {}", f(3, 4)); // 12

    // 包装成员函数
    struct Calc
    {
        int Scale = 10;
        int Apply(int v) const { return v * Scale; }
    };
    Calc calc{};
    std::function<int(const Calc&, int)> memFn = &Calc::Apply;
    PRINT("成员函数: {}", memFn(calc, 5)); // 50

    // ===== 2. 函数指针（对应 C 函数指针 / Go 函数值底层）=====
    std::cout << "--- 函数指针 ---" << std::endl;

    int (*fp)(int, int) = Add; // 传统函数指针
    PRINT("函数指针调用: {}", fp(8, 9)); // 17

    // ===== 3. 回调参数（对应 Go sort.Slice / Rust iterator adaptor / C qsort）=====
    std::cout << "--- 回调参数 ---" << std::endl;

    Process(5, Add);                        // Process(5) -> 15
    Process(6, [](int a, int b) { return a * b; }); // Process(6) -> 60

    // ===== 4. lambda 捕获与 std::bind（对应 Go 闭包 / Rust move 闭包）=====
    std::cout << "--- 捕获与 bind ---" << std::endl;

    // 捕获外部变量形成闭包
    int base = 100;
    auto closure = [base](int x) { return base + x; };
    PRINT("闭包捕获: {}", closure(23)); // 123

    // std::bind 预绑定参数，生成新的可调用对象（对应 Go 部分应用）
    auto addTen = std::bind(Add, 10, std::placeholders::_1);
    PRINT("bind 预绑定: {}", addTen(5)); // 15

    // std::function 存储闭包后再调用（对应 Rust Box<dyn Fn>）
    std::function<int(int)> stored = [base](int x) { return base * x; };
    PRINT("function 存闭包: {}", stored(3)); // 300
}
