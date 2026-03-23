#include "Chrono.h"

Chrono::Chrono()
{
    auto today = std::chrono::year{2026}/3/4;
    std::cout << std::format("今天是: {:%Y-%m-%d %A}\n", today);

    std::chrono::zoned_time local_now{std::chrono::current_zone(), std::chrono::system_clock::now()};
    std::cout << std::format("本地当前时间: {}\n", local_now);
}
