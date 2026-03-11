### 我们要想执行我们编写的 C 程序，那么第一步需要对这个程序进行编译
> 预处理：宏定义展开、头文件展开、条件编译，这里并不会检查语法
> 
> 编 译：检查语法，将预处理后文件编译生成汇编文件
> 
> 汇 编：将汇编文件生成目标文件(二进制文件)
> 
> 链 接：将目标文件链接为可执行程序
>

### 内存四区之代码区，全局区，栈区和堆区 
> C++ 在程序执行时，将内存大致分为代码区，全局区，栈区和堆区四个区域。不同的区域存储不同的数据，赋予不同的生命周期，能够更灵活地进行编程。

1. 代码区：存放函数体的二进制代码，由操作系统管理创建，代码区时共享的，对于频繁被执行的程序，只需要存有一份代码即可(代码段)
2. 全局区：存放全局变量和静态变量以及常量，在程序结束后由操作系统释放(数据段)
3. 栈区：由编译其自动分配和释放，存放函数的参数值以及局部变量，因此不要返回局部变量的地址(自动分配和释放)
4. 堆区：一般由程序员通过 `new` 开辟空间，进行分配和释放，若程序员不释放，则程序结束时由操作系统回收(需要主动去申请和释放)

```
// 统一的字符字面量
#include <string>
char8_t   std::u8string   u8""  [C++ 20] UTF-8
char16_t  std::u16string  u""   [C++ 11] UTF-16
char32_t  std::u32string  U""   [C++ 11] UTF-32

// 带分隔的转义序列
// [C++ 23]
\o{}  八进制
\x{}  十六进制
\u{}  16位 Unicode 码点
\U{}  32位 Unicode 码点

// 固定宽度的整数类型
// [C++ 11]
#include <cstdint>
  std::int8_t     std::uint8_t
  std::int16_t    std::uint16_t
  std::int32_t    std::uint32_t
  std::int64_t    std::uint64_t
  std::INT8_MIN   std::INT8_MAX   std::UINT8_MAX
  std::INT16_MIN  std::INT16_MAX  std::UINT16_MAX
  std::INT32_MIN  std::INT32_MAX  std::UINT32_MAX
  std::INT64_MIN  std::INT64_MAX  std::UINT64_MAX
  intptr_t uintptr_t 用于存储指针的整数类型，确保可以存储任何指针值的整数

// 类型安全的格式化（替代 printf/iostream）
// [C++ 20]
#include <format>

// 只读字符串视图（只读字符串引用，不分配内存、不复制数据）
// [C++ 17]
#include <string_view>

// 专为处理原始字节数据而设计
// [C++ 17] #include <cstddef>
std::byte

// 任意类型
// [C++ 17]
#include <any>

// 可选值
// [C++ 17]
#include <optional>
  std::optional<T>
  std::nullopt 可空

// 错误处理
// [C++ 23]
#include <expected>

// 复合数据
#include <utility> std::pair   用于存储两个不同类型的对象（std::make_pair）
#include <array>   // [C++ 11] 数组
#include <tuple>   // [C++ 17] 元组
#include <variant> // [C++ 17] 类型安全的联合体（std::monostate）

// 字符串与数值互转
// [C++ 17]
#include <charconv>

// 智能指针
// [C++ 11]
#include <memory>

// 互斥锁（独占锁）
// [C++ 11]
#include <mutex>

// 共享锁（读写锁与独占锁）
// [C++ 17]
#include <shared_mutex>

// 处理时间和日期
// [C++ 11]
#include <chrono>

// 位操作
// [C++ 20]
#include <bit>
  std::endian    获取标量类型的端序（大小端）
  std::byteswap  [C++23] 反转任何整数类型的字节序

// 连续内存访问抽象（替代裸指针与 std::vector 引用）
// [C++ 20]
#include <span>
  std::span<T>

// 内存流（文本/二进制）（基于 std::span）
// [C++ 23]
#include <spanstream>
  std::ispanstream 输入流（只读）（对应传统流 istringstream）
  std::ospanstream 输出流（只写）（对应传统流 ostringstream）
  std::spanstream  双向流（读+写）（对应传统流 stringstream）

// 路径与文件操作
// [C++ 17]
#include <filesystem>

// 范围操作
// [C++ 20]
#include <ranges>
  std::views 处理数据（过滤、变换、切片）

// 原子操作
// [C++ 11]
#include <atomic>

// 多线程
// [C++ 11]
#include <thread>

// 协程
// [C++ 20]
#include <coroutine>

// 并行执行策略（结构化并发）
// [C++ 17 26]
#include <execution>
```