package main

import (
	"fmt"
	"net"
)

func main() {

	addr := net.UDPAddr{IP: net.ParseIP("0.0.0.0"), Port: 5501}
	conn, err := net.ListenUDP("udp", &addr)
	if err != nil {
		fmt.Printf("listen failed, err:%v\n", err)
		return
	}
	defer conn.Close()

	fmt.Printf("ListenUDP: <%s> \n", conn.LocalAddr().String())

	// 循环读取消息内容
	data := make([]byte, 1024)
	for {
		n, connAddr, err := conn.ReadFromUDP(data)
		if err != nil {
			fmt.Printf("read failed from connAddr: %v, err:%v\n", connAddr, err)
			break
		}

		fmt.Printf("connAddr: %v data:%v count:%v\n", connAddr, string(data[:n]), n)

		// 回复数据
		_, err = conn.WriteToUDP([]byte("is server ListenUDP"), connAddr)
		if err != nil {
			fmt.Printf("write failed, err:%v\n", err)
			break
		}
	}

}
