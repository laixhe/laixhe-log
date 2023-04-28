package osio

import (
	"io"
	"log"
	"os"
	"time"
)

// FileWriteRead 读写一个文件的简单例子
func FileWriteRead() {

	// 获取当前目录
	dir, err := os.Getwd()
	if err != nil {
		log.Println("获取当前目录失败：", err)
		return
	}

	var file *os.File
	defer file.Close()

	// 组合文件路径
	fileTxt := dir + "/FileWriteRead.txt"

	// 获取文件的信息
	fileInfo, err := os.Stat(fileTxt)
	if err != nil {

		if fileInfo == nil {

			// 文件不存在就创建
			file, err = os.Create(fileTxt)
			if err != nil {
				log.Println("创建文件失败：", err)
				return
			}

		} else {
			log.Println("获取文件的信息失败：", err)
			return
		}

	} else {

		// 文件存在就打开
		file, err = os.OpenFile(fileTxt, os.O_RDWR, 0666)
		if err != nil {
			log.Println("打开文件失败：", err)
			return
		}

	}

	WriteData := []byte("hello:" + time.Now().String())

	// 写入文件数据
	wlen, err := file.Write(WriteData)
	if err != nil {
		log.Println("写入文件失败：", err)
		return
	}

	log.Println("写入文件：", wlen, "字节")

	// 设置下次读写位置（移动文件指针位置）
	// 因为当前的文件指针位置在末尾位置
	// 移动到首位位置
	ret, err := file.Seek(0, 0)
	if err != nil {
		log.Println("移动文件指针位置失败:", err)
		return
	}
	log.Println("当前文件指针位置:", ret)

	ReadData := make([]byte, 512)
	// 读取文件数据
	rlen, err := file.Read(ReadData)
	if err != nil {

		if err == io.EOF {
			log.Println("写入文件失败，已经到文件结尾：", err)
			return
		}

		log.Println("读取文件数据失败：", err)
		return
	}

	log.Println("读取文件数据:", string(ReadData[:rlen]))
}
