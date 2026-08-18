#include "StdClass.h"

#include <format>    // std::format [C++20]
#include <iostream>
#include <string>
#include <memory>    // std::make_unique

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// ===== 封装：private 数据 + public 方法（对应 Java/Python 类）=====
class Person
{
    private:
    std::string name;
    int age;

    public:
    // 构造函数（对应 Rust NewXxx / Go 无构造函数约定）
    Person(const std::string& n, int a) : name(n), age(a) {}

    // 方法（对应 Rust impl / Go 方法）
    std::string greet() const
    {
        return std::format("你好，我是 {}，今年 {} 岁", name, age);
    }

    // setter：修改私有字段
    void birthday()
    {
        age++;
    }

    // getter
    int getAge() const
    {
        return age;
    }
};

// ===== 继承与多态（对应 Rust trait 对象 / Go 接口）=====
// 基类：虚函数 = 虚方法（对应 Rust trait 方法 / Java 抽象方法）
class Animal
{
    public:
    virtual std::string speak() const = 0; // 纯虚函数：Animal 是抽象类（对应 Rust trait）
    virtual ~Animal() = default;           // 虚析构：多态删除安全
};

// 派生类：实现 speak（对应 Rust impl Trait for Type / Go 隐式实现接口）
class Dog : public Animal
{
    private:
    std::string name;

    public:
    explicit Dog(const std::string& n) : name(n) {}

    std::string speak() const override
    {
        return name + " 汪汪叫";
    }
};

class Cat : public Animal
{
    private:
    std::string name;

    public:
    explicit Cat(const std::string& n) : name(n) {}

    std::string speak() const override
    {
        return name + " 喵喵叫";
    }
};

StdClass::StdClass()
{
    // ===== 1. 类与封装 =====
    std::cout << "--- 类与封装 ---" << std::endl;

    Person p{"laixhe", 18};
    PRINT("{}", p.greet()); // 你好，我是 laixhe，今年 18 岁
    p.birthday();
    PRINT("生日后年龄: {}", p.getAge()); // 19

    // ===== 2. 继承与多态（对应 Rust &dyn Trait / Go 接口多态）=====
    std::cout << "--- 继承与多态 ---" << std::endl;

    // 基类指针指向派生类：运行时多态（对应 Rust trait 对象 / Java 接口引用）
    std::unique_ptr<Animal> dog = std::make_unique<Dog>("旺财");
    std::unique_ptr<Animal> cat = std::make_unique<Cat>("咪咪");

    PRINT("{}", dog->speak()); // 旺财 汪汪叫
    PRINT("{}", cat->speak()); // 咪咪 喵喵叫
}
