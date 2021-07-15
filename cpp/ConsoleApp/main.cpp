#include <iostream>

#include "base/ConfigFile.h"
#include "mysqlx/Mysqlx.h"

#include "std_atomic.cpp"

int main() {

    // ConfigFile config("./config.conf");

    // std::cout << "--------------=" << std::endl;

    // std::cout << "ip=" << config.get("ip") << std::endl;
    // std::cout << "port=" << config.get("port") << std::endl;
    // std::cout << "port00=" << config.get("port00") << std::endl;

    // std::cout << "--------------=" << std::endl;

    // Mysqlx::getInstance().initialize("192.168.10.240", "test", 3306, "root", "123456");

    std_atomic_main();

    std::cout << "main end --------------" << " thread_id=" << std::this_thread::get_id() << std::endl;
    return 0;
}
