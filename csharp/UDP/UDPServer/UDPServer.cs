using System;
using System.Text;
using System.Net.Sockets;

namespace UDPServer {
    class UDPServer {

        UdpClient udpClient;

        public void Start(){
            // 创建 udp 通信
            udpClient = new UdpClient(5050);

            Console.WriteLine("UDP服务启动...");
        }

        // 接收来自客户端的消息
        private async void Receive(){
        }

        // 发送消息给客户端
        public async void Send(byte[] data){
        }
    }
}
