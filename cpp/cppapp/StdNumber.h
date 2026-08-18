#ifndef CPPAPP_STDNUMBER_H
#define CPPAPP_STDNUMBER_H

// 数值类型进阶：格式化 / 整数溢出 / 类型转换。
// 对应 Rust rustlog/src/number.rs 与 Go golog 的数值示例。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdNumber
{
    public:
    StdNumber();
};


#endif //CPPAPP_STDNUMBER_H
