package main

import (
	"bytes"
	"io"
	"log"
	"net"
	"os"
	"strconv"
)

// 用于模拟 文件上传 请求
func fileUpdate(conn net.Conn, address string) {
	defer conn.Close() //关闭链接

	// 打开一个文件 - 用于读取文件的数据
	fileStr := "./test.mp4" // 替换成自已相应的测试地址
	file, err := os.Open(fileStr)
	if err != nil {
		log.Println("打开文件失败：", err)
		return
	}
	defer file.Close()

	// 获取文件信息 - 主要用于获取文件大小
	fileInfo, err := file.Stat()
	if err != nil {
		log.Println("获取文件信息失败：", err)
		return
	}

	//==========================================================================

	//边界的分界符 - boundary - 最好用唯一的
	boundary := "AaBb010203Cc"

	// 用于模拟文件上传 的 开始
	dataStart := bytes.NewBuffer(nil)
	dataStart.WriteString("--" + boundary)
	dataStart.WriteString("\r\n")
	dataStart.WriteString("Content-Disposition: form-data; name=\"test\"; filename=\"test.mp4\"") //替换成自已相应的测试名称
	dataStart.WriteString("\r\n")
	dataStart.WriteString("Content-Type: application/octet-stream")
	dataStart.WriteString("\r\n")
	dataStart.WriteString("\r\n")

	// 用于模拟文件上传 的 结束
	dataEnd := bytes.NewBuffer(nil)
	dataEnd.WriteString("\r\n")
	dataEnd.WriteString("--" + boundary + "--")
	dataEnd.WriteString("\r\n")

	// 计算总数据大小
	dataLen := dataStart.Len() + dataEnd.Len() + int(fileInfo.Size())

	//==========================================================================

	// 用于请求行和请求头
	str := bytes.NewBuffer(nil)
	str.WriteString("POST /test.php HTTP/1.1") // test.php 替换成自已相应的测试地址
	str.WriteString("\r\n")
	str.WriteString("Host: " + address)
	str.WriteString("\r\n")
	str.WriteString("Connection: keep-alive") //保持长连接
	str.WriteString("\r\n")
	str.WriteString("Content-Type: multipart/form-data; boundary=" + boundary) // 上传文件
	str.WriteString("\r\n")
	str.WriteString("Content-Length: " + strconv.Itoa(dataLen))
	str.WriteString("\r\n")
	str.WriteString("\r\n")

	// 向服务端发送数据 - 用于请求行和请求头
	wIen, err := conn.Write(str.Bytes())
	if err != nil {
		log.Println("请求行和请求头写入失败：", err)
		return
	}
	log.Println("请求行和请求头写入：", wIen, "字节")

	// 向服务端发送数据 - 用于模拟文件上传 的 开始
	wIen, err = conn.Write(dataStart.Bytes())
	if err != nil {
		log.Println("开始模拟文件上传写入失败：", err)
		return
	}
	log.Println("开始模拟文件上传写入：", wIen, "字节")

	// 将本地文件流复制到网络流 - 上传文件中
	wIen64, err := io.Copy(conn, file)
	if err != nil {
		log.Println("上传文件中失败：", err)
	}
	log.Println("上传文件中：", wIen64, "字节")

	// 向服务端发送数据 - 用于模拟文件上传 的 结束
	wIen, err = conn.Write(dataEnd.Bytes())
	if err != nil {
		log.Println("结束模拟文件上传写入失败：", err)
		return
	}
	log.Println("结束模拟文件上传写入：", wIen, "字节")

	// 读取服务端响应的数据
	read := make([]byte, 10240)
	rLen, err := conn.Read(read)
	if err != nil {
		log.Println("读取服务端响应失败：", err)
		return
	}
	log.Println("读取服务端响应：", rLen, "字节")

	log.Println(string(read[:rLen]))

}
