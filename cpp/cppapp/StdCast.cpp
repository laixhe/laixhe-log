#include "StdCast.h"

#include <bit>      // std::bit_cast [C++20]
#include <cstdint>   // uintptr_t / uint32_t
#include <format>   // std::format [C++20]
#include <iostream>
#include <memory>   // std::unique_ptr

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 演示类放入匿名命名空间：内部链接，避免与 StdClass.cpp 的同名类（Animal/Dog/Cat）
// 发生 ODR 冲突（跨翻译单元同名不同类型的未定义行为）。
namespace
{
// 多态基类（dynamic_cast 需要虚函数）
class Animal
{
    public:
    virtual ~Animal() = default;                       // 虚析构：多态析构正确
    virtual const char* Sound() const { return "..."; }
};

class Dog : public Animal
{
    public:
    const char* Sound() const override { return "汪汪"; }
    void Fetch() const {}
};

class Cat : public Animal
{
    public:
    const char* Sound() const override { return "喵喵"; }
};
} // namespace

StdCast::StdCast()
{
    // ===== 1. static_cast：编译期检查的显式转换（最常用，对应 Go T(x) / Rust as）=====
    std::cout << "--- static_cast ---" << std::endl;

    // 数值转换：double → int 会截断（对应 Go int(f)）
    double pi = 3.99;
    PRINT("3.99 → int: {}", static_cast<int>(pi)); // 3（截断）
    PRINT("int → char: {}", static_cast<char>(65)); // A

    // 类型提升/降级是显式的，不会静默丢失精度
    int big = 300;
    PRINT("300 → uint8_t: {}", static_cast<unsigned char>(big)); // 44（溢出回绕）

    // 向上转型（派生类 → 基类）：安全
    Dog dog;
    Animal& animal = static_cast<Animal&>(dog); // 引用绑定基类
    PRINT("向上转型后调用虚函数: {}", animal.Sound()); // 汪汪（仍是多态）

    // ===== 2. dynamic_cast：运行时类型检查的安全向下转型（多态专用）=====
    std::cout << "--- dynamic_cast ---" << std::endl;

    Animal* pet = new Dog{};
    // 向下转型并检查类型（对应 Go 类型断言 v, ok := x.(T) / Rust downcast）
    if (Dog* d = dynamic_cast<Dog*>(pet)) {
        PRINT("pet 是 Dog"); // 走这里
        d->Fetch();
    } else {
        PRINT("pet 不是 Dog");
    }

    if (Cat* c = dynamic_cast<Cat*>(pet)) {
        (void)c; // 仅演示类型判定成功，无需使用 c
        PRINT("pet 是 Cat");
    } else {
        PRINT("pet 不是 Cat"); // 走这里
    }
    delete pet;

    // ===== 3. const_cast：去掉/加上 const（仅用于修改非真正常量的对象）=====
    std::cout << "--- const_cast ---" << std::endl;

    const int value = 100;
    const int* cp = &value;
    // *cp = 200; // 错误：不能通过 const 指针修改
    int* mp = const_cast<int*>(cp); // 去掉 const
    PRINT("const_cast 后读取: {}", *mp); // 100
    PRINT("*mp == value: {}", *mp == value); // true

    // ===== 4. reinterpret_cast：按位重新解释（危险，对应 C 强制转换 / Zig @bitCast）=====
    std::cout << "--- reinterpret_cast ---" << std::endl;

    // 把地址当作整数打印（指针 → 整数）
    int x = 42;
    PRINT("&x 地址（按整数）: {:#x}", reinterpret_cast<uintptr_t>(&x));

    // 把 float 的二进制位解释为 int（用 bit_cast 安全实现位级重解释）
    float f = 1.0f;
    uint32_t bits = std::bit_cast<uint32_t>(f);
    PRINT("1.0f 的 IEEE 754 位表示: {:#x}", bits); // 0x3f800000

    // ===== 5. C 风格转换对比（尽量用上面的四种 cast）=====
    std::cout << "--- 对比 ---" << std::endl;

    // (int)3.99 等价 static_cast，但 C 风格无法区分语义、可读性差
    PRINT("C 风格: {}, C++ 风格: {}", static_cast<int>(3.99), (int)3.99); // 3 3

    // 唯一例外：void* ↔ T* 只能用 reinterpret_cast 或 C 风格
    void* raw = &x;
    PRINT("void* 还原: {}", *static_cast<int*>(raw)); // 42
}
