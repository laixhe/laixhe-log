#include <iostream>

class ClassStatic {
public:
    // 静态变量
    static int m_test;
    // 静态方法
    static void run(){
        std::cout << "ClassStatic::run() : " << m_test << std::endl;
        ++m_test;
    }
};

// 初始化类的静态成员
int ClassStatic::m_test = 0;

void ClassStaticMain(){
    ClassStatic::run();
    ClassStatic::run();
    ClassStatic::run();
    ClassStatic::run();
}