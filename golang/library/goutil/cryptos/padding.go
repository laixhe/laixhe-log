package cryptos

import "bytes"

// CryptoPadding 填充方式
type CryptoPadding string

// 填充方式类型
const (
	PKCS5padding CryptoPadding = "PKCS5Padding"
	PKCS7padding CryptoPadding = "PKCS7Padding"
)

// PKCS7Padding 对加密文本进行分块填充
// 已对齐，填充一个长度为 blockSize 且每个字节均为 blockSize 的数据
// 未对齐，需要补充的字节个数为n，则填充一个长度为n且每个字节均为n的数据
func PKCS7Padding(data []byte, blockSize int) []byte {
	padding := blockSize - len(data)%blockSize
	padText := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(data, padText...)
}

// PKCS5Padding 是 PKCS7Padding 的子集，只是块大小固定为 8 字节
func PKCS5Padding(cipherText []byte) []byte {
	return PKCS7Padding(cipherText, 8)
}

// PKCSUnPadding 删除填充，对解密文本，解密时删除填充(padding)的文本
func PKCSUnPadding(data []byte) []byte {
	length := len(data)
	unPadding := int(data[length-1])
	return data[:(length - unPadding)]
}
