// 数值进阶：格式化 / 溢出与饱和 / 类型转换
// 对应 Go golog slice/time 风格、Rust rustlog number.rs
using System.Globalization;

public static class NumberDemo
{
    public static void Run()
    {
        // ===== 1. 数值格式化输出（对应 Rust number_to_string）=====
        Console.WriteLine("--- 数值格式化 ---");

        // 精度控制（四舍五入）
        Console.WriteLine($"f1={88.888:F2} f2={88.0:F2}"); // f1=88.89 f2=88.00

        // 十六进制 / 八进制 / 二进制（X 大写，x 小写）
        Console.WriteLine($"666 hex=0x{666:X} octal=0o{Convert.ToString(666, 8)} binary=0b{Convert.ToString(666, 2)}");
        // 666 hex=0x29A octal=0o1232 binary=0b1010011010

        // 前导零填充 + 宽度控制（D8 补零）
        Console.WriteLine($"leading zeros: {666:D8}"); // 00000666

        // 对齐：-10 左对齐，10 右对齐
        Console.WriteLine($"left=|{666,-10}| right=|{666,10}|");
        // left=|666       | right=|       666|

        // 正负号显式显示（自定义格式 "+0;-0"）
        Console.WriteLine($"positive={666.ToString("+0;-0")}  negative={(-888).ToString("+0;-0")}");
        // positive=+666  negative=-888

        // 千分位分组（N0 = Number + 0 位小数）
        Console.WriteLine($"grouping: {1234567.ToString("N0", CultureInfo.InvariantCulture)}");
        // grouping: 1,234,567

        // ===== 2. 整数溢出（对应 Rust overflow / Go 溢出检查）=====
        Console.WriteLine("--- 整数溢出 ---");

        // 无符号回绕：byte 255 + 1 = 0（unchecked 下无符号溢出是定义行为，会回绕）
        byte b = 255;
        Console.WriteLine($"u8 255 + 1 = {(byte)(b + 1)}（回绕到 0）");

        // C# 默认 unchecked；checked 关键字开启溢出检查（对应 Go 溢出检查 / Rust debug 检查）
        try
        {
            int max = int.MaxValue;
            checked { Console.WriteLine(max + 1); }
        }
        catch (OverflowException)
        {
            Console.WriteLine("checked: MAX + 1 = 抛 OverflowException");
        }

        // 饱和（对应 Rust saturating_add）：先用更大类型计算再 clamp
        long wide = (long)int.MaxValue + 1; // long（64 位）拓宽，不会溢出
        int sat = (int)Math.Clamp(wide, int.MinValue, int.MaxValue);
        Console.WriteLine($"saturating_add: MAX + 1 = {sat}（饱和）");

        // 浮点精度：0.1 + 0.2 != 0.3（IEEE 754）
        Console.WriteLine($"0.1 + 0.2 = {0.1 + 0.2}（浮点精度问题）");

        // ===== 3. 类型转换（对应 Rust type_conversion）=====
        Console.WriteLine("--- 类型转换 ---");

        // 浮点转整数：(int) 强转向零截断
        Console.WriteLine($"int(3.99) = {(int)3.99}（向零截断）");
        // 四舍五入：Math.Round 先取整再转换
        Console.WriteLine($"round(3.99) = {(int)Math.Round(3.99)}");

        // 字符串解析：int.Parse / double.Parse（失败抛 FormatException）
        Console.WriteLine($"int.Parse('666') = {int.Parse("666")}");
        Console.WriteLine($"double.Parse('88.88') = {double.Parse("88.88")}");

        // 进制解析（对应 Go strconv.ParseInt 指定 base / Java Integer.parseInt(s, radix)）
        Console.WriteLine($"Convert.ToInt32('29A', 16) = {Convert.ToInt32("29A", 16)}"); // 666
        Console.WriteLine($"Convert.ToInt32('1232', 8) = {Convert.ToInt32("1232", 8)}"); // 666
        Console.WriteLine($"Convert.ToString(666, 16) = {Convert.ToString(666, 16)}");   // 29a

        // 解析失败：TryParse 返回 false（区别于 Java 抛异常 / PHP 返回 0）
        Console.WriteLine($"int.TryParse('not_a_number') = {int.TryParse("not_a_number", out _)}");
    }
}
