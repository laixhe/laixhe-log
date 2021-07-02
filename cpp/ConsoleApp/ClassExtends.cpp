#include <iostream>

// 继承
// 子类的构造函数默认会调用父类的无参构造函数
// 如果子类的构造函数显式地调用了父类的有参构函数，就不会再去默认调用父类的无参构造函数
// 如果父类缺少无参构造函数，子类的构造函数必须显式调用父类的有参构造函数

class Person {
public:
    int m_age;

    Person(){
       std::cout << "Person::Person() m_age=" << m_age << std::endl; 
    }

    Person(int age) : m_age(age) {
       std::cout << "Person::Person() m_age=" << m_age << std::endl; 
    }

    void run(){
        std::cout << "Person::run() m_age=" << m_age << std::endl;
    }
};

class Student : public Person {
public:
    int m_score;

    Student(){
    }

    // 构造函数 调用  父类的构造函数
    Student(int score): Person(score), m_score(score) {
    }

    void study(){
        std::cout << "Student::study() m_age=" << m_age << " | m_score=" << m_score << std::endl;
    }
};

class Worker : public Person {
public:
    int m_salary;
    void work(){
        std::cout << "Worker::work() m_age=" << m_age << " | m_salary=" << m_salary << std::endl;
    }

    // 构造函数 调用  构造函数
    Worker() : Worker(11){
        m_age = 10;
    }

    // 构造函数，并初始化列表
    Worker(int salary) : m_salary(salary){
    }
};

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
