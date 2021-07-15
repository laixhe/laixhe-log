#include <iostream>
#include <thread>
#include <atomic>

// [C++11]
// 原子库为细粒度的原子操作提供组件，允许无锁并发编程
// 头文件 <atomic>

std::atomic<int> std_atomic_main_g_i = 0;

void std_atomic_main_print(){
    for(int i=0; i<100000; ++i) {
        ++std_atomic_main_g_i;
        // 不可以为： std_atomic_main_g_i = std_atomic_main_g_i + 1
        // 可以为：   std_atomic_main_g_i += 1
    }
}

void std_atomic_main(){
    std::thread obj1(std_atomic_main_print);
    std::thread obj2(std_atomic_main_print);

    obj1.join();
    obj2.join();

    std::cout << "std_atomic_main_g_i=" << std_atomic_main_g_i << std::endl;

    std::cout << "std_atomic_main end..." << std::endl;
}