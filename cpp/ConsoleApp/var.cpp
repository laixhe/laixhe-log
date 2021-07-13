#include <iostream>

// 区分左值与右值的便捷方法是：可以对表达式取地址（&）就是左值，否则为右值
// 所有有名字的变量或对象都是左值，而右值是匿名的

void var(){

    int a = 10;
    std::string b = "数据类型";

    // 字符串原始字面量 R
    // [C++11]
    std::string str = R"(D:\software\v)"; // 以前写法 "D:\\software\\v"
    std::cout << "str=" << str << std::endl;

    std::string str1 = R"(
<html>
    <head>
        <title>字符串原始字面量 R</title>
    </head>
</html>)";
    std::cout << "str1=" << str1 << std::endl;

    // decltype 返回数据类型
    // [C++11]
    decltype(a) c = 20; // int 类型 
    std::cout << "c=" << c << std::endl;
}

// 引用相关
void reference(){

    // 引用相当是变量的别名
    // 在定义的时候就必须初始化，一旦指向了某个变量，就不可以再改变量的本身引用

    int a = 10;
    int b = 30;

    // 定义一个引用
    int& refA = a;
    refA = 20;

    refA = b; // 是赋值操作，不是修改指向

    // 结果： a=20 refA=20
    std::cout << "a=" << a << " refA=" << refA << std::endl;

    // 不能进行赋值修改操作
    const int&  refB = a;
    // refB = 40; // error

}