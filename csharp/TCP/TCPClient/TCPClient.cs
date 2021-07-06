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
                Console.WriteLine("连接服务器ok");
            }
            catch (Exception err){
                Console.WriteLine("connect error:{0}", err.Message);
            }
        }
    }
}
