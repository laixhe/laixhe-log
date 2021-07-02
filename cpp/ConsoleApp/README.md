### 能被赋值的就是左值，不能被赋值的就是右值

### C++中可以使用 struct class 来定义一个类
> struct 默认成员权限是 public
>
> class 默认成员权限是 private
>

```
#  类的定义         |        结构体
 class Test{       |     struct Test{
    //成员变量      |        //成员变量
    int m_age;     |        int m_age;
    // 成员函数     |        // 成员函数
    void run(){    |        void run(){
    }              |        }
 };                |      };
```

> 实例化对象：

```
#### 局部和全局的区别

Test test;
test.m_age;
# 局部：其内存在栈空间的，系统并没有初始化成员变量
# 全局：其内存在全局区空间的，系统并初始化成员变量，int=0

#### 指针对象的区别

# 系统没有初始化成员变量
Test ×test = new Test;
×test->m_age;

# 系统会初始化成员变量，int=0
Test ×test = new Test();
×test->m_age;

# 都在堆空间
# 如果定义了构造函数就必须自已初始化成员变量
````

> 注意：

```
Test test;   // 调用无参的构造函数
Test test(); // 注意：是函数的声明，不是调用无参的构造函数
Test *test = new Test;   // 调用无参的构造函数
Test *test = new Test(); // 调用无参的构造函数
```
