#ifndef CPPAPP_STDENUM_H
#define CPPAPP_STDENUM_H

// 枚举：enum class 强类型枚举 / switch 配合 / 底层类型。
// 对应 Rust rustlog struct_enum.rs 的枚举、Go golog 的 iota。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdEnum
{
    public:
    StdEnum();
};


#endif //CPPAPP_STDENUM_H
