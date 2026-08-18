// 泛型进阶：约束全解 / 装箱性能 / 泛型接口 / 自定义协变逆变 / 泛型数学 / 泛型缓存
// 对应 Go golog generic_test.go、C++ 模板约束与特化、Java 泛型上界与通配符
using System.Collections;
using System.Numerics;
using System.Runtime.CompilerServices;

public static class GenericAdvDemo
{
    public static void Run()
    {
        // ===== 1. 约束全解（对应 C++ 模板约束 / Java 泛型上界）=====
        Console.WriteLine("--- 约束全解 ---");

        // class + new()：仅引用类型，且必须可无参构造（对应 Java T extends Object + new T()）
        Logger logger = CreateInstance<Logger>();
        Console.WriteLine($"class + new() 约束: {logger.GetType().Name}"); // class + new() 约束: Logger

        // 基类约束：T 必须继承自 Shape（对应 Java T extends Shape / C++ 继承约束）
        PrintShape(new Circle(5));

        // unmanaged 约束：仅非托管类型（值类型且不含引用字段），可 sizeof
        Console.WriteLine($"unmanaged 约束: sizeof(Point) = {SizeOfStruct<Point>()} 字节"); // unmanaged 约束: sizeof(Point) = 16 字节

        // ===== 2. 泛型与性能：避免装箱（对应 Java 泛型擦除 / C++ 模板零开销）=====
        Console.WriteLine("--- 装箱与性能 ---");

        // 非泛型 ArrayList 存 int：值类型装箱为 object，取出要拆箱
        var boxed = new ArrayList { 1, 2, 3 };
        int unbox = (int)boxed[0]!;
        Console.WriteLine($"ArrayList 装箱存取 = {unbox}（int -> object -> int）"); // ArrayList 装箱存取 = 1（int -> object -> int）

        // 泛型 List<int>：运行时类型就是 int，零装箱零拆箱
        var typed = new List<int> { 1, 2, 3 };
        Console.WriteLine($"List<int> 无装箱 = {typed[0]}（存储的就是 int）"); // List<int> 无装箱 = 1（存储的就是 int）

        // ===== 3. 泛型接口：IComparable<T> / IEquatable<T> =====
        Console.WriteLine("--- 泛型接口 ---");

        var products = new List<Product>
        {
            new("laptop", 9999), new("phone", 6999), new("mouse", 99),
        };
        products.Sort(); // 依赖 IComparable<Product>
        Console.WriteLine($"按价格排序: {string.Join(" -> ", products.Select(p => $"{p.Name}({p.Price})"))}"); // 按价格排序: mouse(99) -> phone(6999) -> laptop(9999)

        // 泛型默认比较器（对应 Java Objects.equals / C++ std::equal_to）
        var pa = new Product("mouse", 99);
        var pb = new Product("mouse", 99);
        Console.WriteLine($"EqualityComparer 值相等: {EqualityComparer<Product>.Default.Equals(pa, pb)}"); // EqualityComparer 值相等: True

        // ===== 4. 自定义协变 / 逆变接口（深入 out / in）=====
        Console.WriteLine("--- 自定义协变逆变 ---");

        // out T：只能出现在返回值位置（生产者），IProducer<Dog> 可当 IProducer<Animal>
        IProducer<Dog> dogProducer = new Producer<Dog>(() => new Dog("小黑"));
        IProducer<Animal> animalProducer = dogProducer; // 协变：Dog -> Animal
        Console.WriteLine($"协变: {animalProducer.Produce().Name}"); // 协变: 小黑

        // in T：只能出现在参数位置（消费者），IConsumer<Animal> 可当 IConsumer<Dog>
        IConsumer<Animal> animalConsumer = new Consumer<Animal>(a => Console.WriteLine($"  消费 {a.Name}"));
        IConsumer<Dog> dogConsumer = animalConsumer; // 逆变：Animal -> Dog
        dogConsumer.Consume(new Dog("小黄")); // 消费 小黄

        // ===== 5. 泛型数学 INumber<T>（.NET 7+，对应 C++ 模板数值函数）=====
        Console.WriteLine("--- 泛型数学 ---");

        Console.WriteLine($"Sum([1,2,3]) = {Sum([1, 2, 3])}"); // Sum([1,2,3]) = 6
        Console.WriteLine($"Sum([1.5, 2.5]) = {Sum([1.5, 2.5])}"); // Sum([1.5, 2.5]) = 4

        // ===== 6. 泛型缓存：每种 T 独立一份（对应 C++ 模板静态成员）=====
        Console.WriteLine("--- 泛型缓存 ---");

        ConfigCache<DbConfig>.Value = new("db=localhost");
        ConfigCache<CacheConfig>.Value = new("cache=redis");
        Console.WriteLine($"ConfigCache<DbConfig> = {ConfigCache<DbConfig>.Value}"); // ConfigCache<DbConfig> = DbConfig { Connection = db=localhost }
        Console.WriteLine($"ConfigCache<CacheConfig> = {ConfigCache<CacheConfig>.Value}（互不影响）"); // ConfigCache<CacheConfig> = CacheConfig { Endpoint = cache=redis }（互不影响）
    }

    // where T : class, new()：引用类型 + 可无参构造（依赖工厂）
    private static T CreateInstance<T>() where T : class, new() => new();

    // where T : unmanaged：非托管类型约束，配合 sizeof / 指针使用
    private static int SizeOfStruct<T>() where T : unmanaged => Unsafe.SizeOf<T>();

    // 泛型数学：任何实现 INumber<T> 的数值类型都可求和
    private static T Sum<T>(IEnumerable<T> values) where T : INumber<T> =>
        values.Aggregate(T.Zero, (acc, v) => acc + v);

    // 基类约束：T 必须是 Shape 的子类
    private static void PrintShape<T>(T shape) where T : Shape =>
        Console.WriteLine($"基类约束: {shape.GetType().Name} 面积 = {shape.Area():F2}"); // 基类约束: Circle 面积 = 78.54
}

// 用于演示 class+new() 约束的普通类
public class Logger
{
}

// 用于演示 IComparable<T> 的自定义类型（按价格排序）
public record Product(string Name, int Price) : IComparable<Product>
{
    public int CompareTo(Product? other) => other is null ? 1 : Price.CompareTo(other.Price);
}

// 自定义协变接口：out T 只能出现在返回值位置（生产者）
public interface IProducer<out T>
{
    T Produce();
}

// 自定义逆变接口：in T 只能出现在参数位置（消费者）
public interface IConsumer<in T>
{
    void Consume(T item);
}

public class Producer<T> : IProducer<T>
{
    private readonly Func<T> _factory;
    public Producer(Func<T> factory) => _factory = factory;
    public T Produce() => _factory();
}

public class Consumer<T> : IConsumer<T>
{
    private readonly Action<T> _action;
    public Consumer(Action<T> action) => _action = action;
    public void Consume(T item) => _action(item);
}

// 泛型缓存：每种 T 拥有独立静态字段（单例式缓存容器）
public static class ConfigCache<T>
{
    public static T? Value { get; set; }
}

// 演示用配置类型
public record DbConfig(string Connection);
public record CacheConfig(string Endpoint);
