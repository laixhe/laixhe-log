#include <iostream>
#include <functional>

// [C++11]
// std::function 是(可调用对象的包装器)，它最重要的功能是实现延时调用
// std::function<返回值类型(参数类型列表)> diy_name = 可调用对象
// 可做为回调函数使用，因为回调函数本身就是通过函数指针实现的，使用对象包装器可以取代函数指针的作用
//
// std::bind 用来将可调用对象与其参数一起进行绑定(绑定器)
// 绑定后的结果可以使用 std::function 进行保存，并延迟调用到任何我们需要的时候
// 绑定非类成员函数/变量
// auto f = std::bind(可调用对象地址, 绑定的参数/占位符);
// 绑定类成员函/变量
//auto f = std::bind(类函数/成员地址, 类实例对象地址, 绑定的参数/占位符);
//
// 头文件 <functional>

// 普通函数
void std_function_bind_func(){
    std::cout << __FUNCTION__ << std::endl;
}

// 静态类成员函数
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

// 做为回调函数使用

// 回调函数
class FunctionCallback {
public:
    // 构造函数参数是一个包装器对象
    FunctionCallback(const std::function<void()>& f) : callback(f){
    }

    void notify(){
        callback(); // 调用通过构造函数得到的函数指针
    }

private:
    std::function<void()> callback;

};

// 仿函数
class CallbackOperator {
public:
    void operator()(){
        std::cout << "CallbackOperator 做为回调函数..." << std::endl;
    }
};

void std_function_callback(){

    CallbackOperator callback;

    FunctionCallback fCallback(callback); // 仿函数通过包装器对象进行包装
    fCallback.notify();

}

// 函数指针(可调用对象) (不推荐)
/*
int print(int a, double b)
{
    cout << a << b << endl;
    return 0;
}
// 定义函数指针
int (*func)(int, double) = &print;
*/