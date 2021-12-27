using System;
using System.Net.Sockets;

namespace TCPClient{
    class TCPClient{

        // TCP 客户端
        TcpClient client;

        public void Start(){
            client = new TcpClient();
            // 连接服务器
            Connect();
        }

        // 连接服务器
        async void Connect(){
            try{
                await client.ConnectAsync("192.168.10.240", 5050);
                Console.WriteLine("连接服务器OK...");
            }
            catch (Exception err){
                Console.WriteLine("connect error:{0}", err.Message);
            }
        }

        // 发送消息给服务器
        public async void Send(byte[] data){
            // 要客户端处于连接状态，才能够发送消息
            if (client.Connected){
                try{ 
                    // 异步的写入数据发给服务器
                    await client.GetStream().WriteAsync(data, 0, data.Length);
                    Console.WriteLine("发送成功...");
                }
                catch (Exception err){
                    Console.WriteLine("send error:{0}", err.Message);
                    client.Close();
                }
            }
        }
            
    }
}
