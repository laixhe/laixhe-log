package main

import (
	"fmt"
	"net"
	"strings"
)

// 客户端
type Client struct {
	C    chan string // 用户发送数据的管道
	Name string      // 用户名
	Addr string      // 网络地址
}

// 保存在线用户
var onlineMap = make(map[string]*Client)

// 用于广播消息
var messaage = make(chan string)

// 用于转发消息，只要有消息来了，就遍历 onlineMap，给每个用户发消息
func Manager() {

	for {

		// 没有消息前，这里会阻塞
		msg := <-messaage

		// 给每个用户发消息
		for _, cli := range onlineMap {
			cli.C <- msg
		}

	}

}

// 专门给当前客户端发送信息
func WriteMsgToClient(cli *Client, conn net.Conn) {

	for msg := range cli.C {
		conn.Write([]byte(msg + "\n"))
	}

}

// 组合发送的字符串
func MakeMsg(cli *Client, msg string) string {
	return "[" + cli.Addr + "]" + cli.Name + ": " + msg
}

// 处理用户链接
func HandleConn(conn net.Conn) {
	defer conn.Close()

	// 获取客户端的网络地址
	cliAddr := conn.RemoteAddr().String()

	// 创建一个结构体，默认用户名和网络地址一样
	cli := &Client{
		make(chan string),
		cliAddr,
		cliAddr,
	}

	// 保存在线用户
	onlineMap[cliAddr] = cli

	// 新开一个协程，专门给当前客户端发送信息
	go WriteMsgToClient(cli, conn)

	// 广播在线
	messaage <- MakeMsg(cli, "login")

	// 用户是否退出
	isQuit := make(chan bool)

	// 新开一个协程，接收用户发送过来的数据
	go func() {

		buf := make([]byte, 1024)

		for {

			n, err := conn.Read(buf)
			if err != nil {

				// 用户退出信号
				isQuit <- true

				fmt.Println("conn.Read err = ", err)
				return
			}

			msg := string(buf[:n])

			if len(msg) == 4 && msg == "list" {
				// 遍历在线用户列表
				conn.Write([]byte("user list:"))
				for _, tmp := range onlineMap {
					msg = tmp.Addr + ": " + tmp.Name+"\n"
					conn.Write([]byte(msg))
				}

			} else if len(msg) >= 8 && msg[:7] == "rename|" {
				// 修改用户名
				name := strings.Split(msg, "|")[1]
				cli.Name = name
				conn.Write([]byte("rename ok"))

			} else {

				// 转发信息内容
				messaage <- MakeMsg(cli, msg)
			}

		}

	}()

	for {

		// 通过 select 检测 channel 的流动
		select {
		case <-isQuit:
			delete(onlineMap, cliAddr)            // 删除当前用户
			messaage <- MakeMsg(cli, "login out") // 广播下线
			return
		}

	}
}

func main() {

	// 设置监听
	listener, err := net.Listen("tcp", ":8000")
	if err != nil {
		fmt.Println("net.Listen err = ", err)
		return
	}
	defer listener.Close()

	// 用于转发消息，只要有消息来了，就遍历 onlineMap，给每个用户发消息
	go Manager()

	// 主协程，循环阻塞待用户链接
	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("listener.Accept err = ", err)
			continue
		}

		// 处理用户链接
		go HandleConn(conn)
	}

}
