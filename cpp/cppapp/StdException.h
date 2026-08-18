#ifndef CPPAPP_STDEXCEPTION_H
#define CPPAPP_STDEXCEPTION_H

// 异常处理：try / catch / throw / 自定义异常。
// 对应 Rust rustlog error.rs（Result/?）、Go golog error_test.go（error/panic）。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdException
{
    public:
    StdException();
};


#endif //CPPAPP_STDEXCEPTION_H
