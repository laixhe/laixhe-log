#ifndef CPPAPP_STDITER_H
#define CPPAPP_STDITER_H

// 迭代器基础：begin/end / 分类 / 反向 / 插入迭代器 / 迭代器工具。
// 对应 Rust rustlog iterators、Go golog iterators、C# IEnumerable。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdIter
{
    public:
    StdIter();
};


#endif //CPPAPP_STDITER_H
