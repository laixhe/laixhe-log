#include <iostream>

#include "function.h"

// 匿名函数 [C++11]
void lambdaFunction(){

    // Lambda 表达式
    // 完整结构: [capture list](params list) mutable exception -> return type {function body}
    //            capture list 捕获外部变量列表
    //            params list  形参列表，不能使用默认参数，不能省略参数名
    //            mutable      用来说用是否可以修改捕获的变量
    //            exception    异常设定
    //            return type  返回值类型
    //            function body 函数体
    //            

    auto add = [](int a, int b) -> int {
        return a + b;
    };

    // 当没有参数时，可缩写为 []{ ... }
    auto run = []{
        int a = 10;
        int b = 20;
        
        std::cout<< "lambdaFunction - (a & b) = " << (a & b) << std::endl; // 01010 & 10100 = 00000 = 0
        std::cout<< "lambdaFunction - (a | b) = " << (a | b) << std::endl; // 01010 | 10100 = 11110 = 30
        std::cout<< "lambdaFunction - (a ^ b) = " << (a ^ b) << std::endl; // 01010 ^ 10100 = 11110 = 30

        std::cout<< "lambdaFunction - (a << 2) = " << (a << 2) << std::endl; // 00001010 << 2 = 00101000 = 40
        std::cout<< "lambdaFunction - (a >> 2) = " << (a >> 2) << std::endl; // 00001010 >> 2 = 00000010 = 2
    };

    std::cout << "lambdaFunction - add = " << add(1, 3) << std::endl;

    run();

    // 执行当前函数
    ([]{
        std::cout << "lambdaFunction... " << std::endl;
    })();

    // 可省略参数类型 [C++14]
    auto func = [](auto v1, auto v2) {
        return v1 + v2;
    };
    std::cout << "func... " << func(22, 33) << std::endl;
}


