#ifndef CPPAPP_STDCLASS_H
#define CPPAPP_STDCLASS_H

// 类与对象：封装 / 构造函数 / 析构 / 继承 / 多态。
// 对应 Go golog type_test.go（结构体/接口）、Rust rustlog struct_enum.rs + generic_trait.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdClass
{
    public:
    StdClass();
};


#endif //CPPAPP_STDCLASS_H
