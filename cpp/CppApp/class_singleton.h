#ifndef CLASSS_INGLETON_H
#define CLASSS_INGLETON_H

#include <iostream>

// 静态局部变量的单例模式

// [C++11]
class Singleton {
public:
    // 由于静态成员函数 getInstance() 返回的是一个引用
    // 因此用户无法通过构造函数或析构函数来直接创建或销毁 Singleton 对象
    // 这保证了 Singleton 类只有一个实例，是线程安全的
    // 在 C++11 标准中，静态局部变量的初始化被保证是线程安全的
    static Singleton& getInstance();

    void doSomething();

    Singleton(const Singleton&) = delete;            // 禁止拷贝构造函数
    Singleton& operator=(const Singleton&) = delete; // 禁止赋值操作符
private:
    // 将其构造函数、析构函数、拷贝构造函数和赋值操作符全部设为私有
    // 以防止用户在类外部创建多个实例或拷贝、复制实例

    Singleton() = default;  // 私有构造函数
    ~Singleton() = default; // 私有析构函数
};

// [C++17]
inline Singleton& getSingletonInstance() {
    return Singleton::getInstance();
}

#endif //CLASSS_INGLETON_H
