#ifndef CPPAPP_STDIO_H
#define CPPAPP_STDIO_H

// 输入输出流：std::cin 输入 / iomanip 格式化 / stringstream 字符串流。
// 对应 Rust rustlog 无（标准输入输出）、Go fmt 包、C# Console。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdIO
{
    public:
    StdIO();
};


#endif //CPPAPP_STDIO_H
