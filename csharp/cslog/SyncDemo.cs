// 并发与同步：Task / lock / Monitor / Interlocked / 并行
// 对应 Go golog sync_test.go（Once / WaitGroup / Mutex / atomic）、Java SyncDemo.java

public static class SyncDemo
{
    private static int _counter;

    public static void Run()
    {
        // ===== 1. Task.Run 并发（对应 Go goroutine / Java 线程池）=====
        Console.WriteLine("--- Task 并发 ---");

        var tasks = new List<Task>();
        for (int i = 0; i < 5; i++)
        {
            int id = i; // 捕获循环变量
            tasks.Add(Task.Run(() => Console.WriteLine($"任务 {id}")));
        }
        Task.WaitAll(tasks.ToArray()); // 等待全部完成（对应 Go WaitGroup.Wait）
        Console.WriteLine("所有任务完成");

        // ===== 2. lock 互斥（对应 Go Mutex / C# Monitor）=====
        Console.WriteLine("--- lock 互斥 ---");

        var lockObj = new object();
        _counter = 0;
        var lockTasks = new List<Task>();
        for (int i = 0; i < 10; i++)
        {
            lockTasks.Add(Task.Run(() =>
            {
                for (int j = 0; j < 100; j++)
                {
                    lock (lockObj) // 等价于 Monitor.Enter/Exit
                    {
                        _counter++;
                    }
                }
            }));
        }
        Task.WaitAll(lockTasks.ToArray());
        Console.WriteLine($"count = {_counter}（加锁保证 1000）");

        // ===== 3. Interlocked 原子操作（对应 Go atomic / C++ std::atomic）=====
        Console.WriteLine("--- Interlocked 原子操作 ---");

        long atomicCount = 0;
        var atomicTasks = new List<Task>();
        for (int i = 0; i < 10; i++)
        {
            atomicTasks.Add(Task.Run(() =>
            {
                for (int j = 0; j < 100; j++)
                {
                    Interlocked.Increment(ref atomicCount); // 原子自增，无需加锁
                }
            }));
        }
        Task.WaitAll(atomicTasks.ToArray());
        Console.WriteLine($"count = {atomicCount}（原子自增）");

        // ===== 4. Parallel.For 并行（对应 Go 并发循环 / .NET 并行 LINQ）=====
        Console.WriteLine("--- Parallel.For ---");

        int sum = 0;
        Parallel.For(0, 1000, i => Interlocked.Add(ref sum, 1));
        Console.WriteLine($"Parallel.For 累加 = {sum}");

        // ===== 5. 只执行一次（对应 Go sync.Once / Java 静态初始化）=====
        Console.WriteLine("--- Lazy 只执行一次 ---");

        var lazy = new Lazy<int>(() =>
        {
            Console.WriteLine("初始化（仅执行一次）");
            return 42;
        });
        _ = lazy.Value;
        _ = lazy.Value;
        Console.WriteLine($"Lazy value = {lazy.Value}");
    }
}
