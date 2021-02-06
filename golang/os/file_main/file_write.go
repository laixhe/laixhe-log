package main

import (
	"bufio"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"strconv"
)

// 创建文件,并写入文件
func WriteFile(f string) {

	// 写入文件
	file, err := os.Create(f)
	if err != nil {
		fmt.Println(err)
		return
	}

	data := "写入数据\r\n"
	for i := 0; i < 10; i++ {

		// 写入字节流
		file.Write([]byte(data))
		// 写入字符串
		file.WriteString(data)

	}

	// 关闭文件
	file.Close()

}

// 创建文件,并按指定偏移量写入数据
func WriteAtFile(f string) {

	// 写入文件
	file, err := os.Create(f)
	if err != nil {
		fmt.Println(err)
		return
	}

	for i := 0; i < 10; i++ {
		// 按指定偏移量写入数据
		ix := i * 64
		file.WriteAt([]byte("[偏移量"+strconv.Itoa(i)+"]\r\n"), int64(ix))
	}

	// 关闭文件
	file.Close()

}

// 一次性写入文件 - 是以覆盖形式
func IoutilWrite(f string) {

	data := "// BufioNewWriter\r\n//一次性写入文件\r\n"
	err := ioutil.WriteFile(f, []byte(data), 0776)
	if err != nil {
		fmt.Println(err)
	}

}

// 带缓冲写入文件
func BufioNewWriter(f string) {

	// 以读写方式打开文件，如果不存在，则创建，并以追加形式
	file, err := os.OpenFile(f, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0766)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer file.Close()

	// 设置下次读写位置（移动文件指针位置）
	// 因为当前的文件指针位置在首位位置
	// 移动到末尾位置
	//file.Seek(0, 2)

	// 缓存写入
	fileWrite := bufio.NewWriter(file)

	data := "// BufioNewWriter\r\n"

	// 待写入数据 - 现在还在缓存中
	fileWrite.WriteString(data)
	fileWrite.WriteString(data)
	fileWrite.WriteString(data)
	fileWrite.WriteString(data)

	// 将缓存中的数据写入文件
	// bufio 是写在缓冲区里面，一定要 Flush，否则文件中没有数据
	fileWrite.Flush()

}

// 使用 IO 写入文件(字符串)
func IoWrite(f string) {

	// 以读写方式打开文件，并以追加形式
	file, err := os.OpenFile(f, os.O_RDWR|os.O_APPEND, 0766)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer file.Close()

	data := "// BufioNewWriter\r\n"

	// 写入数据
	io.WriteString(file, data)
	io.WriteString(file, data)

}
