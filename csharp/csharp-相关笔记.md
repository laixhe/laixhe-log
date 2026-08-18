# C# 高级特性速查

> 参考《C#高级编程(第11版)》

## 异步方法

使用 `async` 修饰的方法被称为异步方法，调用时应在前面加上 `await`：

```csharp
async Task<string> FetchAsync()
{
    await Task.Delay(1000);     // 等待耗时操作
    return "done";
}
```

## 参数传递：ref / out

```csharp
// ref：引用传递，传入前必须初始化，方法内可读可写
void Add(ref int x) => x += 10;
int a = 5;
Add(ref a);
Console.WriteLine(a);   // 15

// out：引用传递，传入前无需初始化，方法内必须赋值
void TryParse(string s, out int result) => result = int.Parse(s);
TryParse("42", out int b);
Console.WriteLine(b);   // 42
```

## 多态：virtual / override / base

```csharp
class Animal
{
    public virtual void Speak() => Console.WriteLine("动物叫");
}

class Dog : Animal
{
    public override void Speak()
    {
        base.Speak();           // 调用父类方法
        Console.WriteLine("汪汪");
    }
}

Animal a = new Dog();
a.Speak();                      // 输出：动物叫 + 汪汪
```

## 抽象与密封：abstract / sealed

```csharp
abstract class Shape
{
    public abstract double Area();  // 抽象方法：子类必须实现
}

sealed class FinalClass { }         // 密封类：不能被继承
```
