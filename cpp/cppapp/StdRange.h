#ifndef CPPAPP_STDRANGE_H
#define CPPAPP_STDRANGE_H

// 迭代器/范围专题：std::ranges 视图（transform / filter / take / drop / zip / join）。
// 对应 Rust rustlog/src/iterators.rs 与 Java Stream。
// 视图是惰性的：只有在迭代时才真正计算，链式组合零拷贝。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdRange
{
    public:
    StdRange();
};


#endif //CPPAPP_STDRANGE_H
