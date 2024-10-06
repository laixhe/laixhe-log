package main

import (
	"log"
	"net"
)

func main() {

	// 使用协议是 tcp ,监听本地所有 ip 的 5501 端口
	l, err := net.Listen("tcp", ":5501")
	if err != nil {
		log.Fatal("监听端口：", err)
	}
	//关闭监听的端口
	defer l.Close()

	log.Println("服务端已启动！")

	// 循环接受请求
	for {

		//***************** 1、接收请求连接 **********************

		// 接受请求连接
		conn, err := l.Accept() //用 conn 接收链接
		if err != nil {

			// err.(net.Error) 是类型断言,判断是否来自网络上的错误
			// ne.Temporary() 是否是网络临时错误
			if ne, ok := err.(net.Error); !ok || !ne.Temporary() {
				log.Printf("接受请求失败：%v", err)
			}
			continue //抛弃当前请求，继续监听

		}

		// 获取远程的 IP 地址(公网地址)
		ip := conn.RemoteAddr().String()
		log.Println("请求接受成功：", ip)

		//***************** 2、处理接收请求连接 **********************

		// 向客户端发送 信息
		wData := "你在学习 TCP ?"
		wLen, err := conn.Write([]byte(wData))
		if err != nil {
			log.Printf("向 %s 发送信息失败!", ip)
			return
		}

		log.Printf("向 %s 发送：%s (共 %d 字节)", ip, wData, wLen)

		// 定义一个切片用于接收到的内容
		readData := make([]byte, 20)
		// 接收服务端发过来的数据
		rLen, err := conn.Read(readData)
		if err != nil {
			log.Printf("接收 %s 数据失败!", ip)
			return
		}

		log.Printf("成功接收 %s 数据：%s (共 %d 字节)", ip, readData[:rLen], rLen)

		//与客户端断开连接
		conn.Close()
	}

}
