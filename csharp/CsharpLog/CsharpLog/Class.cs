using System;

/*
#### 类的定义
> 由关键字```class```声明
> 
> 有 4 个访问修饰符 public、protected、private、internal 默认情况下，类是 internal，成员是 private 的
>
> 有虚(virtual)方法，用于子类重写(override)
>
> 有静态(static)成员或方法
>
> 有继承，只能单继承
>
> 有抽象类(abstract)，是不能被实例化的，其方法默认为虚方法，只能被继承的子类中重写 (只能单继承)(抽象类是类的一种)
>
> 有接口(interface)声明默认是 public 的，可以实现多个接口
>

#### 类
```
class Example {
    // 成员
    private string name;
    private int age;
    // 构造函数
    Example(string name, int age) {
        this.name = name;
        this.age = age;
    }
    // 析构函数
    ~Example(){}
    // 方法
    int Test() {
        return 0;
    }
}
```

#### 抽象类
```
public abstract class Example{
    public void DoWork(){
        Console.WriteLine("Example.DoWork");
    }
}
```

#### 接口
> 从 C# 8 开始，接口的方法可以拥有具体实现的代码

```
interface IExample {
    void DoWork();                                     // 允许
    void Test() => Console.WriteLine("IExample.Test"); // 允许
}

*/

namespace CsharpLog{
    class Class {
        public static void Run() {

            // 实例化对象
            var b = new Base(18, "laixhe");
            b.Show();     // 结果： base show id=18, name=laixhe
            b.BaseShow(); // 结果： base BaseShow age=18, name=laixhe

            Console.WriteLine("{0}", TestBase.StaticAge); // 结果： 20
            TestBase.ShowStaticAge();
            TestBase.ShowStaticAge(); // 结果： 21

            var t = new TestBase(16, "laiki");
            t.Show();     // 结果： base show age=16, name=laiki
            t.BaseShow(); // 结果： TestBase BaseShow age=16, name=laiki
            Console.WriteLine(t.Age);
            
            Show(b); // 结果： base show age=18, name=laixhe
            Show(t); // 结果： base show age=16, name=laiki

        }

        // 利用接口实现多态
        static void Show(IBase i){
            i.Show();
        }
        
    }

    // 定义接口
    interface IBase {
        void Show();
    }

    // 定义类
    class Base: IBase
    {
        public int Age;
        public string Name;
        // 构造函数
        public Base(int age, string name) {
            Age = age;
            Name = name;
        }
        // 析构函数
        //~Base() {}

        // 成员方法并实现 IBase 接口的方法
        public void Show() {
            Console.WriteLine("base show age={0}, name={1}", Age, Name);
        }

        // 定义虚方法，用于子类重写的
        public virtual void BaseShow() {
            Console.WriteLine("base BaseShow age={0}, name={1}", Age, Name);
        }
    }
    
    // 继承 Base
    class TestBase: Base {
        // 构造函数
        public TestBase(int age, string name) : base(age, name) {}

        // 靜态属性
        public static int StaticAge = 20;
        // 靜态方法
        public static void ShowStaticAge()
        {
            Console.WriteLine(StaticAge);
            StaticAge++;
        }

        // 重写虚方法
        public override void BaseShow() {
            Console.WriteLine("TestBase BaseShow age={0}, name={1}", Age, Name);
        }
    }

}