// 异常处理：try-catch-finally / 自定义异常 / using 释放资源 / 异常链
// 对应 Python pylog exceptions.py、Java try-catch（Go 则是 error 返回值风格）

public static class ExceptionDemo
{
    public static void Run()
    {
        // ===== 1. try-catch-finally（对应 Java / Python 异常机制）=====
        Console.WriteLine("--- try / catch / finally ---");

        try
        {
            int.Parse("not_a_number"); // 抛 FormatException
        }
        catch (FormatException)
        {
            Console.WriteLine("捕获 FormatException"); // 捕获 FormatException
        }
        catch (Exception ex) // 兜底：多级 catch 从上到下匹配
        {
            Console.WriteLine($"其它异常: {ex.Message}");
        }
        finally
        {
            Console.WriteLine("finally 无论是否异常都会执行"); // finally 无论是否异常都会执行
        }

        // ===== 2. 自定义异常 + throw（对应 Python raise / Java throw）=====
        Console.WriteLine("--- throw ---");

        static double Divide(double a, double b) =>
            b == 0 ? throw new DivideByZeroException("除数不能为 0") : a / b;

        try
        {
            _ = Divide(1, 0);
        }
        catch (DivideByZeroException ex)
        {
            Console.WriteLine($"捕获: {ex.Message}"); // 捕获: 除数不能为 0
        }

        // 业务校验异常（对应 Java IllegalArgumentException / Python ValueError）
        try
        {
            Deposit(-100);
        }
        catch (ArgumentException ex)
        {
            Console.WriteLine($"业务校验失败: {ex.Message}"); // 业务校验失败: 存款金额必须为正数 (Parameter 'amount')
        }

        // ===== 3. using 释放资源（对应 Java try-with-resources / Python with）=====
        Console.WriteLine("--- using 释放资源 ---");

        string tmpFile = Path.Combine(Path.GetTempPath(), $"exception_demo_{Guid.NewGuid():N}.txt");
        try
        {
            using (var writer = new StreamWriter(tmpFile)) // using 语句：作用域结束自动 Dispose
            {
                writer.WriteLine("hello using");
            }
            Console.WriteLine($"写入完成，文件存在 = {File.Exists(tmpFile)}"); // 写入完成，文件存在 = True

            using var reader = new StreamReader(tmpFile); // using 声明：离开作用域自动释放
            Console.WriteLine($"读取内容 = {reader.ReadToEnd().Trim()}"); // 读取内容 = hello using
        }
        finally
        {
            if (File.Exists(tmpFile)) File.Delete(tmpFile); // 清理临时文件
        }

        // ===== 4. 异常链 InnerException（对应 Java cause / Python __cause__）=====
        Console.WriteLine("--- 异常链 ---");

        try
        {
            try
            {
                throw new InvalidOperationException("内层错误");
            }
            catch (InvalidOperationException ex)
            {
                throw new ApplicationException("外层包装", ex); // 传入原始异常
            }
        }
        catch (ApplicationException ex)
        {
            Console.WriteLine($"外层: {ex.Message}"); // 外层: 外层包装
            Console.WriteLine($"内层: {ex.InnerException?.Message}"); // 内层: 内层错误
        }
    }

    // 业务方法：参数校验失败抛异常
    private static void Deposit(int amount)
    {
        if (amount <= 0)
            throw new ArgumentException("存款金额必须为正数", nameof(amount));
    }
}
