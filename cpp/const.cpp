#include <iostream>

// 在 C语言 中
// 1. 外部文件可用，为外部连接
// 2. const 修饰全局变量为只读，内存空间在文字常量区(只读)，不能间接修改
// 3. const 修饰局部变量为只读，内存空间栈区(可读可写)，可间接修改

// 在 cpp 中
// 1. 只作用于当前文件，为内部连接
// 2. 如果必须用在其它文件使用，那要加上 extern 修饰后就转换成了外接链接
// 3. 对于基础类型，系统不会创建空间，会放到符号表中 (如：const int data=10)
// 4. 是否为 const 常量分配内存空间，是要依赖于如何使用这个常量 (
//       如：仅仅使用值那么就不必创建内存
//           对一个 const 取地址或者把它定义为 extern 则会创建内存空间
//     )
// 5. 当以变量的形式对 const 修饰的变量初始化(如：const int a=b)，也会分配内存空间，当不会放到符号表中
// 6. 对于自定义数据类型，比如 类对象、结构体，那么也会分配内存

// 指针的指向 和 指针指向的值不可以改
// const int * const p = &a;

// 使用
void constInit(){
    // 对于基础类型，系统不会创建空间，会放到符号表中
    const int data = 10;
    //data = 10; // error 因为是只读的不能修改 (因为在 符号表中)

    // 对 data 取地址的时候，系统会对 data 创建内存空间
    int* p = (int*)&data;
    *p = 200; // 空间内容修改成功
    std::cout << "*p=" << *p << std::endl; // 结果：200

    // 这里还是会在 符号表中 取值
    // 结果：10
    std::cout << "data=" << data << std::endl;

    std::cout << "-----------" << std::endl;

    int b = 20;
    const int a = b; // 系统直接为 a 开辟空间，而不会把 a 放入符号表中
    int* pa = (int*)&a;
    *pa = 300; // 空间内容修改成功
    std::cout << "a=" << a << std::endl;     // 结果：300
    std::cout << "*pa=" << *pa << std::endl; // 结果：300
}

// 常量指针
void constPointer(){

    int a = 10;
    int b = 20;

    // 常量指针
    // 指针的指向可以改，但指针指向的值不可以改
    const int* p = &a;
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
    int* const p = &a;
    //p = &b; // 错误，指针指向不可以改
    *p = 30; // 正确，指针指向的值可以改

    std::cout << *p << std::endl;
}