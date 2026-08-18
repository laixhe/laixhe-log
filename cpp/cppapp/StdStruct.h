#ifndef CPPAPP_STDSTRUCT_H
#define CPPAPP_STDSTRUCT_H

// 结构体与联合：struct / 位域 / union / 内存对齐。
// 对应 Rust rustlog struct_enum 的 struct 部分、Go golog 的 type 示例。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdStruct
{
    public:
    StdStruct();
};


#endif //CPPAPP_STDSTRUCT_H
