#ifndef CPPAPP_STDCAST_H
#define CPPAPP_STDCAST_H

// 类型转换：static_cast / dynamic_cast / const_cast / reinterpret_cast。
// 对应 Go 显式类型转换（T(x)）、Rust as / TryFrom、C# 强制转换 / as。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdCast
{
    public:
    StdCast();
};


#endif //CPPAPP_STDCAST_H
