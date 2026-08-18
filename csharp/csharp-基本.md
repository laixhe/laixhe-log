# C# 基础语法

C# 是 .NET 平台的主流语言，强类型、面向对象。本篇覆盖最常用的基础语法。

## 第一个程序

现代 C#（.NET 6+）支持「顶层语句」，无需显式 `Main` 方法：

```csharp
Console.WriteLine("Hello, C#!");
```

## 变量与数据类型

```csharp
int age = 18;              // 整型
double price = 3.14;       // 双精度浮点
bool ok = true;            // 布尔
string name = "laixhe";    // 字符串
char c = 'A';              // 字符

var n = 42;                // var 自动推断类型（仍是强类型）
```

## 控制流

### if / else

```csharp
int score = 85;
if (score >= 90)
    Console.WriteLine("优秀");
else if (score >= 60)
    Console.WriteLine("及格");
else
    Console.WriteLine("不及格");
```

### switch

```csharp
string day = "周一";
switch (day)
{
    case "周一":
        Console.WriteLine("星期一");
        break;
    default:
        Console.WriteLine("其它");
        break;
}
```

### for / while / foreach

```csharp
for (int i = 0; i < 5; i++)
    Console.Write(i + " ");      // 0 1 2 3 4

int j = 0;
while (j < 3)
{
    Console.Write(j + " ");
    j++;
}

foreach (var item in new[] { 1, 2, 3 })
    Console.Write(item + " ");   // 1 2 3
```

## 方法

```csharp
int Add(int a, int b) => a + b;        // 表达式体方法
Console.WriteLine(Add(1, 2));          // 3

void Say(string name = "world") => Console.WriteLine($"hello {name}");
Say();                                  // hello world
```

## 类与对象

```csharp
class Person
{
    public string Name { get; set; } = "";  // 属性
    public int Age { get; set; }

    public Person(string name, int age)     // 构造函数
    {
        Name = name;
        Age = age;
    }

    public void SayHello() => Console.WriteLine($"我是 {Name}，今年 {Age} 岁");
}

var p = new Person("laixhe", 18);
p.SayHello();   // 我是 laixhe，今年 18 岁
```

## 接口

```csharp
interface IShape
{
    double Area();
}

class Circle : IShape
{
    public double Radius { get; set; }
    public double Area() => Math.PI * Radius * Radius;
}

IShape s = new Circle { Radius = 2 };
Console.WriteLine(s.Area());
```

## 集合与 LINQ

```csharp
var list = new List<int> { 3, 1, 2 };

// LINQ：筛选、转换、排序
var even = list.Where(x => x % 2 == 0).ToList();   // [2]
var sorted = list.OrderBy(x => x).ToList();         // [1, 2, 3]

var dict = new Dictionary<string, int>
{
    ["a"] = 1,
    ["b"] = 2,
};
Console.WriteLine(dict["a"]);   // 1
```

## 异步（async / await）

```csharp
async Task<string> FetchAsync()
{
    await Task.Delay(1000);     // 模拟耗时操作
    return "done";
}

var result = await FetchAsync();
Console.WriteLine(result);      // done
```
