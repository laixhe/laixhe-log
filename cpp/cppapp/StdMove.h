#ifndef CPPAPP_STDMOVE_H
#define CPPAPP_STDMOVE_H

// 移动语义：左值/右值 / std::move / 移动构造与拷贝构造对比。
// 对应 Rust rustlog ownership.rs（所有权/移动语义）。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdMove
{
    public:
    StdMove();
};


#endif //CPPAPP_STDMOVE_H
