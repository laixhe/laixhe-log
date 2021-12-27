using System;
using System.Text;
using System.Net.Sockets;

namespace TCPServer {
    class Agent {
        TcpClient client;

        public Agent(TcpClient tcpClient){
            client = tcpClient;

            // 接收来自客户端的消息
            Receive();
        }

        // 接收来自客户端的消息
        private async void Receive(){
            // 要处于连接状态，才能够读取数据
            while (client.Connected){
                try{
                    // 用于缓存接收的数据
                    byte[] buffer = new byte[4096];
                    // 异步接收数据
                    int length = await client.GetStream().ReadAsync(buffer, 0, buffer.Length);

                    // 将字节数组转化为 字符串
                    string str = Encoding.UTF8.GetString(buffer, 0, length);
                    Console.WriteLine("收到：{0}", str);
                }
                catch (Exception err){
                    Console.WriteLine("receive error:{0}", err.Message);
                    client.Close();
                }
            }
        }
    }
}