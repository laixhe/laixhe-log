package des_main

import (
	"fmt"
	"testing"
)

const OEIG = "Hello World! 对称加密算法,也就是加密和解密用相同的密钥"

// DES 声明秘钥,利用此秘钥实现明文的加密和密文的解密，长度必须为8
const DESkey = "123ABs78"

// DES 加解密
func TestDesEncrypt(t *testing.T) {

	// 加密
	encyptCode := DesEncrypt(OEIG, DESkey)
	fmt.Println("DES 密文：", encyptCode)

	// 解密
	decyptCode := DESDecrypt(encyptCode, DESkey)
	fmt.Println("DES 解密结果：", decyptCode)
}

// 3DES 的秘钥长度必须为24位
const DESkey3 = "123456781234567812345678"

// 3DES 加解密
func TestTripleDesEncrypt(t *testing.T) {

	// 加密
	encryptCode := TripleDesEncrypt(OEIG, DESkey3)
	fmt.Println("3DES 密文：", encryptCode)

	// 解密
	decryptCode := TipleDesDecrypt(encryptCode, DESkey3)
	fmt.Println("3DES 解密结果：", decryptCode)
}
