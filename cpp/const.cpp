#include <iostream>


// 在 C语言 中
// 外部文件可用，为外部连接
// const 修饰全局变量为只读，内存空间在文字常量区(只读)，不能间接修改
// const 修饰局部变量为只读，内存空间栈区(可读可写)，可间接修改

// 在 cpp 中
// 只作用于当前文件，为内部连接
// 如果必须用在其它文件使用，那要加上 extern 修饰后就转换成了外接链接

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