#ifndef CPPAPP_STDSTRING_H
#define CPPAPP_STDSTRING_H

#include <iostream>
#include <string>
// [C++20]
#include <format>
// [C++23]
// #include <print>

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdString
{
    public:
    StdString();
};


#endif //CPPAPP_STDSTRING_H