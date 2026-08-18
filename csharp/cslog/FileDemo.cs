// 文件与目录 IO：一次性读写 / 追加 / 流式读写 / 二进制 / 目录遍历
// 对应 Go golog fileio_test.go、C++ cppapp StdFile

public static class FileDemo
{
    public static void Run()
    {
        // 演示目录（结束时递归清理）
        string dir = Path.Combine(Directory.GetCurrentDirectory(), ".cslog_tmp");
        string file = Path.Combine(dir, "demo.txt");

        try
        {
            // ===== 1. 一次性读写（对应 Go os.WriteFile / os.ReadFile）=====
            Console.WriteLine("--- 一次性读写 ---");

            Directory.CreateDirectory(dir);        // 创建目录（已存在不报错）
            File.WriteAllText(file, "第一行\n第二行\n"); // 覆盖写入
            string content = File.ReadAllText(file);    // 整体读入
            Console.WriteLine($"ReadAllText = {content.Trim().Replace("\n", " / ")}"); // ReadAllText = 第一行 / 第二行

            // 按行读取（对应 Go bufio.Scanner / Python readlines）
            string[] lines = File.ReadAllLines(file);
            Console.WriteLine($"行数 = {lines.Length}，首行 = {lines[0]}"); // 行数 = 2，首行 = 第一行

            // ===== 2. 追加写入（对应 Go os.OpenFile 的 O_APPEND）=====
            Console.WriteLine("--- 追加写入 ---");

            File.AppendAllText(file, "第三行\n");
            Console.WriteLine($"追加后总行数 = {File.ReadAllLines(file).Length}"); // 追加后总行数 = 3

            // ===== 3. 流式读写（大文件逐行处理场景）=====
            Console.WriteLine("--- 流式读写 ---");

            using (var writer = new StreamWriter(file, append: false))
            {
                writer.WriteLine("流式写入 1");
                writer.WriteLine("流式写入 2");
            }
            using (var reader = new StreamReader(file))
            {
                string? line;
                while ((line = reader.ReadLine()) is not null)
                {
                    Console.WriteLine($"  read: {line}"); // read: 流式写入 1 / 流式写入 2
                }
            }

            // ===== 4. 二进制读写（对应 Go encoding/binary / Java DataOutputStream）=====
            Console.WriteLine("--- 二进制读写 ---");

            string bin = Path.Combine(dir, "data.bin");
            using (var fs = new FileStream(bin, FileMode.Create))
            using (var bw = new BinaryWriter(fs))
            {
                bw.Write(123);       // int
                bw.Write(45.67);     // double
                bw.Write("c# demo"); // string（自动带长度前缀）
            }
            using (var fs = new FileStream(bin, FileMode.Open))
            using (var br = new BinaryReader(fs))
            {
                Console.WriteLine($"读回 int={br.ReadInt32()} double={br.ReadDouble()} string={br.ReadString()}"); // 读回 int=123 double=45.67 string=c# demo
            }

            // ===== 5. 文件信息与目录遍历（对应 Go os.Stat / filepath.WalkDir）=====
            Console.WriteLine("--- 文件信息 ---");

            var fi = new FileInfo(file);
            Console.WriteLine($"名称={fi.Name} 大小={fi.Length}B 扩展名={fi.Extension}"); // 名称=demo.txt 大小=xxB 扩展名=.txt

            string[] all = Directory.GetFiles(dir); // 列出目录下所有文件
            Console.WriteLine($"目录内文件: {string.Join(", ", all.Select(Path.GetFileName))}"); // 目录内文件: data.bin, demo.txt
        }
        finally
        {
            if (Directory.Exists(dir)) Directory.Delete(dir, recursive: true); // 递归删除
        }
    }
}
