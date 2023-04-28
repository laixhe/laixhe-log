package main

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"fmt"
	"io"
)

// AES 加密数据块分组长度必须为 128bit(16bytes)
// 密钥长度可以是 128bit(16bytes)、192bit(24bytes)、256bit(32bytes) 中的任意一个

// AesEncryptCBC 使用 AES 加密文本,加密的文本不能为空
func AesEncryptCBC(orig string, key string) string {

	// 转成字节数组
	origData := []byte(orig)
	keyData := []byte(key)

	// 分组秘钥
	block, _ := aes.NewCipher(keyData)
	// 获取秘钥块的长度
	blockSize := block.BlockSize()
	// 补全码
	origData = PKCS7Padding(origData, blockSize)
	// 加密模式
	blockMode := cipher.NewCBCEncrypter(block, keyData[:blockSize])
	// 创建数组
	crypted := make([]byte, len(origData))
	// 加密
	blockMode.CryptBlocks(crypted, origData)

	return base64.StdEncoding.EncodeToString(crypted)
}

// AesDecryptCBC 使用 AES 解密文本
func AesDecryptCBC(crypted string, key string) string {

	// 转成字节数组
	cryptedByte, _ := base64.StdEncoding.DecodeString(crypted)
	keyData := []byte(key)

	// 分组秘钥
	block, _ := aes.NewCipher(keyData)
	// 获取秘钥块的长度
	blockSize := block.BlockSize()
	// 加密模式
	blockMode := cipher.NewCBCDecrypter(block, keyData[:blockSize])
	// 创建数组
	origData := make([]byte, len(cryptedByte))
	// 解密
	blockMode.CryptBlocks(origData, cryptedByte)
	// 去补全码
	origData = PKCS7UnPadding(origData)
	return string(origData)
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

// AesCryptCTR AES加密算法 (CTR模式)
func AesCryptCTR(text []byte, key []byte) []byte {

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

// -----------------------------------------------

func AesEncryptECB(origData []byte, key []byte) (encrypted []byte) {
	cipher, _ := aes.NewCipher(generateKey(key))
	length := (len(origData) + aes.BlockSize) / aes.BlockSize
	plain := make([]byte, length*aes.BlockSize)
	copy(plain, origData)
	pad := byte(len(plain) - len(origData))
	for i := len(origData); i < len(plain); i++ {
		plain[i] = pad
	}
	encrypted = make([]byte, len(plain))
	// 分组分块加密
	for bs, be := 0, cipher.BlockSize(); bs <= len(origData); bs, be = bs+cipher.BlockSize(), be+cipher.BlockSize() {
		cipher.Encrypt(encrypted[bs:be], plain[bs:be])
	}

	return encrypted
}

func AesDecryptECB(encrypted []byte, key []byte) (decrypted []byte) {
	cipher, _ := aes.NewCipher(generateKey(key))
	decrypted = make([]byte, len(encrypted))
	//
	for bs, be := 0, cipher.BlockSize(); bs < len(encrypted); bs, be = bs+cipher.BlockSize(), be+cipher.BlockSize() {
		cipher.Decrypt(decrypted[bs:be], encrypted[bs:be])
	}

	trim := 0
	if len(decrypted) > 0 {
		trim = len(decrypted) - int(decrypted[len(decrypted)-1])
	}

	return decrypted[:trim]
}

func generateKey(key []byte) (genKey []byte) {
	genKey = make([]byte, 16)
	copy(genKey, key)
	for i := 16; i < len(key); {
		for j := 0; j < 16 && i < len(key); j, i = j+1, i+1 {
			genKey[j] ^= key[i]
		}
	}
	return genKey
}

// -----------------------------------------------

func AesEncryptCFB(origData []byte, key []byte) (encrypted []byte) {
	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}
	encrypted = make([]byte, aes.BlockSize+len(origData))
	iv := encrypted[:aes.BlockSize]
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		panic(err)
	}
	stream := cipher.NewCFBEncrypter(block, iv)
	stream.XORKeyStream(encrypted[aes.BlockSize:], origData)
	return encrypted
}

func AesDecryptCFB(encrypted []byte, key []byte) (decrypted []byte) {
	block, _ := aes.NewCipher(key)
	if len(encrypted) < aes.BlockSize {
		panic("ciphertext too short")
	}
	iv := encrypted[:aes.BlockSize]
	encrypted = encrypted[aes.BlockSize:]

	stream := cipher.NewCFBDecrypter(block, iv)
	stream.XORKeyStream(encrypted, encrypted)
	return encrypted
}

// -----------------------------------------------

// 密钥位数限定 16,24,32
// 秘钥长度需要时 AES-128(16bytes) 或者 AES-256(32bytes)

const KEY = "1234567812345623fqwefwefweq34fwe"

const OEIG = "hello world 对称加密算法,也就是加密和解密用相同的密钥"

func main() {
	encryptCBC := AesEncryptCBC(OEIG, KEY)
	fmt.Println("CBC 加密：", encryptCBC)

	decryptCBC := AesDecryptCBC(encryptCBC, KEY)
	fmt.Println("CBC 解密：", decryptCBC)

	encryptCTR := AesCryptCTR([]byte(OEIG), []byte(KEY))
	fmt.Println("CTR 加密：", base64.StdEncoding.EncodeToString(encryptCTR))

	decryptCTR := AesCryptCTR(encryptCTR, []byte(KEY))
	fmt.Println("CTR 解密：", string(decryptCTR))

	encryptECB := AesEncryptECB([]byte(OEIG), []byte(KEY))
	fmt.Println("ECB 加密：", base64.StdEncoding.EncodeToString(encryptECB))

	decryptECB := AesDecryptECB(encryptECB, []byte(KEY))
	fmt.Println("ECB 解密：", string(decryptECB))

	encryptCFB := AesEncryptCFB([]byte(OEIG), []byte(KEY))
	fmt.Println("CFB 加密：", base64.StdEncoding.EncodeToString(encryptCFB))

	decryptCFB := AesDecryptCFB(encryptCFB, []byte(KEY))
	fmt.Println("CFB 解密：", string(decryptCFB))
}
