package main

import (
	"fmt"
	"net"
)

func main() {

	// 客户端
	// Port 为 0 为随机端口
	srcAddr := &net.UDPAddr{IP: net.IPv4zero, Port: 5502}

	// 连接服务端
	dstAddr := &net.UDPAddr{IP: net.ParseIP("127.0.0.1"), Port: 5501}
	conn, err := net.DialUDP("udp", srcAddr, dstAddr)
	if err != nil {
		fmt.Printf("connect failed, err:%v\n", err)
		return
	}
	defer conn.Close()

	fmt.Printf("DialUDP: <%s> \n", conn.LocalAddr().String())

	// 发送数据
	_, err = conn.Write([]byte("is DialUDP!"))
	if err != nil {
		fmt.Printf("write data failed, err:%v\n", err)
		return
	}

	// 接收数据
	data := make([]byte, 1024)
	n, addr, err := conn.ReadFromUDP(data)
	if err != nil {
		fmt.Printf("read data failed, err: %v\n", err)
		return
	}

	fmt.Printf("read <%s> %s\n", addr, data[:n])

}
