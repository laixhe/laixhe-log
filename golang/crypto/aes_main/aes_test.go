package aes_main

import (
	"encoding/base64"
	"fmt"
	"testing"
)

// 密钥位数限定 16,24,32
// 秘钥长度需要时 AES-128(16bytes) 或者 AES-256(32bytes)
const KEY = "1234567812345623fqwefwefweq34fwe"

const OEIG = "hello world 对称加密算法,也就是加密和解密用相同的密钥"

// AES加密算法 (CBC模式)
func TestAesDecrypt(t *testing.T) {

	encryptCode := AesEncrypt(OEIG, KEY)
	fmt.Println("CBC 加密：", encryptCode)

	decryptCode := AesDecrypt(encryptCode, KEY)
	fmt.Println("CBC 解密：", decryptCode)
}

// AES加密算法 (CTR模式)
func TestAecCtrCrypt(t *testing.T) {

	//加密
	cipherText := AecCtrCrypt([]byte(OEIG), []byte(KEY))
	fmt.Println("CTR 加密后为：", base64.StdEncoding.EncodeToString(cipherText))

	//解密
	plainText := AecCtrCrypt(cipherText, []byte(KEY))
	fmt.Println("CTR 解密后为：", string(plainText))
}
