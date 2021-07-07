using System;
using System.Text;

namespace TCPClient{
    class Program{
        static void Main(string[] args){

            TCPClient client = new TCPClient();
            client.Start();

            Console.WriteLine("TCPClient::Program::Main...");
            
            while (true)
            {
                string input = Console.ReadLine();
                client.Send( Encoding.UTF8.GetBytes(input) );
            }
        }
    }
}
