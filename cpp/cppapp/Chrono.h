#ifndef CPPAPP_CHRONO_H
#define CPPAPP_CHRONO_H

// 处理时间和日期
// [C++ 11]
//
// 三大核心概念
// duration   时间段
// time_point 时间点
// clock      时钟

#include <iostream>
#include <chrono>
#include <format>

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class Chrono
{
    public:
    Chrono();
};


#endif //CPPAPP_CHRONO_H