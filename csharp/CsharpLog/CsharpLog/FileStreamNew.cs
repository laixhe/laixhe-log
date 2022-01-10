using System;
using System.IO;

namespace CsharpLog
{
    class FileStreamNew {
        public static void Run() {

            // 1. 创建文件流对象
            var fs = new FileStream("./test.txt", FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None);
            
            // 2. 创建相应的 文本读(StreamReader)写(StreamWriter)器
            var sw = new StreamWriter(fs);
            // 2. 创建相应的 二进制读(BinaryReader)写(BinaryWriter)器
            //var bw = new BinaryWriter(fs);

            // 3. 执行读写操作
            sw.Write("fds执行读写操作432");

            sw.Close();
            fs.Close();

        }
    }
}