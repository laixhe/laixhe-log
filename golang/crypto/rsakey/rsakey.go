package main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"os"
)

// GenRsaKey 密钥文件生成
func GenRsaKey(bits int) error {

	// 生成私钥文件
	// GenerateKey 函数使用随机数据生成器 random 生成一对具有指定字位数的 RSA 密钥
	// Reader 是一个全局、共享的密码用强随机数生成器
	privateKey, err := rsa.GenerateKey(rand.Reader, bits)
	if err != nil {
		return err
	}

	// 保存私钥
	// 通过 x509 标准将得到的 ras 私钥序列化为 ASN.1 的 DER 编码字符串
	derStream := x509.MarshalPKCS1PrivateKey(privateKey)
	block := &pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: derStream,
	}

	// 使用 pem 格式对 x509 输出的内容进行编码
	// 创建文件保存私钥
	privFile, err := os.Create("private.pem")
	if err != nil {
		return err
	}
	defer privFile.Close()

	// 将数据保存到文件
	err = pem.Encode(privFile, block)
	if err != nil {
		return err
	}

	// 生成公钥文件
	// 获取公钥的数据
	publicKey := &privateKey.PublicKey
	// X509 对公钥编码
	derPkix, err := x509.MarshalPKIXPublicKey(publicKey)
	if err != nil {
		return err
	}

	block = &pem.Block{
		Type:  "PUBLIC KEY",
		Bytes: derPkix,
	}

	// pem 格式编码
	// 创建用于保存公钥的文件
	pubFile, err := os.Create("public.pem")
	if err != nil {
		return err
	}
	defer pubFile.Close()
	// 保存到文件
	err = pem.Encode(pubFile, block)
	if err != nil {
		return err
	}

	return nil
}
