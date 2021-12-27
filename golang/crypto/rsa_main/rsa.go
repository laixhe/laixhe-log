package rsa_main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"errors"
)

// 这是一个非对称加密算法，一般通过公钥加密，私钥解密。
// 在加解密过程中，使用 openssl 生产密钥。执行如下操作：
// 创建私钥：
// openssl genrsa -out private.pem 1024 //密钥长度，1024 觉得不够安全的话可以用 2048，但是代价也相应增大
// 创建公钥：
// openssl rsa -in private.pem -pubout -out public.pem

// RsaEncrypt 加密
func RsaEncrypt(publicKey, origData []byte) ([]byte, error) {

	// pem 解码
	block, _ := pem.Decode(publicKey)
	if block == nil {
		return nil, errors.New("public key error")
	}
	// x509 解码
	pubInterface, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		return nil, err
	}

	// 类型断言
	pub := pubInterface.(*rsa.PublicKey)
	// 对明文进行加密
	return rsa.EncryptPKCS1v15(rand.Reader, pub, origData)

}

// RsaDecrypt 解密
func RsaDecrypt(privateKey, ciphertext []byte) ([]byte, error) {

	// pem 解码
	block, _ := pem.Decode(privateKey)
	if block == nil {
		return nil, errors.New("private key error")
	}

	// X509 解码
	priv, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		return nil, err
	}

	// 对密文进行解密
	return rsa.DecryptPKCS1v15(rand.Reader, priv, ciphertext)

}
