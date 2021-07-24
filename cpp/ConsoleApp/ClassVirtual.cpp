#include <iostream>

// 虚函数 virtual
// 利用虚函数实现多态
//
// 抽象类
// 在类中至少有一个 纯虚函数，则这个类就是抽象类
// 纯虚函数是通过在声明中使用 "= 0" 来指定的
//
// 重写 override [C++11]
// 表示派生类重写基类虚函数
//
// 调用父类
// 使用类名::xxx (类作用域操作符 ::)

class VirtualAnimal {
public:
    // 虚函数
    virtual void speak();
    // 纯虚函数 - 子类必须实现
    virtual void run() = 0;
    // 虚析构函数 (当在 父类指针 指向 子类对象时)
    virtual ~VirtualAnimal();
};

void VirtualAnimal::speak(){
    std::cout << "Animal::speak()" << std::endl;
}

VirtualAnimal::~VirtualAnimal(){
    std::cout << "Animal::~Animal()" << std::endl;
}

//=============================

class VirtualDog : public VirtualAnimal {
public:
    void speak() override; // override(重写)[C++11]，表示派生类重写基类虚函数
    void run() override;

    ~VirtualDog() override;
};

void VirtualDog::speak(){
    std::cout << "Dog::speak()" << std::endl;
}

void VirtualDog::run(){
    std::cout << "Dog::run()" << std::endl;
}

VirtualDog::~VirtualDog(){
    std::cout << "Dog::~Dog()" << std::endl;
}

//=============================

class VirtualCat : public VirtualAnimal {
public:
    void speak() override;
    void run() override;

    ~VirtualCat() override;
};

void VirtualCat::speak(){
    // 调用父类的方法
    VirtualAnimal::speak();
    std::cout << "Cat::speak()" << std::endl;
}

void VirtualCat::run(){
    std::cout << "Cat::run()" << std::endl;
}

VirtualCat::~VirtualCat(){
    std::cout << "Cat::~Cat()" << std::endl;
}


//=============================

// 利用继承与虚函数实现多态
void ClassVirtualLiu(VirtualAnimal *p) {
    p->speak();
    p->run();

    delete p;
}

void ClassVirtualMain() {
    auto dog = new VirtualDog();
    auto cat = new VirtualCat();
    ClassVirtualLiu(dog);
    ClassVirtualLiu(cat);

    //delete dog;
    //delete cat;
}
