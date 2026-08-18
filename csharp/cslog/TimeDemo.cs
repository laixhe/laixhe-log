// 时间与日期：DateTime / 格式化 / 解析 / 时区 / 耗时
// 对应 Go golog time_test.go、Java javalog TimeDemo.java

public static class TimeDemo
{
    public static void Run()
    {
        // ===== 1. 当前时间（对应 Go time.Now）=====
        Console.WriteLine("--- 当前时间 ---");

        DateTime now = DateTime.Now;           // 本地时间
        DateTime utcNow = DateTime.UtcNow;     // UTC 时间
        Console.WriteLine($"本地时间: {now:yyyy-MM-dd HH:mm:ss}");
        Console.WriteLine($"UTC 时间: {utcNow:yyyy-MM-dd HH:mm:ss}");

        // ===== 2. 格式化（对应 Go Format / Java SimpleDateFormat）=====
        Console.WriteLine("--- 格式化 ---");

        Console.WriteLine($"yyyy-MM-dd: {now:yyyy-MM-dd}");
        Console.WriteLine($"yyyy年M月d日: {now:yyyy年M月d日}");
        Console.WriteLine($"星期几: {now:dddd}");
        Console.WriteLine($"ISO 8601: {now:o}");

        // ===== 3. 字符串解析（对应 Go Parse）=====
        Console.WriteLine("--- 解析 ---");

        var parsed = DateTime.ParseExact("2026-03-04 15:30", "yyyy-MM-dd HH:mm", null);
        Console.WriteLine($"ParseExact: {parsed:yyyy-MM-dd HH:mm}");

        // ===== 4. 时间运算（对应 Go Add / Sub）=====
        Console.WriteLine("--- 时间运算 ---");

        Console.WriteLine($"明天: {now.AddDays(1):yyyy-MM-dd}");
        Console.WriteLine($"10 分钟后: {now.AddMinutes(10):HH:mm:ss}");
        Console.WriteLine($"1970 至今秒数（Unix 时间戳）: {new DateTimeOffset(now).ToUnixTimeSeconds()}");

        // ===== 5. 时区转换（对应 Go time.LoadLocation）=====
        Console.WriteLine("--- 时区 ---");

        // 时区 ID：'China Standard Time' 是 Windows 专属，Linux/macOS 需用 IANA 名 'Asia/Shanghai'
        var utc = TimeZoneInfo.ConvertTimeToUtc(now);
        var shanghai = TimeZoneInfo.ConvertTimeFromUtc(utc, TimeZoneInfo.FindSystemTimeZoneById("China Standard Time"));
        var ny = TimeZoneInfo.ConvertTimeFromUtc(utc, TimeZoneInfo.FindSystemTimeZoneById("Eastern Standard Time"));
        Console.WriteLine($"上海: {shanghai:yyyy-MM-dd HH:mm}");
        Console.WriteLine($"纽约: {ny:yyyy-MM-dd HH:mm}");

        // ===== 6. 耗时测量（对应 Go time.Since / Stopwatch）=====
        Console.WriteLine("--- 耗时测量 ---");

        var sw = System.Diagnostics.Stopwatch.StartNew();
        Thread.Sleep(50);
        sw.Stop();
        Console.WriteLine($"耗时: {sw.ElapsedMilliseconds} ms");

        // ===== 7. 定时器（对应 Go time.After / Timer）=====
        Console.WriteLine("--- 定时器 ---");

        using var cts = new CancellationTokenSource(TimeSpan.FromMilliseconds(100));
        var timerTask = Task.Delay(1000, cts.Token);
        _ = timerTask.ContinueWith(
            t => Console.WriteLine(t.IsCanceled ? "定时器被取消（未等满 1s）" : "定时器触发"),
            TaskContinuationOptions.ExecuteSynchronously);
        Thread.Sleep(200); // 等待取消回调执行
    }
}
