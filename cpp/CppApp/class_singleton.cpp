#include "class_singleton.h"

// 静态局部变量的单例模式

Singleton& Singleton::getInstance() {
    static Singleton instance;
    return instance;
}

void Singleton::doSomething(){
    std::cout << "Singleton::doSomething..." << std::endl;
}
