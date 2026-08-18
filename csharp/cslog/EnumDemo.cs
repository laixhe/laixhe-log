// 枚举与位标志：enum / [Flags] / 解析与转换 / 遍历
// 对应 C++ cppapp StdEnum、Java enum、Python Enum

public static class EnumDemo
{
    // 枚举：底层默认 int（对应 C++ enum class / Java enum）
    private enum Color { Red = 1, Green = 2, Blue = 3 }

    // [Flags] 位标志枚举：值是 2 的幂，可任意组合（对应 C++ 位掩码 / Go iota）
    [Flags]
    private enum Permission
    {
        None = 0,
        Read = 1,
        Write = 2,
        Execute = 4,
        All = Read | Write | Execute
    }

    public static void Run()
    {
        // ===== 1. 基本使用 =====
        Console.WriteLine("--- 基本使用 ---");

        Color c = Color.Green;
        Console.WriteLine($"值 = {c}，整数 = {(int)c}，名称 = {c}"); // 值 = Green，整数 = 2，名称 = Green

        // 数值 -> 枚举（对应 C++ 强转 / Java 无直接写法）
        Color fromInt = (Color)2;
        Console.WriteLine($"(Color)2 = {fromInt}"); // (Color)2 = Green

        // 字符串 -> 枚举（TryParse 解析失败返回 false，对应 Java Enum.valueOf）
        bool ok = Enum.TryParse("Blue", out Color parsed);
        Console.WriteLine($"TryParse('Blue') = {ok} -> {parsed}"); // TryParse('Blue') = True -> Blue

        // 遍历所有枚举值（对应 Python 迭代 Enum / Java values()）
        foreach (Color item in Enum.GetValues<Color>())
        {
            Console.WriteLine($"  {item} = {(int)item}");
        }

        // ===== 2. [Flags] 位标志组合 =====
        Console.WriteLine("--- [Flags] 位标志 ---");

        Permission p = Permission.Read | Permission.Execute; // 组合权限
        Console.WriteLine($"组合 = {p}"); // 组合 = Read, Execute
        Console.WriteLine($"包含 Read? {p.HasFlag(Permission.Read)}，包含 Write? {p.HasFlag(Permission.Write)}"); // 包含 Read? True，包含 Write? False

        p |= Permission.Write; // 追加权限
        Console.WriteLine($"追加 Write 后 = {p}"); // 追加 Write 后 = All

        p &= ~Permission.Read; // 移除权限
        Console.WriteLine($"移除 Read 后 = {p}"); // 移除 Read 后 = Write, Execute

        // 手写位运算判断（等价 HasFlag）
        Console.WriteLine($"位与判断 Execute = {(p & Permission.Execute) != 0}"); // 位与判断 Execute = True

        // 整数 -> 标志（典型场景：数据库 int 字段存储权限）
        Permission fromDb = (Permission)6; // 6 = 110₂ = Read(1) | Execute(4)
        Console.WriteLine($"数据库读回 (Permission)6 = {fromDb}"); // 数据库读回 (Permission)6 = Write, Execute
    }
}
