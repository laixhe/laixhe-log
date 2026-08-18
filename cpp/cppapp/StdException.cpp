#include "StdException.h"

#include <format>     // std::format [C++20]
#include <iostream>
#include <stdexcept>  // std::runtime_error
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// ===== 自定义异常（对应 Rust 自定义错误类型 / Python 异常类）=====
class DivideError : public std::runtime_error
{
    public:
    explicit DivideError(const std::string& msg) : std::runtime_error(msg) {}
};

// ===== 抛异常函数（对应 Rust return Err / Python raise）=====
static int safeDivide(int a, int b)
{
    if (b == 0) {
        throw DivideError(std::format("不能除以 0：{}/{}", a, b));
    }
    return a / b;
}

StdException::StdException()
{
    // ===== 1. try / catch：捕获标准异常（对应 Python try/except / Rust match Err）=====
    std::cout << "--- try/catch ---" << std::endl;

    try {
        int result = safeDivide(10, 0); // 抛 DivideError
        PRINT("结果: {}", result);        // 不会执行
    } catch (const DivideError& e) {     // 捕获自定义异常（对应 Python except 特定类型）
        PRINT("捕获自定义异常: {}", e.what());
    }

    // ===== 2. 正常路径不抛异常 =====
    PRINT("safeDivide(10, 2) = {}", safeDivide(10, 2)); // 5

    // ===== 3. 标准库异常：stoi 解析失败（对应 Rust ? 传播 / Go error）=====
    std::cout << "--- 标准库异常 ---" << std::endl;

    try {
        (void)std::stoi("not_a_number"); // 抛 std::invalid_argument
    } catch (const std::invalid_argument& e) {
        PRINT("stoi 解析失败: {}", e.what());
    }

    // ===== 4. 多异常捕获（对应 Python 多个 except）=====
    std::cout << "--- 多异常 ---" << std::endl;

    try {
        (void)std::string().at(5); // 越界：抛 std::out_of_range（(void) 表示忽略返回值）
    } catch (const std::out_of_range& e) {
        PRINT("越界异常: {}", e.what());
    } catch (const std::exception& e) { // 兜底：捕获所有 std::exception 子类
        PRINT("其他异常: {}", e.what());
    }

    // ===== 5. 异常安全说明 =====
    // C++ 异常与 Rust/Go 差异：
    // - Rust 用 Result（显式返回值），C++ 用异常（隐式向上传播）
    // - 现代 C++ 用 RAII 保证异常时资源自动释放（智能指针/容器）
}
