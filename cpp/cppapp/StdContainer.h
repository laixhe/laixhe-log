#ifndef CPPAPP_STDCONTAINER_H
#define CPPAPP_STDCONTAINER_H

// 容器进阶：vector / map / set / deque / list / priority_queue / 环形缓冲 / LRU / 去重。
// 对应 Go golog 的 container/list、container/ring、container/heap、lru_test.go、
// slice_test.go、map_test.go、unique_test.go 与 Rust array_map.rs 的进阶部分。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdContainer
{
    public:
    StdContainer();
};


#endif //CPPAPP_STDCONTAINER_H
