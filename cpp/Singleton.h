//
// Created by laixhe on 2021/1/18.
//

#ifndef UNTITLED_SINGLETON_H
#define UNTITLED_SINGLETON_H

// 单例模式
// C++11 版本最简洁的跨平台方案
class Singleton {
public:
    // 禁用 拷贝构造函数
    Singleton(const Singleton&)=delete;
    // 禁用 =运算符重载
    Singleton& operator=(const Singleton&)=delete;

    // 注意返回的是引用
    static Singleton& getInstance();

    void test();

// 将构造和析构函数私有化
// 防止外部访问
private:
    Singleton();
    ~Singleton();

    int i;
};


#endif //UNTITLED_SINGLETON_H
