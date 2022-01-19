#include <iostream>
#include <vector>
#include <algorithm>

// [C++11]
// std::find 可用于查找容器中是否存在某个特定值，对于基本类型的容器用法
// 头文件 <algorithm>

void std_find_find_if(){
    std::vector<int> arrInt{11, 22, 33, 44};

    int num_to_find = 33; // 要查找的元素,类型要与 vector<> 类型一致
    
    auto arr_int_i = std::find(arrInt.begin(), arrInt.end(), num_to_find);
    if ( arr_int_i == arrInt.end() ) {
        std::cout << "在 arrInt 中没找到：" << num_to_find << std::endl;
    } else {
        std::cout << "在 arrInt 中找到：" << num_to_find << std::endl;
    }
    
}