package main

import (
	"bytes"
	"crypto/cipher"
	"crypto/des"
	"encoding/base64"
	"fmt"
)

// 对称加密算法,也就是加密和解密用相同的密钥
// key、data、mode。key 为加密解密使用的密钥，data 为加密解密的数据，mode 为其工作模式
// Go的 DES 的默认隐藏了 ECB 模式, 因为go认为 ECB 不安全, 所以不建议使用,就隐藏了
// 默认采用 CBC 模式加解密 (加密分组链接模式)
// 只支持 8 字节密钥和 24 字节密钥

// DesEncryptCBC DES CBC 加密方法
func DesEncryptCBC(orig, key string) string {

	// 将加密内容和秘钥转成字节数组
	origData := []byte(orig)
	k := []byte(key)

	// 指定加密算法，创建并返回一个使用 DES 算法的 cipher.Block 接口对象-秘钥分组
	block, _ := des.NewCipher(k)

	// 将明文按秘钥的长度做补全操作
	origData = PKCS5Padding(origData, block.BlockSize())

	// 设置加密方式－CBC
	blockMode := cipher.NewCBCDecrypter(block, k)

	// 创建明文长度的字节数组
	crypted := make([]byte, len(origData))

	// 加密明文
	blockMode.CryptBlocks(crypted, origData)

	// 将字节数组转换成字符串，base64编码
	return base64.StdEncoding.EncodeToString(crypted)

}

// DESDecryptCBC DES CBC 解密方法
func DESDecryptCBC(data string, key string) string {

	k := []byte(key)

	// 将加密字符串用base64转换成字节数组
	crypted, _ := base64.StdEncoding.DecodeString(data)

	// 将字节秘钥转换成block快
	block, _ := des.NewCipher(k)

	// 设置解密方式－CBC
	blockMode := cipher.NewCBCEncrypter(block, k)

	//创建密文大小的数组变量
	origData := make([]byte, len(crypted))

	// 解密密文到数组origData中
	blockMode.CryptBlocks(origData, crypted)

	// 去掉加密时补全的部分
	origData = PKCS5UnPadding(origData)

	return string(origData)
}

// TripleDesEncrypt 3DES加密
func TripleDesEncrypt(orig, key string) string {
	// 转成字节数组
	origData := []byte(orig)
	k := []byte(key)

	// 3DES的秘钥长度必须为24位
	block, _ := des.NewTripleDESCipher(k)
	// 补全码
	origData = PKCS5Padding(origData, block.BlockSize())
	// 设置加密方式
	blockMode := cipher.NewCBCEncrypter(block, k[:8])
	// 创建密文数组
	crypted := make([]byte, len(origData))
	// 加密
	blockMode.CryptBlocks(crypted, origData)

	return base64.StdEncoding.EncodeToString(crypted)
}

// TipleDesDecrypt 3DES解密
func TipleDesDecrypt(crypted string, key string) string {
	// 用base64转成字节数组
	cryptedByte, _ := base64.StdEncoding.DecodeString(crypted)
	// key转成字节数组
	k := []byte(key)

	block, _ := des.NewTripleDESCipher(k)
	blockMode := cipher.NewCBCDecrypter(block, k[:8])
	origData := make([]byte, len(cryptedByte))
	blockMode.CryptBlocks(origData, cryptedByte)
	origData = PKCS5UnPadding(origData)

	return string(origData)
}

// PKCS5Padding 补码
// 实现明文的补全 - 只是块大小固定为 8 字节
// 如果 ciphertext 的长度为 blockSize 的整数倍，则不需要补全
// 否则差几个则被几个，例：差5个则补5个5 , 4个则补4个4
func PKCS5Padding(ciphertext []byte, blockSize int) []byte {

	fmt.Println("blockSize：", blockSize)

	padding := blockSize - len(ciphertext)%blockSize
	padtext := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(ciphertext, padtext...)
}

// PKCS5UnPadding 补码
// 实现去补码 PKCS5Padding 的反函数
func PKCS5UnPadding(origData []byte) []byte {
	length := len(origData)
	// 去掉最后一个字节 unpadding 次
	unpadding := int(origData[length-1])
	return origData[:(length - unpadding)]
}

// ZeroPadding   数据长度不对齐时使用 0 填充，否则不填充。
//func ZeroPadding(ciphertext []byte, blockSize int) []byte {
//	padding := blockSize - len(ciphertext)%blockSize
//	padtext := bytes.Repeat([]byte{0}, padding)
//	return append(ciphertext, padtext...)
//}
//
//func ZeroUnPadding(origData []byte) []byte {
//	length := len(origData)
//	unpadding := int(origData[length-1])
//	return origData[:(length - unpadding)]
//}
