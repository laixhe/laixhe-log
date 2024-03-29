package main

import (
	"crypto/hmac"
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

// sha1 sha256 sha512 与 md5 用法一样 (hmac 也一样)

// 从计算效果看： md5.Sum(bdata) = hash.Write(data) + hash.Sum(nil)
// 123456 结果都一样：   e10adc3949ba59abbe56e057f20f883e

// Md5 加密
func Md5(encrypt string) string {
	// 等待要加密的数据
	data := []byte(encrypt)
	// 初始化 MD5 实例
	hash := md5.New()
	// 写入 MD5 的缓存，等待计算
	hash.Write(data)
	// 进行 MD5 算计，返回 16进制的 byte 数组
	sumData := hash.Sum(nil)
	// hex.EncodeToString(sumData)
	return fmt.Sprintf("%x\n", sumData)
}

// Md5Sun 加密
func Md5Sun(encrypt string) string {
	// 直接进行 MD5 算计，返回 16进制的 byte 数组
	sumData := md5.Sum([]byte(encrypt))
	// hex.EncodeToString(sumData)
	return fmt.Sprintf("%x\n", sumData)
}

// HmacMd5 带秘钥加密
func HmacMd5(key string, data string) string {
	mac := hmac.New(md5.New, []byte(key))
	mac.Write([]byte(data))
	return hex.EncodeToString(mac.Sum(nil))
}

// -----------------------------------------------

const Key = "123456789"
const Data = "123456"

func main() {
	fmt.Println(Md5(Data))          // e10adc3949ba59abbe56e057f20f883e
	fmt.Println(Md5Sun(Data))       // e10adc3949ba59abbe56e057f20f883e
	fmt.Println(HmacMd5(Key, Data)) // 2fa5a2a2d2e9d68a2cfb9821e2415464
}
