// 泛型：泛型方法 / 泛型类 / 约束 where / 默认值 / 协变逆变
// 对应 Go golog generic_test.go（Go 1.18+）、C++ cppapp StdTemplate、Java 泛型

public static class GenericDemo
{
    public static void Run()
    {
        // ===== 1. 泛型方法（对应 Go 泛型函数 / C++ 模板函数）=====
        Console.WriteLine("--- 泛型方法 ---");

        Console.WriteLine($"Max(3, 8) = {Max(3, 8)}");                    // Max(3, 8) = 8（T = int）
        Console.WriteLine($"Max(abc, abd) = {Max("abc", "abd")}");        // Max(abc, abd) = abd（T = string）

        // 类型推断：无需显式写 <string, int>
        var pair = MakePair("key", 666);
        Console.WriteLine($"MakePair = {pair}"); // MakePair = [key, 666]

        // ===== 2. 泛型类（对应 Go 泛型 struct / C++ 模板类 / Java 泛型类）=====
        Console.WriteLine("--- 泛型类 ---");

        var stack = new SimpleStack<int>();
        stack.Push(1); stack.Push(2); stack.Push(3);
        Console.WriteLine($"弹出顺序 = {stack.Pop()} -> {stack.Pop()}（LIFO）"); // 弹出顺序 = 3 -> 2（LIFO）
        Console.WriteLine($"栈大小 = {stack.Count}"); // 栈大小 = 1

        // ===== 3. 约束 where（对应 Go 接口约束 / Java 泛型上界）=====
        Console.WriteLine("--- 约束 ---");

        // where T : struct 值类型约束
        Console.WriteLine($"WrapNullable(42) = {WrapNullable(42)}"); // WrapNullable(42) = 42

        // where T : notnull 非空约束（引用或值类型）
        Save(new User(1, "laixhe"));

        // where T : IComparable 接口约束（Max 内部依赖 CompareTo）

        // ===== 4. 默认值与泛型静态字段 =====
        Console.WriteLine("--- 默认值 ---");

        Console.WriteLine($"default(int)={default(int)} default(string)=|{default(string)}| default(bool)={default(bool)}");

        // 泛型静态字段：每种 T 各一份独立计数（对应 C++ 模板静态成员）
        Counter<int>.Count++;
        Counter<int>.Count++;
        Counter<string>.Count++;
        Console.WriteLine($"Counter<int>.Count = {Counter<int>.Count}，Counter<string>.Count = {Counter<string>.Count}"); // Counter<int>.Count = 2，Counter<string>.Count = 1

        // ===== 5. 协变 / 逆变（对应 Java ? extends / ? super）=====
        Console.WriteLine("--- 协变与逆变 ---");

        // 协变 out：IEnumerable<string> 可当作 IEnumerable<object>（只读）
        IEnumerable<string> strs = ["a", "b"];
        IEnumerable<object> objs = strs;
        Console.WriteLine($"协变: IEnumerable<object> 元素数 = {objs.Count()}"); // 协变: IEnumerable<object> 元素数 = 2

        // 逆变 in：Action<object> 可当作 Action<string>（只写入参）
        Action<object> printObj = o => Console.WriteLine($"  逆变打印: {o}");
        Action<string> printStr = printObj;
        printStr("hello");
    }

    // 泛型方法 + IComparable<T> 约束（对应 C++ template<typename T>）
    private static T Max<T>(T a, T b) where T : IComparable<T> =>
        a.CompareTo(b) >= 0 ? a : b;

    private static KeyValuePair<TK, TV> MakePair<TK, TV>(TK k, TV v) => new(k, v);

    // where T : struct 值类型约束（返回可空包装）
    private static T? WrapNullable<T>(T value) where T : struct => value;

    private static void Save<T>(T entity) where T : notnull =>
        Console.WriteLine($"保存实体（泛型 {typeof(T).Name}）");
}

// 泛型类：简单栈（LIFO），where T : notnull 约束
public class SimpleStack<T> where T : notnull
{
    private readonly List<T> _items = [];

    public int Count => _items.Count;

    public void Push(T item) => _items.Add(item);

    public T Pop()
    {
        T item = _items[^1]; // ^1 从末尾取（C# 8 索引语法）
        _items.RemoveAt(_items.Count - 1);
        return item;
    }
}

// 记录类型（record）：用于演示泛型参数
public record User(int Id, string Name);

// 泛型静态类：每种 T 拥有独立的静态字段
public static class Counter<T>
{
    public static int Count;
}
