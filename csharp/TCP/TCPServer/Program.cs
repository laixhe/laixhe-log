using System;

namespace TCPServer {
    class Program {
        static void Main(string[] args){

            TCPServer server = new TCPServer();
            server.Start();

            Console.WriteLine("TCPServer::Program::Main...");
            Console.ReadLine();
        }
    }
}
