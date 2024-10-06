package main

import (
	"log"
	"net"
)

// 模拟 POST 请求
// 用于模拟 文件上传 请求 - 有点绕
//

func main() {

	//替换成自已相应的测试地址
	address := "127.0.0.1:5501"

	//连接服务端
	conn, err := net.Dial("tcp", address)
	if err != nil {
		log.Println("无法建立链接：", err)
		return
	}

	// 模拟 POST 请求
	post(conn, address+"/post")

	// 用于模拟 文件上传 请求
	fileUpdate(conn, address+"/file")

}
