#include <iostream>

// 匿名函数(Lambda 表达式) [C++11]
void lambdaFunction(){

    // Lambda 表达式
    // 完整结构: [capture list](params list) mutable exception -> return type {function body}
    //            capture list 捕获列表(外部变量)
    //            params list  形参列表，不能使用默认参数，不能省略参数名
    //            mutable      是否可以修改按值传递进来的拷贝（注意是能修改拷贝，而不是值本身）
    //            exception    异常设定
    //            return type  返回值类型
    //            function body 函数体
    //            
    //            捕获列表 []: 捕获一定范围内的变量
    //            参数列表 (): 和普通函数的参数列表一样，如果没有参数参数列表可以省略不写
    //            auto f = [](){return 1;}	// 没有参数, 参数列表为空
    //            auto f = []{return 1;}		// 没有参数, 参数列表省略不写
    //
    //            捕获列表含义
    //                [] - 不捕捉任何变量
    //                [&] - 捕获外部作用域中所有变量，并作为引用在函数体内使用 (按引用捕获)
    //                [=] - 捕获外部作用域中所有变量，并作为副本在函数体内使用 (按值捕获)（拷贝的副本在匿名函数体内部是只读的）
    //                [=, &foo] - 按值捕获外部作用域中所有变量，并按照引用捕获外部变量 foo
    //                [bar] - 按值捕获 bar 变量，同时不捕获其他变量
    //                [&bar] - 按引用捕获 bar 变量，同时不捕获其他变量
    //                [this] - 捕获当前类中的 this 指针
    //
    //
    auto add = [](int a, int b) -> int {
        return a + b;
    };

    // 当没有参数时，可缩写为 []{ ... }
    auto run = []{
        int a = 10;
        int b = 20;
        
        std::cout<< "lambdaFunction - (a & b) = " << (a & b) << std::endl; // 01010 & 10100 = 00000 = 0
        std::cout<< "lambdaFunction - (a | b) = " << (a | b) << std::endl; // 01010 | 10100 = 11110 = 30
        std::cout<< "lambdaFunction - (a ^ b) = " << (a ^ b) << std::endl; // 01010 ^ 10100 = 11110 = 30

        std::cout<< "lambdaFunction - (a << 2) = " << (a << 2) << std::endl; // 00001010 << 2 = 00101000 = 40
        std::cout<< "lambdaFunction - (a >> 2) = " << (a >> 2) << std::endl; // 00001010 >> 2 = 00000010 = 2
    };

    std::cout << "lambdaFunction - add = " << add(1, 3) << std::endl;

    run();

    // 执行当前函数
    ([]{
        std::cout << "lambdaFunction... " << std::endl;
    })();

    // 可省略参数类型 [C++14]
    auto func = [](auto v1, auto v2) {
        return v1 + v2;
    };
    std::cout << "func... " << func(22, 33) << std::endl;
}

// 可调用对象 - 仿函数
class operatorFunction {
public:
    // ()操作符重载
    int operator() (int a){
        std::cout << __FUNCTION__ << "(" << a << ")" << std::endl;
        return a;
    }
};

void operatorFunctionMain(){
    operatorFunction f;
    f(1);
}

