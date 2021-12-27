#include <iostream>
#include <memory>

// [C++11]
// 为了解决 C++ 内存泄漏的问题，C++11 引入了智能指针 (Smart Pointer)
// 头文件 <memory>

class TestPtr{
public:
    TestPtr(std::string str): m_str(str){
        std::cout << "TestPtr::TestPtr("<< m_str <<")..." << std::endl;
    }

    ~TestPtr(){
        std::cout << "TestPtr::~TestPtr("<< m_str <<")..." << std::endl;
    }

    void print(){
        std::cout << "TestPtr::print("<< m_str <<")" << std::endl;
    }
private:
    std::string m_str;
};

// unique_ptr 独占的智能指针
// .release() // 把指针赋为空，但是并没有释放指针指向的内存，并返回原本的指针
// .reset(p)  // 重新绑定对象，原来的对象会被释放掉
// [C++11]
void uniquePtr(){

    std::unique_ptr<TestPtr> ptest1(new TestPtr("123")); // 调用了构造函数，输出 TestPtr::TestPtr(123)
    std::unique_ptr<TestPtr> ptest2(new TestPtr("456")); // 调用了构造函数，输出 TestPtr::TestPtr(456)

    // 不能直接 ptest2 = ptest1
    // 调用 move 后 ptest2 原本对象会被释放
    ptest2 = std::move(ptest1); // 调用了 ptest2 析构函数，输出 TestPtr::~TestPtr(456)
    ptest2->print(); // 已经将 ptest1 转移到了 ptest2 了，会输出 TestPtr::print(123)

    // 经过前面的 move 后 ptest1 会被赋值为 nullptr
    if (ptest1 == nullptr) {
        std::cout << "ptest1 == nullptr" << std::endl;
    }

    std::cout << "uniquePtr end --------------" << std::endl;
    // 当本函数执行结束时，会调用 析构函数
}


// shared_ptr 是指多个智能指针可以同时管理同一块有效的内存(共享的智能指针)
// 使用引用计数，每一个 shared_ptr 的拷贝都指向相同的内存
// 每使用他一次，内部的引用计数加1，每析构一次，内部的引用计数减1，减为0时，删除所指向的堆内存
// 注意：不能将一个原始指针直接赋值给一个智能指针 (error：std::shared_ptr<int> p = new int(1) )
// 注意：不要引起循环引用
//
// .use_count() // 查看引用的数量
// [C++11]
void sharedPtr(){

    // 初始化有三种方式
    // 1. 通过构造函数
    // 2. 通过 std::make_shared 辅助函数
    // 3. 通过 reset 方法
    //
    // 1.1 通过构造函数初始化
    // shared_ptr<int> ptr1(new int(520));
    // 调用拷贝构造函数
    // shared_ptr<int> ptr2(ptr1);
    // 如果使用拷贝的方式初始化共享智能指针对象，这两个对象会同时管理同一块堆内存，
    // 堆内存对应的引用计数也会增加
    //
    // 1.2 调用移动构造函数
    // std::shared_ptr<int> ptr3 = std::move(ptr2);
    // 如果使用移动的方式初始智能指针对象，只是转让了内存的所有权，
    // 管理内存的对象并不会增加，因此内存的引用计数不会变化
    //
    // 2.1 通过 std::make_shared 初始化
    // shared_ptr<int> ptr1 = make_shared<int>(520);
    //
    // 3.1 通过 reset 方法
    // 有两个操作：
    //   3.1.1 当有值的时候，调用 reset() 会使引用计数减1，如果发现引用计数为0时，则析构旧对象
    //   3.1.2 当调用 reset（new xxx()) 重新赋值时，智能指针首先是生成新对象，然后将就对象的引用计数减1，然后将新对象的指针交给智能指针保管

    std::shared_ptr<TestPtr> ptest1(new TestPtr("123")); // 调用了构造函数，输出 TestPtr::TestPtr(123)
    std::shared_ptr<TestPtr> ptest2 = std::make_shared<TestPtr>("456"); // 调用了构造函数，输出 TestPtr::TestPtr(456)
    std::shared_ptr<TestPtr> ptest3 = std::make_shared<TestPtr>("789"); // 调用了构造函数，输出 TestPtr::TestPtr(789)

    std::cout << "ptest2 use_count=" << ptest1.use_count() << std::endl;

    ptest1 = ptest2; // ptest2 引用次数加1，然后原有的 ptest1 引用计数减1，引用计数为0，故析构 "123"销毁，输出 TestPtr::~TestPtr(123)...

    std::cout << "ptest2 use_count=" << ptest1.use_count() << std::endl;

    ptest3.reset(new TestPtr("000")); // 首先生成新对象，然后原有的 ptest3 引用计数减1，引用计数为0，故析构 "789"销毁，输出 TestPtr::~TestPtr(789)...

    std::shared_ptr<TestPtr> ptest4 = ptest2; // ptest2 引用次数加1

    // ptest1 ptest2 ptest4 都指向同一个地指
    std::cout << "ptest1 use_count=" << ptest1.use_count() << " p=" << ptest1 << std::endl;
    std::cout << "ptest2 use_count=" << ptest2.use_count() << " p=" << ptest2 << std::endl;
    std::cout << "ptest4 use_count=" << ptest4.use_count() << " p=" << ptest4 << std::endl;

    std::cout << "sharedPtr end --------------" << std::endl;
}

// weak_ptr 是用来解决 shared_ptr 相互引用时的死锁问题(弱引用的智能指针)
