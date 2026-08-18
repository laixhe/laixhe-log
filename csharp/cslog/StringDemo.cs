// 字符串：常用方法 / 码点与码元 / 拼接与格式化
// 对应 Go golog string_test.go、Rust rustlog char_string.rs
using System.Text;

public static class StringDemo
{
    public static void Run()
    {
        // ===== 1. 常用方法（对应 Go strings 包）=====
        Console.WriteLine("--- 常用方法 ---");

        string s = "hello, world";
        // 是否包含 / 是否以前缀开头
        Console.WriteLine($"Contains('world') = {s.Contains("world")}");
        Console.WriteLine($"StartsWith('he') = {s.StartsWith("he")}");
        // 大小写转换
        Console.WriteLine($"ToUpper = {s.ToUpper()}");
        // 替换 / 截取
        Console.WriteLine($"Replace('l', 'L') = {s.Replace('l', 'L')}");
        Console.WriteLine($"Substring(7) = {s.Substring(7)}"); // world
        // 分割 / 拼接（对应 strings.Split / Join）
        string[] parts = "a,b,c".Split(',');
        Console.WriteLine($"Split = {string.Join(" | ", parts)}");
        // 去空白 / 去字符
        Console.WriteLine($"Trim = |{string.Concat("  pad  ".Trim())}|");

        // ===== 2. 码元与码点（对应 Go byte/rune 的区别、Rust chars）=====
        Console.WriteLine("--- 码元与码点 ---");

        string text = "Hello 世界 🌍";
        // string 长度 = UTF-16 码元数量（一个 Emoji = 2 个码元）
        Console.WriteLine($"Length（UTF-16 码元）= {text.Length}");
        // 实际字符（码点）数量：Emoji 算 1 个字符
        Console.WriteLine($"字符数（码点）= {text.EnumerateRunes().Count()}");

        // 按码点遍历（对应 Rust chars()）
        foreach (Rune r in text.EnumerateRunes())
        {
            Console.Write($"{r} ");
        }
        Console.WriteLine();

        // 码点转字符串：增补平面 Emoji（对应 Go string(rune)）
        Console.WriteLine($"U+1F600 = {Rune.GetRuneAt("\U0001F600", 0)}");

        // ===== 3. 拼接与格式化（对应 Java StringBuilder / Rust String）=====
        Console.WriteLine("--- 拼接与格式化 ---");

        // StringBuilder：循环拼接场景远快于 +=
        var sb = new StringBuilder();
        for (int i = 0; i < 3; i++)
        {
            sb.Append("item").Append(i).Append(", ");
        }
        Console.WriteLine($"StringBuilder = {sb}");

        // 插值字符串（对应 Rust format! / Go fmt.Sprintf / Java String.format）
        string name = "laixhe";
        int age = 18;
        Console.WriteLine($"$插值: name={name} age={age}");
        Console.WriteLine($"string.Format: name={string.Format("{0} is {1}", name, age)}");

        // 词频统计（对应 Go map[string]int / TS Map）
        Console.WriteLine("--- 词频统计 ---");
        string sentence = "the quick brown fox jumps over the lazy dog the";
        var freq = new Dictionary<string, int>();
        foreach (string word in sentence.Split(' '))
        {
            freq[word] = freq.GetValueOrDefault(word) + 1;
        }
        Console.WriteLine($"the = {freq["the"]} 次, dog = {freq["dog"]} 次");
    }
}
