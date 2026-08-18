#ifndef CPPAPP_STDTEMPLATE_H
#define CPPAPP_STDTEMPLATE_H

// 模板（泛型）：函数模板 / 类模板 / 概念约束（C++20）。
// 对应 Go golog generic_test.go、Rust rustlog generic_trait.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdTemplate
{
    public:
    StdTemplate();
};


#endif //CPPAPP_STDTEMPLATE_H
