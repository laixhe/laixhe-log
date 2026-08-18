#ifndef CPPAPP_STDCONST_H
#define CPPAPP_STDCONST_H

#include <iostream>

// 统一设计：静态方法逐个演示主题内容，main.cpp 中取消注释对应调用即可运行；
// 用类而非函数，便于集中组织同主题的多个示例。
class StdConst
{
public:
    // 常量指针
    static void ConstPointer();
    // 指针常量
    static void PointerConst();
    // 指向常量的指针常量
    static void ConstPointerConst();
};


#endif //CPPAPP_STDCONST_H