#include <iostream>

// 虚函数 virtual
// 利用虚函数实现多态 

class Animal {
public:
    // 虚函数
    virtual void speak(){
        std::cout << "Animal::speak()" << std::endl;
    }

    // 纯虚函数 - 子类必须实现
    virtual void run() = 0;

    // 虚析构函数 (当在 父类指针 指向 子类对象时)
    virtual ~Animal(){
        std::cout << "Animal::~Animal()" << std::endl;
    }
};

class Dog : public Animal {
public:
    void speak(){
        std::cout << "Dog::speak()" << std::endl;
    }
    void run(){
        std::cout << "Dog::run()" << std::endl;
    }

    ~Dog(){
        std::cout << "Dog::~Dog()" << std::endl;
    }
};

class Cat : public Animal {
public:
    void speak(){
        // 调用父类的方法
        Animal::speak();
        std::cout << "Cat::speak()" << std::endl;
    }
    void run(){
        std::cout << "Cat::run()" << std::endl;
    }

    ~Cat(){
        std::cout << "Cat::~Cat()" << std::endl;
    }
};

void ClassVirtualLiu(Animal *p) {
    p->speak();
    p->run();

    delete p;
}

void ClassVirtualMain() {
    auto dog = new Dog();
    auto cat = new Cat();
    ClassVirtualLiu(dog);
    ClassVirtualLiu(cat);

    //delete dog;
    //delete cat;
}
