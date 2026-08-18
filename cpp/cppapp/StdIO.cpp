#include "StdIO.h"

#include <format>     // std::format [C++20]
#include <iomanip>    // setw / setprecision / setfill [格式化输出]
#include <iostream>
#include <sstream>    // std::stringstream（字符串流）
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdIO::StdIO()
{
    // ===== 1. 格式化输出：iomanip（对应 Go fmt 格式 / C printf）=====
    std::cout << "--- 格式化输出 ---" << std::endl;

    // 宽度与填充：setw 设置最小宽度，setfill 设置填充字符
    std::cout << "setw 左对齐: |" << std::left << std::setw(10) << "hello" << "|" << std::endl;
    // |hello     |
    std::cout << std::right; // 恢复右对齐

    // 整数补零：setfill + setw（对应 format {:08d} / Go %08d）
    std::cout << "补零: " << std::setfill('0') << std::setw(8) << 666 << std::setfill(' ') << std::endl;
    // 00000666

    // 精度：setprecision（对应 format {:.2f} / Go %.2f）
    std::cout << "精度: " << std::fixed << std::setprecision(2) << 88.888 << std::endl;
    // 88.89
    std::cout << std::defaultfloat; // 恢复默认

    // 进制：hex / oct / dec（对应 format {:x} {:o} / Go %x %o）
    std::cout << "十六进制: 0x" << std::hex << 666 << std::dec << std::endl;   // 0x29a
    std::cout << "八进制: 0o" << std::oct << 666 << std::dec << std::endl;     // 0o1232

    // ===== 2. 输入：std::cin（对应 Rust stdin / C scanf）=====
    // 交互式输入会阻塞程序，这里仅展示用法；运行时可取消注释体验
    std::cout << "--- 输入 ---" << std::endl;
    std::cout << "// int num; std::cin >> num; // 从标准输入读取整数" << std::endl;
    std::cout << "// 输入错误时：std::cin.fail() == true，需 clear() + ignore() 恢复" << std::endl;

    // ===== 3. 字符串流：stringstream（类型转换 + 拼接，对应 Go strconv / Rust parse）=====
    std::cout << "--- 字符串流 ---" << std::endl;

    // 数字 → 字符串
    std::stringstream ss;
    ss << 666 << "-" << 88.8;
    PRINT("拼接: {}", ss.str()); // 666-88.8

    // 字符串 → 数字（对应 Go strconv.Atoi / Rust parse）
    std::stringstream parser("123 45.67");
    int i = 0;
    double d = 0.0;
    parser >> i >> d;
    PRINT("解析: int={} double={}", i, d); // 123 45.67

    // 解析失败：流状态检查（对应 Go 解析错误 / Rust Err）
    std::stringstream bad("abc");
    int v = 0;
    bad >> v;
    PRINT("解析非数字失败: {}", bad.fail()); // true
}
