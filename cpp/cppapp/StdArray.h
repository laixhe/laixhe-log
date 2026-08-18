#ifndef CPPAPP_STDARRAY_H
#define CPPAPP_STDARRAY_H

// 数组基础：C 数组 / std::array / 多维数组。
// 对应 Go golog array_test.go、Rust rustlog array_map.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdArray
{
    public:
    StdArray();
};


#endif //CPPAPP_STDARRAY_H
