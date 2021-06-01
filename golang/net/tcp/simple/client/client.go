package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"net"
	"os"
)

// 将命令行的参数解析到全局变量
var (
	addr = flag.String("addr", "127.0.0.1:8000", "服务端监听IP端口")
)

func main() {

	// 解析命令行的参数
	flag.Parse()

	// 向服务端建立链接
	conn, err := net.Dial("tcp", *addr)
	if err != nil {
		log.Fatal("无法建立链接：", err)
	}
	defer conn.Close() //关闭链接

	log.Println("链接建立成功！")

	// 命令行交互
	scanner := bufio.NewScanner(os.Stdin)
	go func() {

		for scanner.Scan() {
			line := scanner.Bytes()
			_, err := conn.Write(line)
			if err != nil {
				log.Println("向服务发送数据失败!", err)
				return
			}
		}

	}()

	for{


		// 定义一个切片用于接收到的内容
		readData := make([]byte, 1024)

		// 接收服务端发过来的数据
		rLen, err := conn.Read(readData)
		if err != nil {
			log.Println("接收服务端数据失败!")
			return
		}

		fmt.Printf("%s", readData[:rLen])

	}

}
