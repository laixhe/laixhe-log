using System;
using System.Net.Sockets;

namespace TCPServer {
    class TCPServer {

        // TCP 服务
        TcpListener tcpListener;

        public void Start(){
            try{
                // 创建 tcp 通信
                tcpListener = TcpListener.Create(5050);
                tcpListener.Start(1024); // 限制连接数量

                Console.WriteLine("TCP服务启动...");

                // Accept 方法进行监听来自客户端的连接
                Accept();
            }
            catch (Exception err){
                Console.WriteLine("start error:{0}", err.Message);
            }
        }

        // 监听客户端连接的接口
        private async void Accept(){
            try{
                Console.WriteLine("等待客户端的连接...");

                // 等待客户端的连接
                TcpClient tcpClient = await tcpListener.AcceptTcpClientAsync();
                // 获取访问连接的客户端的远程IP和端口
                Console.WriteLine("客户端已连接:{0}", tcpClient.Client.RemoteEndPoint);

                Agent agent = new Agent(tcpClient);

                // 重新回调
                Accept();

            }
            catch (Exception err){
                Console.WriteLine("accept error:{0}", err.Message);
                // 停止TCP服务
                tcpListener.Stop();
            }
        }

    }
}
