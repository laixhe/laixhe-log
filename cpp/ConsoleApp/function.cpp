#include <iostream>

#include "function.h"

// 匿名函数
void lambdaFunction(){

    auto add = [](int a, int b) -> int {
        return a + b;
    };

    auto run = []() {
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
}


