#include "std_chrono.h"

#include <iostream>
#include <chrono>
#include <iomanip>
#include <sstream>
#include <thread>
#include <format>

// std::chrono::system_clock          系统的实际时间，可能会受到系统时间调整的影响
// std::chrono::steady_clock          稳定的时钟，时间从不调整
// std::chrono::high_resolution_clock 提供最小的可表示的时间间隔
// std::chrono::time_point            表示特定的时间点
// std::chrono::duration              表示时间的长度

StdChrono::StdChrono()
{
    // 1小时
    std::chrono::hours hour{1};
    // 1分钟
    std::chrono::minutes minute{1};
    // 1秒钟
    std::chrono::seconds second{1};
    // 1000毫秒（即1秒）时间
    std::chrono::milliseconds millisecond{1000};
    // 1000微秒（即1毫秒）时间
    std::chrono::microseconds microsecond{1000};

    // 1000毫秒（即1秒）时间
    std::chrono::duration<int, std::ratio<1, 1000>> one_thousand_milliseconds(1000);
    // std::ratio<60, 1> minutes;         // 1分钟
    // std::ratio<60 * 60> hours;         // 1小时
    // std::ratio<1, 1000> milliseconds;  // 1毫秒

    // 时间单位转换
    std::chrono::seconds sec(6006);
    std::chrono::minutes min = std::chrono::duration_cast<std::chrono::minutes>(sec); // 将秒转换为分钟
    std::chrono::hours hr = std::chrono::duration_cast<std::chrono::hours>(min);      // 将分钟转换为小时

    // 执行延迟 1 秒后的操作(睡眠 1秒)
    // std::this_thread::sleep_for(std::chrono::seconds(1));

    std::cout << "1小时: " << hour.count() << std::endl;
    std::cout << "1分钟: " << minute.count() << std::endl;
    std::cout << "1秒钟: " << second.count() << std::endl;
    std::cout << "1000毫秒: " << millisecond.count() << std::endl;
    std::cout << "1000微秒: " << microsecond.count() << std::endl;
    std::cout << "1000毫秒(即1秒)时间: " << one_thousand_milliseconds.count() << std::endl;

    std::cout << "sec: " << sec.count() << std::endl;
    std::cout << "min: " << min.count() << std::endl;
    std::cout << "hr: " << hr.count() << std::endl;
}

// 当前时间戳
void StdChrono::Timestamp()
{
    // 获取当前系统时间点
    std::chrono::system_clock::time_point now = std::chrono::system_clock::now();
    // 转换为旧式接口，单位:秒
    std::time_t currentTime = std::chrono::system_clock::to_time_t(now);

    std::cout << "当前时间戳: " << currentTime << std::endl;
}

// 格式化时间
void StdChrono::Format()
{
    // 获取当前系统时间点
    std::chrono::time_point<std::chrono::system_clock> now = std::chrono::system_clock::now();
    // 转换为旧式接口，单位:秒
    std::time_t currentTime = std::chrono::system_clock::to_time_t(now);

    // // 将 time_t 对象转换为本地时间
    // struct tm cutTm = {0};
    // std::tm *localTime = localtime_r(&currentTime, &cutTm);
    // // 格式化输出日期和时间
    // std::ostringstream oss;
    // oss << std::put_time(localTime, "%Y-%m-%d %H:%M:%S");
    // std::cout << "格式化时间: " << oss.str() << std::endl;

    // [C++20]
    std::cout << "Date: " << std::format("{:%Y-%m-%d %H:%M:%S}", now) << std::endl;
}

// 计算加减
void StdChrono::Compute()
{
    // 获取当前时间点
    auto startTime = std::chrono::system_clock::now();
    // 在当前时间上加上1秒
    auto endTime = startTime + std::chrono::seconds(1);
    // 计算两个时间点之间的持续时间
    std::chrono::duration<double> elapsedSeconds = endTime - startTime;

    // 进行强制类型转换，将浮点数结果转换为整数
    int second = static_cast<int>(elapsedSeconds.count());
    std::cout << "持续时间: " << second << " 秒" << std::endl;
}