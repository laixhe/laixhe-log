// 位运算：与或非异或 / 移位 / 常用技巧 / BitArray
// 对应 C++ cppapp StdBits、Go / Rust 位运算
using System.Collections;

public static class BitsDemo
{
    public static void Run()
    {
        // ===== 1. 基本位运算（对应 C++ & | ^ ~ << >>）=====
        Console.WriteLine("--- 基本位运算 ---");

        int a = 0b1100; // 12
        int b = 0b1010; // 10
        Console.WriteLine($"a = {a,2}（{ToBin(a)}），b = {b,2}（{ToBin(b)}）"); // a = 12（00001100），b = 10（00001010）
        Console.WriteLine($"a & b = {a & b,2}（{ToBin(a & b)}）  按位与：都为 1 才为 1"); // 8（00001000）
        Console.WriteLine($"a | b = {a | b,2}（{ToBin(a | b)}）  按位或：有 1 就为 1"); // 14（00001110）
        Console.WriteLine($"a ^ b = {a ^ b,2}（{ToBin(a ^ b)}）  异或：不同为 1"); // 6（00000110）
        Console.WriteLine($"a << 1 = {a << 1,2}（{ToBin(a << 1)}） 左移 1 位 = 乘 2"); // 24（00011000）
        Console.WriteLine($"a >> 1 = {a >> 1,2}（{ToBin(a >> 1)}） 右移 1 位 = 除 2"); // 6（00000110）
        Console.WriteLine($"~a = {~a}（{ToBin(~a)}） 按位取反"); // -13（11111111...110011，32 位补码）

        // ===== 2. 常用技巧 =====
        Console.WriteLine("--- 位运算技巧 ---");

        // 判断奇偶：n & 1
        Console.WriteLine($"13 是奇数? {((13 & 1) == 1)}，14 是偶数? {((14 & 1) == 0)}"); // 13 是奇数? True，14 是偶数? True

        // 异或交换两数（无需临时变量）
        int x = 3, y = 5;
        x ^= y; y ^= x; x ^= y;
        Console.WriteLine($"异或交换后: x = {x}, y = {y}"); // 异或交换后: x = 5, y = 3

        // 2 的幂判断：n > 0 且 n & (n-1) == 0
        Console.WriteLine($"16 是 2 的幂? {IsPowerOfTwo(16)}，12 是 2 的幂? {IsPowerOfTwo(12)}"); // 16 是 2 的幂? True，12 是 2 的幂? False

        // 取最低位 1（lowbit，树状数组 / Fenwick 树核心）
        Console.WriteLine($"lowbit(12) = {12 & -12}（{ToBin(12 & -12)}）"); // lowbit(12) = 4（00000100）

        // 统计二进制 1 的个数（对应 C++ std::bitset::count / Java Integer.bitCount）
        int bits = 0b1101_0010; // 210
        Console.WriteLine($"bits = {bits} 二进制 1 的个数 = {int.PopCount(bits)}（.NET 8+ PopCount）"); // bits = 210 二进制 1 的个数 = 4（.NET 8+ PopCount）

        // ===== 3. 位集合 BitArray（对应 C++ std::bitset / Python bitarray）=====
        Console.WriteLine("--- BitArray ---");

        var flags = new BitArray(8); // 8 位，默认全 0
        flags[1] = true;
        flags[3] = true;
        flags[5] = true;
        Console.WriteLine($"长度 = {flags.Length}，索引 3 = {flags[3]}，索引 0 = {flags[0]}"); // 长度 = 8，索引 3 = True，索引 0 = False

        // ===== 4. 进制打印 =====
        Console.WriteLine("--- 进制 ---");

        Console.WriteLine($"255 的 hex = 0x{255:X}，bin = {Convert.ToString(255, 2)}"); // 255 的 hex = 0xFF，bin = 11111111
    }

    private static bool IsPowerOfTwo(int n) => n > 0 && (n & (n - 1)) == 0;

    // 打印 8 位定宽二进制字符串（负数时按实际位宽输出）
    private static string ToBin(int v)
    {
        string s = Convert.ToString(v, 2);
        return s.Length <= 8 ? s.PadLeft(8, '0') : s;
    }
}
