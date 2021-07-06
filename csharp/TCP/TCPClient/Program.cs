using System;

namespace TCPClient{
    class Program{
        static void Main(string[] args){

            TCPClient client = new TCPClient();
            client.Start();

            Console.WriteLine("TCPClient::Program::Main...");
            Console.ReadLine();
        }
    }
}
