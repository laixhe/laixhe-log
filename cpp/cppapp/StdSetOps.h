#ifndef CPPAPP_STDSETOPS_H
#define CPPAPP_STDSETOPS_H

// 集合运算：去重 / 交集 / 并集 / 差集。
// 对应 Go golog unique_test.go、Python set 运算。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdSetOps
{
    public:
    StdSetOps();
};


#endif //CPPAPP_STDSETOPS_H
