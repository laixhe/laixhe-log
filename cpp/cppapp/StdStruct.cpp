#include "StdStruct.h"

#include <format>   // std::format [C++20]
#include <iostream>
#include <string>    // std::string / std::to_string

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 演示类型放入匿名命名空间：内部链接，避免与其他模块的同名类型冲突（ODR）。
namespace
{
// ===== 1. 定义结构体：对应 Rust struct / Go type struct / C# class =====
// 成员默认初始化（C++11 起支持，类内初始化器）
struct Point
{
    int x = 0;   // 类内初始化器
    int y = 0;
    std::string name; // 成员类型可用标准库类型

    // 结构体也可以有成员函数（对应 Go 方法）
    std::string ToString() const
    {
        return name + "(" + std::to_string(x) + ", " + std::to_string(y) + ")";
    }
};

// 位域：按位分配成员空间（对应 C 语言位段，常用于硬件寄存器/协议解析）
struct Flags
{
    unsigned int read : 1;   // 占 1 位
    unsigned int write : 1;  // 占 1 位
    unsigned int exec : 1;   // 占 1 位
    unsigned int mode : 5;   // 占 5 位
};

// 对齐：alignas 指定对齐字节数（对应 Go unsafe.Alignof / Rust repr(align)）
struct alignas(16) Vec4
{
    float x;
    float y;
    float z;
    float w;
};
} // namespace

StdStruct::StdStruct()
{
    // ===== 1. 聚合初始化：对应 Rust 结构体字面量 / Go 结构体复合字面量 =====
    std::cout << "--- struct ---" << std::endl;

    Point p1{1, 2, "p1"};              // 按成员顺序初始化
    Point p2;                          // 使用默认初始化（x=0, y=0）
    PRINT("p1 = {}", p1.ToString());   // p1(1, 2)
    PRINT("p2 = {}", p2.ToString());   // (0, 0)（默认值）

    // 成员访问：对应 Go 点号访问 / Rust 点号访问
    p1.x = 10;
    p1.y = 20;
    PRINT("修改后 p1 = {}", p1.ToString()); // p1(10, 20)

    // ===== 2. 位域：位级紧凑存储 =====
    std::cout << "--- 位域 ---" << std::endl;

    Flags f{1, 1, 0, 0b00101};
    // 位域不能直接绑定引用参数，先复制到普通变量
    unsigned int r = f.read, w = f.write, e = f.exec, m = f.mode;
    PRINT("read={} write={} exec={} mode={}", r, w, e, m);
    // read=1 write=1 exec=0 mode=5
    PRINT("Flags 占用字节: {}", sizeof(Flags)); // 4（对齐到 unsigned int）

    // ===== 3. union：多个成员共享同一内存（对应 Rust union / C union）=====
    std::cout << "--- union ---" << std::endl;

    union Value
    {
        int i;
        float f;
        unsigned char bytes[4];
    };
    Value v;
    v.i = 0x41424344; // 写入整型后，bytes 视图按字节读出
    PRINT("int 视角: {:#x}", v.i);
    PRINT("字节视角: {:02x} {:02x} {:02x} {:02x}",
          static_cast<int>(v.bytes[0]), static_cast<int>(v.bytes[1]),
          static_cast<int>(v.bytes[2]), static_cast<int>(v.bytes[3]));
    // 小端序输出：44 43 42 41

    // ===== 4. 内存对齐：alignof / alignas =====
    std::cout << "--- 对齐 ---" << std::endl;

    PRINT("alignof(Point) = {}", alignof(Point));    // 8（含 std::string）
    PRINT("alignof(Vec4) = {}", alignof(Vec4));      // 16（alignas(16) 指定）
    PRINT("sizeof(Vec4) = {}", sizeof(Vec4));        // 16（4 * float 正好对齐）

    // ===== 5. struct 与 class 差异：默认访问权限不同 =====
    std::cout << "--- struct vs class ---" << std::endl;

    struct A
    {
        int value; // 默认 public
    };
    class B
    {
        int value; // 默认 private
        public:
        explicit B(int v) : value(v) {}
        int Get() const { return value; }
    };
    A a{42};
    B b(42);
    PRINT("struct 默认公开: a.value = {}", a.value);   // 42
    PRINT("class 默认私有: b.Get() = {}", b.Get());    // 42
}
