#include <iostream>

// 继承
// 子类的构造函数默认会调用父类的无参构造函数
// 如果子类的构造函数显式地调用了父类的有参构函数，就不会再去默认调用父类的无参构造函数
// 如果父类缺少无参构造函数，子类的构造函数必须显式调用父类的有参构造函数

class Person {
public:
    int m_age = 0; // [C++11]允许非静态成员进行初始化

    Person();
    Person(int age);

    void run();
};

Person::Person(){
    std::cout << "Person::Person() m_age=" << m_age << std::endl; 
}

// 构造函数，并初始化列表
Person::Person(int age): m_age(age){
    std::cout << "Person::Person(int age) m_age=" << m_age << std::endl; 
}

void Person::run(){
    std::cout << "Person::run() m_age=" << m_age << std::endl;
}

//==================

class Student : public Person {
public:
    int m_score = 0; // [C++11]允许非静态成员进行初始化

    Student();
    Student(int score);

    void study();
};

Student::Student(){
}

// 构造函数 调用  父类的构造函数 与 初始化列表
Student::Student(int score): Person(score), m_score(score){
}

void Student::study(){
    std::cout << "Student::study() m_age=" << m_age << " | m_score=" << m_score << std::endl;
}

//==================

class Worker : public Person {
public:
    int m_salary = 0; // [C++11]允许非静态成员进行初始化

    Worker();
    Worker(int salary);

    void work();
};

// 构造函数 调用  构造函数(委托构造函数)
// [C++11]
Worker::Worker(): Worker(11){
}

// 构造函数，并初始化列表
Worker::Worker(int salary) : m_salary(salary){
}

void Worker::work(){
    std::cout << "Worker::work() m_age=" << m_age << " | m_salary=" << m_salary << std::endl;
}

//==================

void ClassExtentds(){
    
    Worker worker;
    worker.run();
    worker.work();

    std::cout << "------------" << std::endl;

    Student student;
    student.run();
    student.study();

    std::cout << "------------" << std::endl;

    Student student1(11);
    student1.run();
    student1.study();
}
