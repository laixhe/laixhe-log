package main

import (
	"bufio"
	"crypto/md5"
	"io"
	"io/ioutil"
	"log"
	"os"
)

// 计算一个文件的 MD5 值

// ReadFile 暴力读取文件
func ReadFile() {

	// 一次性加载文件数据，其实底层是调用 ioutil.ReadAll 的
	fData, err := ioutil.ReadFile("./md5.jpg")
	if err != nil {
		log.Println("Open File:", err)
		return
	}

	// 进行 MD5 算计，返回 16进制的 byte 数组
	fx := md5.Sum(fData)

	log.Printf("%x", fx)
}

// ReadAll 暴力读取文件，和上面 ReadFile 一样的性质
func ReadAll() {

	// 打开文件
	f, err := os.Open("./md5.jpg")
	if err != nil {
		log.Println("Open File:", err)
		return
	}
	defer f.Close()

	// 一次性加载文件数据
	fData, err := ioutil.ReadAll(f)
	if err != nil {
		log.Println("Open ReadAll:", err)
		return
	}

	fx := md5.Sum(fData)

	log.Printf("%x", fx)

}

// Read 基本使用 File 方法
func Read() {

	// 打开文件
	f, err := os.Open("./md5.jpg")
	if err != nil {
		log.Println("Open File:", err)
		return
	}
	defer f.Close()

	// 初始化 MD5 实例
	md5Hash := md5.New()

	buf := make([]byte, 32)

	for {
		nr, err := f.Read(buf)
		if err != nil {
			break
		}
		if nr > 0 {
			// 写入 MD5 的缓存，等待计算
			_, err = md5Hash.Write(buf[:nr])
			if err != nil {
				break
			}
		}
	}

	// 进行 MD5 算计，返回 16进制的 byte 数组
	fx := md5Hash.Sum(nil)

	log.Printf("%x", fx)
}

// IoCopy 使用 IO 复制
func IoCopy() {

	// 打开文件
	f, err := os.Open("./md5.jpg")
	if err != nil {
		log.Println("Open File:", err)
		return
	}
	defer f.Close()

	// 初始化 MD5 实例
	md5Hash := md5.New()

	_, err = io.Copy(md5Hash, f)
	if err != nil {
		log.Println("Io Copy:", err)
		return
	}

	// 进行 MD5 算计，返回 16进制的 byte 数组
	fx := md5Hash.Sum(nil)

	log.Printf("%x", fx)
}

// IoCopyBufio 使用缓存 IO 复制
func IoCopyBufio() {

	// 打开文件
	f, err := os.Open("./md5.jpg")
	if err != nil {
		log.Println("Open File:", err)
		return
	}
	defer f.Close()

	// 初始化 MD5 实例
	md5Hash := md5.New()

	// 读入缓存
	r := bufio.NewReader(f)

	_, err = io.Copy(md5Hash, r)
	if err != nil {
		log.Println("Io Copy:", err)
		return
	}

	// 进行 MD5 算计，返回 16进制的 byte 数组
	fx := md5Hash.Sum(nil)

	log.Printf("%x", fx)

}
