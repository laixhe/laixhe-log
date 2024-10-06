package main

import (
	"log"
	"net"
)

func main() {

	// 向服务端建立链接
	conn, err := net.Dial("tcp", "127.0.0.1:5501")
	if err != nil {
		log.Fatal("无法建立链接：", err)
	}
	defer conn.Close() //关闭链接

	log.Println("链接建立成功！")

	// 定义一个切片用于接收到的内容
	readData := make([]byte, 20)

	// 接收服务端发过来的数据
	rLen, err := conn.Read(readData)
	if err != nil {
		log.Println("接收服务端数据失败!")
		return
	}

	log.Printf("成功接收服务端数据：%s", readData[:rLen])

	// 向服务发送数据
	wData := "是的!"
	wLen, err := conn.Write([]byte(wData))
	if err != nil {
		log.Println("向服务发送数据失败!")
		return
	}

	log.Printf("向服务发送：%s (共 %d 字节)", wData, wLen)
}
