#include <iostream>
#include <chrono>

// [C++11]
// chrono 是一个时间库
// 头文件 <chrono>

void std_chrono(){

    // 获取当前时间
    std::chrono::system_clock::time_point now = std::chrono::system_clock::now();
    // 距离1970-01-01 00:00:00的纳秒数
    std::chrono::nanoseconds d = now.time_since_epoch();
    // 转换为秒数,会有精度损失
    std::chrono::seconds sec = std::chrono::duration_cast<std::chrono::seconds>(d);

    std::cout << "当前时间戳：" << sec.count() << std::endl;

    std::cout << "------------------" << std::endl;
    std::cout << "std_chrono end --------------" << std::endl;
}