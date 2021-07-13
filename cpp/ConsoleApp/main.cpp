#include <iostream>

#include "base/ConfigFile.h"
#include "mysqlx/Mysqlx.h"

//#include "std_thread.cpp"
#include "std_find_find_if.cpp"

int main() {

    // ConfigFile config("./config.conf");

    // std::cout << "--------------=" << std::endl;

    // std::cout << "ip=" << config.get("ip") << std::endl;
    // std::cout << "port=" << config.get("port") << std::endl;
    // std::cout << "port00=" << config.get("port00") << std::endl;

    // std::cout << "--------------=" << std::endl;

    // Mysqlx::getInstance().initialize("192.168.10.240", "test", 3306, "root", "123456");

    //std_thread_my_print();
    std_find_find_if();

    std::cout << "main end --------------" << std::endl;
    return 0;
}
