### 能被赋值的就是左值，不能被赋值的就是右值
> `const&` 这个特殊技能：既能接受左值又能接受右值 如：void xxx(const string& s)
>
> 在 C++11 引入了右值引用 &&，与 const& 区分开来，因为 const& 虽然可以接受右值引用了，但是却限制了修改，传入的对象是不能修改的
>
> 完美转发，&&在模板参数中表示万能引用，在非模板参数中表示右值引用 [C++11]
```
# 完美转发
template<typename T>
void Func(T &&){
}
# 当传入右值时，模板特化成 void Func(T &&)，当传入左值时，模板特化成 void Func(T &)
```

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
