package main

import (
	"bufio"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"os"
	"strings"
)

// os.O_CREATE 表示文件不存在就会创建新文件
// os.O_APPEND 表示以追加内容的形式添加(将数据附加到文件尾部)
// os.O_WRONLY 表示只写模式
// os.O_RDONLY 表示只读模式
// os.O_RDWR   表示读写模式
// os.O_EXCL   used with O_CREATE, file must not exist(和 O_CREATE 配合使用，文件必须不存在，否则返回一个错误)
// os.O_SYNC   I/O同步的方式打开(当进行一系列写操作时，每次都要等待上次的 I/O 操作完成再进行)
// os.O_TRUNC  表示文件存在，就将该文件的长度截为 0(如果可能，在打开时清空文件)

// 以读写方式打开文件，如果不存在，则创建
// str_file 文件路径
func OpanFile(f string) {

	// 以读写方式打开文件，如果不存在，则创建
	file, err := os.OpenFile(f, os.O_RDWR|os.O_CREATE, 0766)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(file)

	// 关闭文件
	file.Close()

}

// 打开文件,读取文件内容
func ReadFile(f string) {

	// 打开文件,读取文件内容,返回文件指针 ( os.Open 其实是调用 os.OpenFile )
	file, err := os.Open(f)
	if err != nil {
		fmt.Println(err)
		return
	}

	// 创建byte的slice用于接收文件读取数据（按字节读取）
	buf := make([]byte, 1024)
	// 循环读取
	for {

		// Read 函数会改变文件当前偏移量
		len, err := file.Read(buf)
		if err != nil {
			break
		}

		// 读取字节数为0时跳出循环
		if len == 0 {
			break
		}

		fmt.Println(string(buf))
	}

	// 关闭文件
	file.Close()

}

// 读取文件内容,指定的偏移量开始读取
func ReadAtFile(f string) {

	// 读取文件内容
	file, err := os.Open(f)
	if err != nil {
		fmt.Println(err)
		return
	}

	buf2 := make([]byte, 1024)
	ix := 0
	for {

		// ReadAt 从指定的偏移量开始读取，不会改变文件偏移量
		len, _ := file.ReadAt(buf2, int64(ix))

		ix = ix + len
		if len == 0 {
			break
		}

		fmt.Println(string(buf2))
	}

	// 关闭文件
	file.Close()

}

// 一次性读取文件全部数据 - 简单 ReadFile
func IoutilReadFile(f string) {

	// 一次性加载文件数据 ，其实底层是调用 ioutil.ReadAll 的
	data, err := ioutil.ReadFile(f)
	if err != nil {
		log.Println("加载文件数据:", err)
		return
	}

	log.Println(string(data))
}

// 一次性读取文件全部数据 - 简单 ReadAll
func IoutilReadAll(f string) {

	// 打开一个文件，为只读
	file, err := os.OpenFile(f, os.O_RDONLY, 0600) //O_RDONLY 为只读
	if err != nil {
		log.Println(err)
		return
	}
	defer file.Close()

	// 一次性加载文件数据
	data, err := ioutil.ReadAll(file)
	if err != nil {
		log.Println("加载文件数据:", err)
		return
	}

	log.Println(string(data))
}

// 读取文件,读入缓存
func BufioNewReader(f string) {

	// 打开一个文件，为只读
	file, err := os.OpenFile(f, os.O_RDONLY, 0600) //O_RDONLY 为只读
	if err != nil {
		log.Println(err)
		return
	}
	defer file.Close()

	// 读入缓存
	buff := bufio.NewReader(file)

	dada := make([]byte, 10)

	for {

		rlen, err := buff.Read(dada)
		if err != nil {

			// 文件已经到达结尾
			if err == io.EOF {
				break
			}

			log.Println("读取文件错误0")
			return
		}

		if rlen == 0 {
			log.Println("读取文件错误1")
			return
		}

		fmt.Print(string(dada[:rlen]))

	}
}

// 文件按行读取
func BufioReadStringLine(f string){

	// 打开一个文件，为只读
	file, err := os.OpenFile(f, os.O_RDONLY, 0600) //O_RDONLY 为只读
	if err != nil {
		log.Println(err)
		return
	}
	defer file.Close()

	// 读入缓存
	buff := bufio.NewReader(file)

	for {

		// 以'\n'为结束符读入一行
		line, err := buff.ReadString('\n')
		//buff.ReadLine()

		if err != nil || io.EOF == err {
			break
		}
		fmt.Println(line)

	}

}

// 文件按行分隔读取
func IoutilReadStringLine(f string){

	fileData, err := ioutil.ReadFile(f)
	if err != nil {
		fmt.Println(err)
		return
	}

	data := strings.Split(string(fileData), "\n")

	for _,v := range data{
		fmt.Println(v)
	}

}
