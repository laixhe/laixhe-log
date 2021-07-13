#include <iostream>
#include <functional>

// [C++11]
// std::function 是可调用对象的包装器，它最重要的功能是实现延时调用
// 头文件 <functional>

void std_function_bind_func(){
    std::cout << __FUNCTION__ << std::endl;
}

class StdFunctionBindFoo {
public:
    static int foo_func(int a) {
        std::cout << __FUNCTION__ << "(" << a << ") ->: ";
        return a;
    }
};

// 仿函数
class StdFunctionBindBar {
public:
    int operator() (int a){
        std::cout << __FUNCTION__ << "(" << a << ") ->: ";
        return a;
    }
};

class StdFunctionBindA {
public:
    int i = 0; // [C++11]允许非静态成员进行初始化

    void output(int x, int y){
        std::cout << "StdFunctionBindA x=" << x << " y=" << y << std::endl;
    }

};

void std_function_bind(){

    // std::bind 用来将可调用对象与其参数一起进行绑定，
    // 绑定后可以使用 std::function 进行保存，并延迟到我们需要的时候调用

    // 绑定普通函数
    std::function<void()> fr1 = std_function_bind_func;
    fr1();

    // 绑定类的静态成员函数
    std::function<int(int)> fr2 = StdFunctionBindFoo::foo_func;
    std::cout << fr2(100) << std::endl;

    // 绑定仿函数
    StdFunctionBindBar bar;
    std::function<int(int)> fr3 = bar;
    std::cout << fr3(200) << std::endl;

    StdFunctionBindA a;
    // 绑定成员函数，保存为仿函数
    std::function<void(int, int)> a_f = std::bind(&StdFunctionBindA::output, &a, std::placeholders::_1, std::placeholders::_2);
    // 调用成员函数
    a_f(1, 2);

    // 绑定成员变量
    std::function<int&(void)> a_i = std::bind(&StdFunctionBindA::i, &a);
    a_i() = 100;// 对成员变量进行赋值
    std::cout << "a_i=" << a.i << std::endl;

    std::cout << "std_function_bind end --------------" << std::endl;
}