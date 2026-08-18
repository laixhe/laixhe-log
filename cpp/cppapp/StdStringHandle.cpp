#include "StdStringHandle.h"

#include <algorithm>   // std::transform
#include <cctype>      // std::tolower / std::toupper
#include <format>      // std::format [C++20]
#include <iostream>
#include <map>         // 词频统计
#include <sstream>     // std::istringstream
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdStringHandle::StdStringHandle()
{
    // ===== 1. 查找（对应 Go strings.Index / Rust find）=====
    std::cout << "--- 查找 ---" << std::endl;

    const std::string s = "hello, world";
    // find：返回下标，npos 表示未找到
    PRINT("find('world'): {}", s.find("world"));            // 7
    PRINT("rfind('l'): {}", s.rfind('l'));                  // 10（从右查找）
    PRINT("find('xyz') = npos: {}", s.find("xyz") == std::string::npos); // true

    // 是否包含（C++23 contains）
    PRINT("contains('world'): {}", s.contains("world"));    // true

    // 判断前缀 / 后缀（C++20 starts_with / ends_with）
    PRINT("starts_with('he'): {}", s.starts_with("he"));    // true
    PRINT("ends_with('ld'): {}", s.ends_with("ld"));        // true

    // ===== 2. 子串与替换 =====
    std::cout << "--- 子串与替换 ---" << std::endl;

    PRINT("substr(7): {}", s.substr(7));        // world
    PRINT("substr(0, 5): {}", s.substr(0, 5));  // hello
    std::string replaced = s;
    PRINT("replace: {}", replaced.replace(0, 5, "Hi")); // Hi, world

    // ===== 3. 大小写转换（对应 Go strings.ToUpper / Rust to_uppercase）=====
    std::cout << "--- 大小写 ---" << std::endl;

    std::string mixed = "Hello World";
    std::string lower = mixed;
    std::transform(lower.begin(), lower.end(), lower.begin(), [](unsigned char c) {
        return static_cast<char>(std::tolower(c));
    });
    std::string upper = mixed;
    std::transform(upper.begin(), upper.end(), upper.begin(), [](unsigned char c) {
        return static_cast<char>(std::toupper(c));
    });
    PRINT("to_lower: {}", lower); // hello world
    PRINT("to_upper: {}", upper); // HELLO WORLD

    // ===== 4. 分割（对应 Go strings.Split / Python split）=====
    std::cout << "--- 分割 ---" << std::endl;

    // 用 istringstream 按空白分割（对应 Go strings.Fields）
    std::istringstream iss("1 2   3");
    std::string token;
    std::cout << "按空白分割: ";
    while (iss >> token) {
        std::cout << token << " ";
    }
    std::cout << std::endl; // 1 2 3

    // 用 find 按指定分隔符分割（对应 Go strings.Split("a,b,c", ",")）
    const std::string csv = "a,b,c";
    std::string rest = csv;
    std::cout << "按逗号分割: ";
    while (true) {
        const std::size_t pos = rest.find(',');
        if (pos == std::string::npos) {
            std::cout << rest;
            break;
        }
        std::cout << rest.substr(0, pos) << " ";
        rest = rest.substr(pos + 1);
    }
    std::cout << std::endl; // a b c

    // ===== 5. 词频统计（对应 Go map[string]int / TS Map）=====
    std::cout << "--- 词频统计 ---" << std::endl;

    const std::string sentence = "the quick brown fox jumps over the lazy dog the";
    std::istringstream words(sentence);
    std::map<std::string, int> freq;
    while (words >> token) {
        freq[token]++;
    }
    for (const auto& [word, count] : freq) {
        PRINT("{} = {}", word, count);
    }
    // the = 3, dog = 1, ...（map 自动按 key 排序）
}
