// 面向对象：类 / 属性 / 继承多态 / 抽象类 / 接口默认实现 / record / struct
// 对应 C++ cppapp StdClass、Python pylog classes.py、Java 面向对象

public static class ClassDemo
{
    public static void Run()
    {
        // ===== 1. 类：构造函数 / 属性 / 方法 =====
        Console.WriteLine("--- 类 ---");

        var animal = new Animal("旺财");
        animal.Speak(); // 旺财：发出叫声

        // 属性封装（对应 Java getter/setter / Python @property）
        var car = new Car("Tesla");
        car.Speed = 250; // 超速会被 setter 里的 Clamp 上限到 200
        Console.WriteLine($"car: {car.Brand} 速度 {car.Speed} km/h（setter 做了上限校验）"); // car: Tesla 速度 200 km/h（250 被 Clamp 到上限 200）

        // ===== 2. 继承 + 多态（virtual/override，对应 C++ virtual / Java @Override）=====
        Console.WriteLine("--- 继承与多态 ---");

        Animal dog = new Dog("小白");
        Animal cat = new Cat("咪咪");
        dog.Speak(); // 小白：汪汪叫（运行时调用实际类型的重写实现）
        cat.Speak(); // 咪咪：喵喵叫

        // 抽象类：不能实例化（对应 Java 抽象类 / C++ 纯虚函数）
        Shape circle = new Circle(5);
        Shape rect = new Rectangle(3, 4);
        Console.WriteLine($"圆面积 = {circle.Area():F2}，矩形面积 = {rect.Area()}"); // 圆面积 = 78.54，矩形面积 = 12

        // ===== 3. 静态成员（对应 Java static / C++ 静态成员）=====
        Console.WriteLine("--- 静态成员 ---");

        MathUtils.UtilsCounter++; // 静态字段全局共享
        MathUtils.UtilsCounter++;
        Console.WriteLine($"静态字段计数 = {MathUtils.UtilsCounter}，静态方法 Add = {MathUtils.Add(1, 2)}"); // 静态字段计数 = 2，静态方法 Add = 3

        // ===== 4. record：不可变数据 + 值相等（对应 Java record）=====
        Console.WriteLine("--- record ---");

        var p1 = new Person("laixhe", 18);
        var p2 = p1 with { Age = 19 }; // with 表达式生成修改副本（不可变更新）
        Console.WriteLine($"p1 = {p1}，p2 = {p2}");
        Console.WriteLine($"p1 == p2（按值比较）: {p1 == p2}"); // p1 == p2（按值比较）: False

        // ===== 5. struct：值类型（对应 C/C++ struct）=====
        Console.WriteLine("--- struct ---");

        var point = new Point(3, 4);
        Console.WriteLine($"Point(3,4) 到原点距离 = {point.Distance()}"); // Point(3,4) 到原点距离 = 5

        // ===== 6. 接口默认实现（C# 8+）=====
        Console.WriteLine("--- 接口默认实现 ---");

        ILogger logger = new ConsoleLogger();
        logger.Log("hello");
        logger.LogError("something failed"); // 未实现的方法走接口默认实现
    }
}

// 基类 + 虚方法
public class Animal
{
    public string Name { get; }

    public Animal(string name) => Name = name;

    public virtual void Speak() => Console.WriteLine($"{Name}：发出叫声");
}

public class Dog : Animal
{
    public Dog(string name) : base(name) { }

    public override void Speak() => Console.WriteLine($"{Name}：汪汪叫");
}

public class Cat : Animal
{
    public Cat(string name) : base(name) { }

    public override void Speak() => Console.WriteLine($"{Name}：喵喵叫");
}

// 抽象类：子类必须实现抽象成员
public abstract class Shape
{
    public abstract double Area();
}

public class Circle : Shape
{
    private readonly double _r;
    public Circle(double r) => _r = r;
    public override double Area() => Math.PI * _r * _r;
}

public class Rectangle : Shape
{
    private readonly double _w, _h;
    public Rectangle(double w, double h) { _w = w; _h = h; }
    public override double Area() => _w * _h;
}

// 属性封装 + 写入校验
public class Car
{
    private int _speed;

    public string Brand { get; } // 只读自动属性

    public int Speed
    {
        get => _speed;
        set => _speed = Math.Clamp(value, 0, 200);
    }

    public Car(string brand) => Brand = brand;
}

// 静态类：只能有静态成员
public static class MathUtils
{
    public static int UtilsCounter; // 静态字段
    public static int Add(int a, int b) => a + b;
}

// record：不可变、按值比较（== 比较的是字段而非引用）
public record Person(string Name, int Age);

// struct：值类型，复制时拷贝整个数据
public struct Point
{
    public double X { get; }
    public double Y { get; }

    public Point(double x, double y) { X = x; Y = y; }

    public double Distance() => Math.Sqrt(X * X + Y * Y);
}

// 接口 + 默认实现（C# 8+）
public interface ILogger
{
    void Log(string msg);
    void LogError(string msg) => Log($"[ERROR] {msg}"); // 默认实现，实现类可选择性重写
}

public class ConsoleLogger : ILogger
{
    public void Log(string msg) => Console.WriteLine($"  [log] {msg}");
}
