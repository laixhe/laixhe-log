#include "StdConst.h"

// 常量指针(锁数据，不锁指针)
void StdConst::ConstPointer()
{
    int a = 10;
    int b = 20;
    // 向右：看到指针 * 说明 p 是一个指针
    // 向左：看到 const int 说明指向的是 const int
    // 结论：指向常量的指针（指针可改，指向的数据不可改）
    const int* p = &a;
    p = &b;     // OK: 可以修改指针本身
    // *p = 30; // Error: 不能修改指向的数据

    std::cout << *p << std::endl;
}

// 指针常量(锁指针，不锁数据)
void StdConst::PointerConst()
{
    int a = 10;
    int b = 20;
    // 向右：看到 const 说明 p 是常量
    // 向左：看到 int* 说明是一个整数指针
    // 结论：指针常量（指针不可改，指向的数据可改）
    int* const p = &a;
    //p = &b; // Error: 不能修改指针本身
    *p = 30; // OK: 可以修改指向的数据

    std::cout << *p << std::endl;
}

// 指向常量的指针常量(锁指针，锁数据)
void StdConst::ConstPointerConst()
{
    int a = 10;
    int b = 20;
    // 向右：看到 const 说明 p 是常量
    // 向左：看到 const int*，说明是一个指向常量的指针
    // 结论：指向常量的指针常量（指针和指向的数据都不可改）
    const int* const p = &a;
    //p = &b; // Error: 不能修改指针本身
    // *p = 30; // Error: 不能修改指向的数据
}
