#include <iostream>
#include <utility>
#include <vector>

// [C++11]
// 移动构造
// 头文件 <utility>

// 有时候我们会遇到这样一种情况，我们用对象a初始化对象b，后对象a我们就不在使用了，
// 但是对象a的空间还在呀（在析构之前），既然拷贝构造函数，实际上就是把a对象的内容复制一份到b中，
// 那么为什么我们不能直接使用a的空间呢？这样就避免了新的空间的分配，大大降低了构造的成本。
// 这就是移动构造函数设计的初衷

void std_move()
{
	std::string st = "移动构造是C++11标准中提供的一种新的构造方法";

	std::vector<std::string> vc;
	vc.push_back(std::move(st));

	std::cout << "vc[0]=" << vc[0] << std::endl;

	if (!st.empty()){
        std::cout << "st=" << st << std::endl;
    }

    // 输出的结果为：
    // vc[0]=移动构造是C++11标准中提供的一种新的构造方法
}