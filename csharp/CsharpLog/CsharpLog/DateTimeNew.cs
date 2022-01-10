using System;

namespace CsharpLog
{
    class DateTimeNew {
        public static void Run() {

            // 2021/6/28 11:09:46
            Console.WriteLine("当前时间: {0}", DateTime.Now);
            // 2021-06-28 11:15:45
            Console.WriteLine("当前时间: {0}", DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"));
            // 2021/6/28 0:00:00
            Console.WriteLine("当前时间零点: {0}", DateTime.Today);

            // 2021/6/29 11:09:46
            Console.WriteLine("当前时间加上一天: {0}", DateTime.Now.AddDays(1));

            // 获取当前时间戳
            var timeStamp1 = Convert.ToInt64((DateTime.UtcNow - DateTime.UnixEpoch).TotalSeconds);
            // 1624947342
            Console.WriteLine("获取当前时间戳1: {0}", timeStamp1);

            var timeStamp2 = Convert.ToInt64(DateTime.Now.Subtract(new DateTime(1970, 1, 1)).TotalSeconds);
            // 1624947342
            Console.WriteLine("获取当前时间戳2: {0}", timeStamp2);

            var timeStamp3 = new DateTimeOffset(DateTime.UtcNow).ToUnixTimeSeconds();
            // 1624947342
            Console.WriteLine("获取当前时间戳3: {0}", timeStamp3); // 最好的

            // 时间戳转时间
            var timeStampDate1 = DateTimeOffset.FromUnixTimeSeconds(timeStamp1).DateTime.AddHours(8);
            // 2021-06-29 06:15:42
            Console.WriteLine("时间戳转时间: {0}", timeStampDate1.ToString("yyyy-MM-dd HH:mm:ss"));

            var timeStampDate2 = new DateTime(1970, 1, 1).AddSeconds(timeStamp1).AddHours(8);
            // 2021-06-29 06:15:42
            Console.WriteLine("时间戳转时间: {0}", timeStampDate2.ToString("yyyy-MM-dd HH:mm:ss"));

        }
    }
}