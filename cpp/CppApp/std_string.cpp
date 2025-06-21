#include "std_string.h"

StdString::StdString(){
}

void StdString::Var(){

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

}
