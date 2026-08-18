#ifndef CPPAPP_STDALGORITHM_H
#define CPPAPP_STDALGORITHM_H

// STL 算法：sort / find / count / transform / reverse / max_element。
// 对应 Go golog slice_test.go（slices 包）、Rust rustlog iterators.rs（适配器）。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdAlgorithm
{
    public:
    StdAlgorithm();
};


#endif //CPPAPP_STDALGORITHM_H
