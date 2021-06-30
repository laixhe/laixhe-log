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
Test test;
test.m_age;
# 其内存在栈空间的
````

> 实例化指针对象：

```
Test ×test = new Test();
×test->m_age;
````
