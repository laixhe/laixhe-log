package main

import (
	"fmt"
	"golang_log/library/anet-demo/network/pack"
	"log"
	"net"
	"time"
)

func main() {

	// 向服务端建立链接
	conn, err := net.Dial("tcp", "127.0.0.1:5050")
	if err != nil {
		log.Fatal("无法建立链接：", err)
	}
	defer conn.Close() //关闭链接

	log.Println("链接建立成功！")

	go func() {
		for {
			data, err := pack.TcpRead(conn)
			if err != nil {
				log.Println("接收服务端数据失败!")
				return
			}
			log.Println(data.DataLen, data.ID, string(data.Data))
		}
	}()

	go func() {
		for {
			// 向服务发送数据
			wData := "是的! " + fmt.Sprintf("%v", time.Now().UnixMilli())
			data, err := pack.Pack(pack.NewMessage(111, []byte(wData)))
			if err != nil {
				log.Println("Pack data 失败!")
				return
			}
			_, err = conn.Write(data)
			if err != nil {
				log.Println("向服务发送数据失败!")
				return
			}

			//log.Printf("向服务发送：%s (共 %d 字节)", wData, wLen)
			time.Sleep(time.Millisecond * 10)
		}
	}()

	select {}
}
