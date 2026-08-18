#include "StdMove.h"

#include <format>     // std::format [C++20]
#include <iostream>
#include <utility>    // std::move
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// ===== 演示类：统计拷贝/移动次数 =====
// 对应 Rust 所有权转移：C++ 的"移动"≈ Rust 的 move，但 C++ 移动后对象仍存在（空状态）
class Buffer
{
    public:
    // 构造函数
    explicit Buffer(int size) : data_(new int[size]), size_(size), id_(++s_id)
    {
        PRINT("构造 Buffer #{}（{} 个元素）", id_, size_);
    }

    // 拷贝构造（深拷贝：对应 Rust clone）
    Buffer(const Buffer& other) : data_(new int[other.size_]), size_(other.size_), id_(++s_id)
    {
        std::copy(other.data_, other.data_ + size_, data_);
        PRINT("拷贝构造 Buffer #{}（深拷贝，开销大）", id_);
    }

    // 移动构造（转移资源：对应 Rust move，不复制）
    Buffer(Buffer&& other) noexcept : data_(other.data_), size_(other.size_), id_(++s_id)
    {
        other.data_ = nullptr; // 原对象置空（对应 Rust 移动后原变量失效）
        other.size_ = 0;
        PRINT("移动构造 Buffer #{}（转移资源，零拷贝）", id_);
    }

    // 析构
    ~Buffer()
    {
        delete[] data_;
        PRINT("析构 Buffer #{}", id_);
    }

    int size() const { return size_; }

    private:
    int* data_ = nullptr;
    int size_ = 0;
    int id_ = 0;
    inline static int s_id = 0; // C++17 inline 静态成员
};

StdMove::StdMove()
{
    // ===== 1. 左值与右值 =====
    std::cout << "--- 左值与右值 ---" << std::endl;

    int x = 42; // x 是左值（有名字，可取地址）
    // 42 是右值（临时值）
    int& lref = x;  // 左值引用绑定左值
    // int& ref2 = 42; // ❌ 左值引用不能绑定右值
    const int& cref = 42; // const 左值引用可以绑定右值（临时对象生命周期延长）
    PRINT("左值: {}, const 引用绑右值: {}", lref, cref);

    // ===== 2. std::move：把左值转为右值，触发移动而非拷贝 =====
    std::cout << "--- std::move ---" << std::endl;

    // 场景：向 vector 添加元素，避免深拷贝（对应 Rust vec.push(已 move 的值)）
    std::vector<std::string> names;
    std::string name = "laixhe";
    names.push_back(std::move(name)); // 移动而非拷贝
    PRINT("移动后原字符串为空: '{}'", name); // ""（C++ 移动后原对象处于有效但未指定状态）

    // ===== 3. 移动 vs 拷贝的代价对比 =====
    std::cout << "--- 移动 vs 拷贝 ---" << std::endl;

    PRINT("--- 拷贝场景（深拷贝，开销大）---");
    Buffer b1(100);
    Buffer b2 = b1; // 拷贝构造：深拷贝

    PRINT("--- 移动场景（零拷贝）---");
    Buffer b3(100);
    Buffer b4 = std::move(b3); // 移动构造：转移资源
}
