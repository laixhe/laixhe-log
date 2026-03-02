#ifndef STDPTR_H
#define STDPTR_H

#include <memory>


// 智能指针 [C++11]
// #include <memory>

// 独享 std::unique_ptr  std::make_unique [C++14]  独占所有权，不可拷贝，可移动。管理单个动态对象的最佳默认选择
// 共享 std::shared_ptr  std::make_shared          共享所有权，引用计数。用于需要多个地方共享访问的对象
// 共享 std::weak_ptr                              配合 shared_ptr 使用，解决循环引用问题，不增加引用计数
//
// 空指针 nullptr [C++11]
//
// auto ptr = std::unique_ptr<A>(new A); // 不推荐
// auto ptr = std::make_unique<A>();     // 推荐
// std::unique_ptr<A> tem = ptr;         // 不允许移动，编译失败

#endif // STDPTR_H