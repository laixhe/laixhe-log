#include <iostream>

// sizeof
// 本质：不是函数，而是一个操作符
// 当统计类型占的内存空间时候，必须要加 小括号
// 当统计变量占内存空间时候，可以不加小括号
// sizeof 返回值类型是 无符号整型 unsigned int

void sizeofTest(){

    int a[] = {1,2,3,4};
    int len = sizeof(a) / sizeof(a[0]); // 得到数组的容量(长度)
    for( int index=0; index < len; ++index ){
        if (a[index] == 3) {
            std::cout << a[index] << std::endl;
            break;
        }
    }

}