#include <iostream>
#include "Singleton.h"

int main() {

    Singleton& instance_1 = Singleton::getInstance();
    Singleton& instance_2 = Singleton::getInstance();
    Singleton::getInstance();
    Singleton::getInstance();

    instance_1.test();
    instance_2.test();
    instance_2.test();
    instance_2.test();
    instance_2.test();
    instance_1.test();

    return 0;
}
