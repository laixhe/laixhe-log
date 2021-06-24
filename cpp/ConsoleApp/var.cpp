#include <iostream>

#include "var.h"

// 引用相关
void reference(){

    // 引用相当是变量的别名
    // 在定义的时候就必须初始化，一旦指向了某个变量，就不可以再改变量的本身引用

    int a = 10;

    // 定义一个引用
    int& refA = a;
    refA = 20;

    // 结果： a=20 refA=20
    std::cout << "a=" << a << " refA=" << refA << std::endl;
}