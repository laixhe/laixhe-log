#ifndef CPPAPP_STDBASIC_H
#define CPPAPP_STDBASIC_H

// 基础类型与变量：整型/浮点/bool/char/auto/类型转换/引用。
// 对应 Go golog basic_test.go、Rust rustlog basic.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdBasic
{
    public:
    StdBasic();
};


#endif //CPPAPP_STDBASIC_H
