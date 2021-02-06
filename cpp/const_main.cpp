#include <iostream>

// const即修饰指针，又修饰常量
// 指针的指向 和 指针指向的值不可以改
// const int * const p = &a;

// 常量指针
void constPointer(){

    int a = 10;
    int b = 20;

    // 常量指针
    // 指针的指向可以改，但指针指向的值不可以改
    const int * p = &a;
    p = &b;     // 正确，指针指向可以改
    // *p = 30; // 错误，指针指向的值不可以改

    std::cout << *p << std::endl;
}

// 指针常量
void pointerConst(){

    int a = 10;
    int b = 20;

    // 指针常量
    // 指针的指向不可以改，但指针指向的值可以改
    int * const p = &a;
    //p = &b; // 错误，指针指向不可以改
    *p = 30; // 正确，指针指向的值可以改

    std::cout << *p << std::endl;
}