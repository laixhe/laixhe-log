// 正则表达式：匹配 / 提取 / 捕获组 / 替换 / 分割 / 常用选项
// 对应 Go golog regexp_test.go、Python pylog regex.py、C++ cppapp StdRegex
using System.Text.RegularExpressions;

public static class RegexDemo
{
    public static void Run()
    {
        // ===== 1. 基本匹配（对应 Go regexp.MatchString / Python re.search）=====
        Console.WriteLine("--- 基本匹配 ---");

        // 原义字符串 @""：反斜杠无需转义
        string pattern = @"^1[3-9]\d{9}$"; // 中国大陆手机号
        Console.WriteLine($"手机号 13812345678 = {Regex.IsMatch("13812345678", pattern)}"); // 手机号 13812345678 = True
        Console.WriteLine($"手机号 12345 = {Regex.IsMatch("12345", pattern)}"); // 手机号 12345 = False
        Console.WriteLine($"邮箱 abc@example.com = {Regex.IsMatch("abc@example.com", @"^[\w.+-]+@[\w-]+\.\w+$")}"); // 邮箱 abc@example.com = True

        // ===== 2. Matches 提取（对应 Go FindAllString / Python findall）=====
        Console.WriteLine("--- 提取 ---");

        string text = "价格：12.5 元、8.8 元、100 元";
        foreach (Match m in Regex.Matches(text, @"\d+(\.\d+)?"))
        {
            Console.WriteLine($"  找到数字: {m.Value}（位置 {m.Index}）"); // 12.5（位置 3）/ 8.8（位置 10）/ 100（位置 16）
        }

        // ===== 3. 捕获组 + 命名组（对应 Go 子匹配 / Python groupdict）=====
        Console.WriteLine("--- 捕获组 ---");

        Match dm = Regex.Match("2026-08-17", @"^(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})$");
        if (dm.Success)
        {
            Console.WriteLine($"年={dm.Groups["year"].Value} 月={dm.Groups["month"].Value} 日={dm.Groups["day"].Value}"); // 年=2026 月=08 日=17
        }

        // ===== 4. 替换 / 分割（对应 Go ReplaceAllString / Split）=====
        Console.WriteLine("--- 替换与分割 ---");

        // $1 / $2 引用捕获组（对应 Python re.sub 的 \1 / Go 的 ${1}）
        string masked = Regex.Replace("13812345678", @"^(\d{3})\d{4}(\d{4})$", "$1****$2");
        Console.WriteLine($"脱敏手机号 = {masked}"); // 脱敏手机号 = 138****5678

        string[] parts = Regex.Split("a1b22c333", @"\d+");
        Console.WriteLine($"按数字分割 = {string.Join(", ", parts)}"); // 按数字分割 = a, b, c,（末尾空串是 Split 的正常行为）

        // ===== 5. 常用选项与性能 =====
        Console.WriteLine("--- 常用选项 ---");

        // IgnoreCase：忽略大小写
        Console.WriteLine($"忽略大小写 ABC = {Regex.IsMatch("ABC", "abc", RegexOptions.IgnoreCase)}"); // 忽略大小写 ABC = True

        // Multiline：^ $ 匹配每一行的行首行尾（对应 Python re.MULTILINE）
        string multi = "line1\nline2\nline3";
        Console.WriteLine($"多行模式匹配行数 = {Regex.Matches(multi, "^line", RegexOptions.Multiline).Count}"); // 多行模式匹配行数 = 3

        // 预编译 Regex 实例：重复使用避免反复编译（对应 Python 预编译正则）
        var emailRegex = new Regex(@"^[\w.+-]+@[\w-]+\.\w+$", RegexOptions.Compiled);
        Console.WriteLine($"预编译邮箱验证 = {emailRegex.IsMatch("user@mail.com")} / {emailRegex.IsMatch("bad-email")}"); // 预编译邮箱验证 = True / False
    }
}
