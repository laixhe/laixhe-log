#include <iostream>

auto add = [](int a, int b) -> int {
    return a + b;
};

auto run = []() {
    int a = 10;
    int b = 20;
    std::cout << (a & b) << std::endl; // 01010 & 10100 = 00000 = 0
    std::cout << (a | b) << std::endl; // 01010 | 10100 = 11110 = 30
    std::cout << (a ^ b) << std::endl; // 01010 ^ 10100 = 11110 = 30

    std::cout << (a << 2) << std::endl; // 00001010 << 2 = 00101000 = 30
    std::cout << (a >> 2) << std::endl; // 00001010 >> 2 = 00000010 = 2
};
