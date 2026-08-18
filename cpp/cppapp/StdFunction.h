#ifndef CPPAPP_STDFUNCTION_H
#define CPPAPP_STDFUNCTION_H

// 函数：重载 / 默认参数 / lambda / 引用参数。
// 对应 Go golog function_test.go、Rust rustlog function.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdFunction
{
    public:
    StdFunction();
};


#endif //CPPAPP_STDFUNCTION_H
