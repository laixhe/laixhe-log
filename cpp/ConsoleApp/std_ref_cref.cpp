#include <iostream>
#include <functional>

// [C++11]
// std::ref  用于包装按引用传递的值。 
// std::cref 用于包装按const引用传递的值
// 头文件 <functional>

// 主要是考虑函数式编程（ std::bind ）在使用时，是对参数直接拷贝，而不是引用

void std_ref_cref_int(int& n1, int& n2, const int& n3){
    std::cout << "std_ref_cref_int: " << n1 << ' ' << n2 << ' ' << n3 << '\n';

    ++n1;
    ++n2;
}
void std_ref_cref(){

    int n1 = 0, n2 = 0, n3 = 0;

    std::function<void()> bound_f = std::bind(std_ref_cref_int, n1, std::ref(n2), std::cref(n3));

    n1 = 10;
    n2 = 11;
    n3 = 12;

    bound_f();

    std::cout << "std_ref_cref: " << n1 << ' ' << n2 << ' ' << n3 << '\n';

    // 结果：
    // std_ref_cref_int: 0 11 12
    // std_ref_cref:     10 12 12
    // 上述代码在执行 std::bind 后，在函数 std_ref_cref_int() 中 n1 的值仍然是n1、n2、n3改成了修改的值。
    // 说明 std::bind 使用的是参数的拷贝而不是引用。具体为什么std::bind不使用引用，可能确实有一些需求，
    // 使得 C++11 的设计者认为默认应该采用拷贝，如果使用者有需求，加上 std::ref 即可。

    std::cout << "std_ref_cref end --------------" << std::endl;
}