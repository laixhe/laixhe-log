package aes_main

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"encoding/base64"
	"fmt"
)

// AES 加密数据块分组长度必须为 128bit(16bytes)
// 密钥长度可以是 128bit(16bytes)、192bit(24bytes)、256bit(32bytes) 中的任意一个

// AesEncrypt 使用 AES 加密文本,加密的文本不能为空
func AesEncrypt(orig string, key string) string {

	// 转成字节数组
	origData := []byte(orig)
	k := []byte(key)

	// 分组秘钥
	block, _ := aes.NewCipher(k)
	// 获取秘钥块的长度
	blockSize := block.BlockSize()
	// 补全码
	origData = PKCS7Padding(origData, blockSize)
	// 加密模式
	blockMode := cipher.NewCBCEncrypter(block, k[:blockSize])
	// 创建数组
	cryted := make([]byte, len(origData))
	// 加密
	blockMode.CryptBlocks(cryted, origData)

	return base64.StdEncoding.EncodeToString(cryted)

}

// AesDecrypt 使用 AES 解密文本
func AesDecrypt(cryted string, key string) string {

	// 转成字节数组
	crytedByte, _ := base64.StdEncoding.DecodeString(cryted)
	k := []byte(key)

	// 分组秘钥
	block, _ := aes.NewCipher(k)
	// 获取秘钥块的长度
	blockSize := block.BlockSize()
	// 加密模式
	blockMode := cipher.NewCBCDecrypter(block, k[:blockSize])
	// 创建数组
	orig := make([]byte, len(crytedByte))
	// 解密
	blockMode.CryptBlocks(orig, crytedByte)
	// 去补全码
	orig = PKCS7UnPadding(orig)
	return string(orig)
}

// PKCS7Padding 使用 AES 加密文本的时候文本必须定长,即必须是 16,24,32 的整数倍
func PKCS7Padding(ciphertext []byte, blocksize int) []byte {

	fmt.Println("blocksize：", blocksize)

	padding := blocksize - len(ciphertext)%blocksize
	padtext := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(ciphertext, padtext...)
}

// PKCS7UnPadding 使用 AES 解密文本,解密收删除 padding 的文本
func PKCS7UnPadding(origData []byte) []byte {

	length := len(origData)
	unpadding := int(origData[length-1])
	return origData[:(length - unpadding)]
}

// -----------------------------------------------

// AecCtrCrypt AES加密算法 (CTR模式)
func AecCtrCrypt(text []byte, key []byte) []byte {

	// 指定加密、解密算法为 AES，返回一个 AES 的 Block 接口对象
	block, _ := aes.NewCipher(key)

	// 指定计数器,长度必须等于 block 的块尺寸
	// 分组长度：16 固定的
	count := []byte("12345678abcdefgh")

	// 指定分组模式
	blockMode := cipher.NewCTR(block, count)
	// 执行加密、解密操作
	message := make([]byte, len(text))
	blockMode.XORKeyStream(message, text)

	//返回明文或密文
	return message
}
