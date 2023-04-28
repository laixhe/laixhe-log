package main

import (
	"log"
	"testing"
)

func TestGenRsaKey(t *testing.T) {

	// 密钥长度
	if err := GenRsaKey(1024); err != nil {
		log.Fatal("密钥文件生成失败！")
		return
	}

	log.Println("密钥文件生成成功！")
}
