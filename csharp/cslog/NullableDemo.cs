// 可空类型与空安全：可空值类型 / 空合并 / 空传播 / NRT 编译期分析
// 对应 C++ cppapp StdOptional、Rust Option、Java Optional

public static class NullableDemo
{
    public static void Run()
    {
        // ===== 1. 可空值类型 int?（对应 C++ optional<int> / Rust Option<i32>）=====
        Console.WriteLine("--- 可空值类型 ---");

        int? age = null; // 等价 default(int?)
        Console.WriteLine($"HasValue = {age.HasValue}，GetValueOrDefault = {age.GetValueOrDefault()}"); // HasValue = False，GetValueOrDefault = 0

        age = 18;
        Console.WriteLine($"赋值后: HasValue = {age.HasValue}，Value = {age.Value}"); // 赋值后: HasValue = True，Value = 18

        // 判断与取值（对应 Rust match Option / Java isPresent）
        int? maybe = 42;
        if (maybe.HasValue)
        {
            Console.WriteLine($"HasValue 分支: {maybe.Value}"); // HasValue 分支: 42
        }

        // ===== 2. 空合并 ??（对应 Rust unwrap_or / Python or）=====
        Console.WriteLine("--- 空合并 ---");

        int? score = null;
        int result = score ?? 0; // 为空则用默认值
        Console.WriteLine($"score ?? 0 = {result}"); // score ?? 0 = 0

        score ??= 100; // 为空则赋值（空合并赋值）
        Console.WriteLine($"score ??= 100 后 = {score}"); // score ??= 100 后 = 100

        string? name = null;
        Console.WriteLine($"name ?? \"匿名\" = {name ?? "匿名"}"); // name ?? "匿名" = 匿名

        // ===== 3. 空传播 ?.（对应 Kotlin ?. / TS 可选链）=====
        Console.WriteLine("--- 空传播 ---");

        string? user = "laixhe";
        Console.WriteLine($"user?.Length = {user?.Length}"); // user?.Length = 6

        user = null;
        Console.WriteLine($"null?.Length 返回 null 不抛异常，?? 合并后 = {user?.Length ?? 0}"); // null?.Length 返回 null 不抛异常，?? 合并后 = 0

        // 链式调用：任一层为 null 即短路返回 null
        Console.WriteLine($"user?.ToUpper()?.Substring(0, 1) ?? \"空\" = {user?.ToUpper()?.Substring(0, 1) ?? "空"}"); // = 空（短路返回 null）

        // ===== 4. 可空引用类型（NRT，编译期空安全分析）=====
        Console.WriteLine("--- 可空引用类型 ---");

        string? maybeNull = null;
        string safe = maybeNull ?? "fallback";
        Console.WriteLine($"NRT 合并 = {safe}"); // NRT 合并 = fallback

        // 空断言 !：仅在确信非空时使用（对应 TS 非空断言）
        string? definitelyNotNull = "ok";
        Console.WriteLine($"空断言 Length = {definitelyNotNull!.Length}"); // 空断言 Length = 2

        // ===== 5. 实战：数据库"未设置"字段 =====
        Console.WriteLine("--- 实战：可空字段 ---");

        SaveToDb(null); // 写入数据库: NULL
        SaveToDb(99);   // 写入数据库: 99
    }

    // 空值表达"未设置"语义（对应 Go sql.NullInt64 / Java Optional 空值）
    private static void SaveToDb(int? value) =>
        Console.WriteLine($"写入数据库: {value?.ToString() ?? "NULL"}");
}
