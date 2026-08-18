#ifndef CPPAPP_STDBITS_H
#define CPPAPP_STDBITS_H

// 位运算：移位 / 与或异或 / 掩码 / std::bitset。
// 对应 Go/Rust 位运算、Python 位运算符。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdBits
{
    public:
    StdBits();
};


#endif //CPPAPP_STDBITS_H
