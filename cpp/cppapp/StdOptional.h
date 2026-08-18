#ifndef CPPAPP_STDOPTIONAL_H
#define CPPAPP_STDOPTIONAL_H

// 可选值与联合类型（C++17）：optional / variant / tuple。
// 对应 Rust rustlog struct_enum.rs（Option）、Go golog 多返回值、Java Optional。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdOptional
{
    public:
    StdOptional();
};


#endif //CPPAPP_STDOPTIONAL_H
