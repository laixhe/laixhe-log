#include "StdString.h"

StdString::StdString()
{
    // 字符串原始字面量 R
    // [C++11]
    const std::string fileDir = R"(D:\software\v)"; // 以前写法 "D:\\software\\v"
    std::cout << "fileDir: " << fileDir << std::endl;
    // [C++23]
    // std::print("fileDir: {}", fileDir);
    // std::println("fileDir: {}", fileDir); // 自动换行

    const std::string str = R"(
<html>
    <head>
        <title>字符串原始字面量 R</title>
    </head>
</html>)";
    std::cout << "str: " << str << std::endl;

    // 基本格式化
    // [C++20]
    std::string name = "laixhe";
    int age = 18;
    const std::string format1 = std::format("name={} age={}", name, age);
    std::cout << "format1: " << format1 << std::endl;
    // 使用位置参数
    const std::string format2 = std::format("age={1} name={0}", name, age);
    std::cout << "format2: " << format2 << std::endl;

}
