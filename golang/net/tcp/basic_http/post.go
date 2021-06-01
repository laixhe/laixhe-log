package main

import (
	"bytes"
	"log"
	"net"
	"strconv"
)

// 用于模拟 POST 请求
func post(conn net.Conn, address string) {
	defer conn.Close() //关闭链接

	// POST 数据
	data := "name=laixhe&age=18"

	// Buffer是一个实现了读写方法的可变大小的字节缓冲
	// 用于请求行和请求头、post 数据
	str := bytes.NewBuffer(nil)
	str.WriteString("POST /test.php HTTP/1.1") // /test.php 替换成自已相应的测试地址
	str.WriteString("\r\n")
	str.WriteString("Host:" + address)
	str.WriteString("\r\n")
	str.WriteString("Connection:keep-alive") //保持长连接
	str.WriteString("\r\n")
	str.WriteString("Content-Type:application/x-www-form-urlencoded") //post表单
	str.WriteString("\r\n")
	str.WriteString("Content-Length:" + strconv.Itoa(len(data)))
	str.WriteString("\r\n")
	str.WriteString("\r\n")
	str.WriteString(data)

	// 向服务端发送数据
	wIen, err := conn.Write(str.Bytes())
	if err != nil {
		log.Println("写入失败：", err)
		return
	}

	log.Println("写入：", wIen, "字节")

	// 读取服务端响应的数据
	read := make([]byte, 10240)
	rLen, err := conn.Read(read)
	if err != nil {
		log.Println("读取失败：", err)
		return
	}

	log.Println("读取：", rLen, "字节")

	log.Println(string(read[:rLen]))

}
