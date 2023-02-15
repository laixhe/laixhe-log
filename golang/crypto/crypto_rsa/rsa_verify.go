package crypto_rsa

import (
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/pem"
)

// 在用于数字签名时，发送方通常先对消息生成散列值，再利用私钥对散列值进行签名，接收方收到消息及签名时，
// 也先对消息生成散列值（与发送方使用同种单向散列函数），利用发送方发的公钥、签名以及自己生成的散列值进行签名验证。

// GetRSAPrivateKey 读取 RSA 私钥
func GetRSAPrivateKey(publicKey []byte) *rsa.PrivateKey {

	// pem解码
	block, _ := pem.Decode(publicKey)
	// X509解码
	privateKey, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		panic(err)
	}

	return privateKey
}

// GetRSAPublicKey 读取 RSA 公钥
func GetRSAPublicKey(privateKey []byte) *rsa.PublicKey {

	// pem 解码
	block, _ := pem.Decode(privateKey)
	// x509 解码
	publicKeyInterface, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		panic(err)
	}

	publicKey := publicKeyInterface.(*rsa.PublicKey)
	return publicKey
}

// GetSign 对消息的散列值进行数字签名
func GetSign(msg []byte, publicKey []byte) []byte {

	// 取得私钥
	privateKey := GetRSAPrivateKey(publicKey)
	// 计算散列值
	hash := sha256.New()
	_, _ = hash.Write(msg)
	bytes := hash.Sum(nil)
	// SignPKCS1v15 使用 RSA PKCS#1 v1.5 规定的 RSASSA-PKCS1-V1_5-SIGN 签名方案计算签名
	sign, err := rsa.SignPKCS1v15(rand.Reader, privateKey, crypto.SHA256, bytes)
	if err != nil {
		panic(sign)
	}
	return sign
}

// VerifySign 验证数字签名
func VerifySign(msg []byte, sign []byte, privateKey []byte) bool {

	// 取得公钥
	publicKey := GetRSAPublicKey(privateKey)
	// 计算消息散列值
	hash := sha256.New()
	_, _ = hash.Write(msg)
	bytes := hash.Sum(nil)
	// 验证数字签名
	err := rsa.VerifyPKCS1v15(publicKey, crypto.SHA256, bytes, sign)
	return err == nil
}
