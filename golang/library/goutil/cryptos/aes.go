package cryptos

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"encoding/hex"
	"errors"
	"io"
)

// AES 对称加密算法
// 加密数据块分组长度(偏移量 vi)必须为 128bit(16bytes)，一般为 秘钥 的前 16 个字节
// 秘钥长度需要时 AES-128(16bytes)、AES-192(24bytes)、AES-256(32bytes)
//   AES     分组长度(字节)   密钥长度(字节)   加密轮数
// AES-128       16             16           10
// AES-192       16             24           12
// AES-256       16             32           14

// AesConfig 加密配置
type AesConfig struct {
	Key       string        // 秘钥为 AES-128(16bytes)、AES-192(24bytes)、AES-256(32bytes) 之一
	Iv        string        // 偏移量(固定16bytes)，为空时取 秘钥 的前 16 个字节
	Padding   CryptoPadding // 填充方式，默认为 PKCS7padding
	IsFixedIv bool          // 是否固定偏移量，当 CFB模式 有效
	key       []byte
	iv        []byte
}

// CheckKey 检查Key
func (e *AesConfig) CheckKey() error {
	if len(e.Key) == 16 || len(e.Key) == 24 || len(e.Key) == 32 {
		if len(e.key) == 0 {
			e.key = []byte(e.Key)
		}
		return nil
	}
	return errors.New("key is not one of AES-128(16bytes),AES-192(24bytes),AES-256(32bytes) error")
}

// CheckVi 检查Vi
func (e *AesConfig) CheckVi() error {
	if len(e.Iv) == 0 {
		e.Iv = e.Key[:16]
		e.iv = []byte(e.Iv)
		return nil
	}
	if len(e.Iv) != 16 {
		return errors.New("vi the length is not 16bytes error")
	}
	if len(e.iv) == 0 {
		e.iv = []byte(e.Iv)
	}
	return nil
}

// CheckPadding 检查Padding
func (e *AesConfig) CheckPadding() error {
	if e.Padding == "" {
		e.Padding = PKCS7padding
		return nil
	}
	switch e.Padding {
	case PKCS5padding:
	case PKCS7padding:
	default:
		return errors.New("padding error")
	}
	return nil
}

// Check 检查
func (e *AesConfig) Check() error {
	if err := e.CheckKey(); err != nil {
		return err
	}
	if err := e.CheckVi(); err != nil {
		return err
	}
	if err := e.CheckPadding(); err != nil {
		return err
	}
	return nil
}

// AesDecrypt 加解密后数据
type AesDecrypt struct {
	Data []byte // 数据
	Iv   []byte // 偏移量(固定16bytes)
}

// DataBase64 -
func (d *AesDecrypt) DataBase64() string {
	return base64.StdEncoding.EncodeToString(d.Data)
}

// DataHex -
func (d *AesDecrypt) DataHex() string {
	return hex.EncodeToString(d.Data)
}

// IvDataBase64 -
func (d *AesDecrypt) IvDataBase64() string {
	data := make([]byte, 0, len(d.Data)+len(d.Iv))
	data = append(data, d.Iv...)
	data = append(data, d.Data...)
	return base64.StdEncoding.EncodeToString(data)
}

// IvDataHex -
func (d *AesDecrypt) IvDataHex() string {
	data := make([]byte, 0, len(d.Data)+len(d.Iv))
	data = append(data, d.Iv...)
	data = append(data, d.Data...)
	return hex.EncodeToString(data)
}

// DataIvBase64 -
func (d *AesDecrypt) DataIvBase64() string {
	data := make([]byte, 0, len(d.Data)+len(d.Iv))
	data = append(data, d.Data...)
	data = append(data, d.Iv...)
	return base64.StdEncoding.EncodeToString(data)
}

// DataIvHex -
func (d *AesDecrypt) DataIvHex() string {
	data := make([]byte, 0, len(d.Data)+len(d.Iv))
	data = append(data, d.Data...)
	data = append(data, d.Iv...)
	return hex.EncodeToString(data)
}

// AesCBCEncrypt AES(CBC模式)加密算法
// data 未加密的原数据
func AesCBCEncrypt(data []byte, c *AesConfig) (*AesDecrypt, error) {
	if err := c.Check(); err != nil {
		return nil, err
	}
	// 分组秘钥
	block, err := aes.NewCipher(c.key)
	if err != nil {
		return nil, err
	}
	// 获取秘钥块的长度
	//blockSize := block.BlockSize()
	// 填充方式
	paddingData := make([]byte, 0)
	if c.Padding == PKCS5padding {
		paddingData = PKCS5Padding(data)
	}
	if c.Padding == PKCS7padding {
		paddingData = PKCS7Padding(data, block.BlockSize())
	}
	// 加密模式 CBC
	blockMode := cipher.NewCBCEncrypter(block, c.iv)
	// 加密后数据
	encryptData := make([]byte, len(paddingData))
	blockMode.CryptBlocks(encryptData, paddingData)
	return &AesDecrypt{
		Data: encryptData,
		Iv:   c.iv,
	}, nil
}

// AesCBCDecrypt AES(CBC模式)解密算法
// data 加密的数据
func AesCBCDecrypt(data []byte, c *AesConfig) (*AesDecrypt, error) {
	if err := c.Check(); err != nil {
		return nil, err
	}
	// 分组秘钥
	block, err := aes.NewCipher(c.key)
	if err != nil {
		return nil, err
	}
	// 获取秘钥块的长度
	//blockSize := block.BlockSize()
	// 加密模式 CBC
	blockMode := cipher.NewCBCDecrypter(block, c.iv)
	// 解密后数据
	decryptData := make([]byte, len(data))
	blockMode.CryptBlocks(decryptData, data)
	return &AesDecrypt{
		Data: PKCSUnPadding(decryptData), // 删除填充
		Iv:   c.iv,
	}, nil
}

// AesCFBEncrypt AES(CFB模式)加密算法
// data 未加密的原数据
func AesCFBEncrypt(data []byte, c *AesConfig) (*AesDecrypt, error) {
	if err := c.Check(); err != nil {
		return nil, err
	}
	// 分组秘钥
	block, err := aes.NewCipher(c.key)
	if err != nil {
		return nil, err
	}
	// 获取秘钥块的长度
	//blockSize := block.BlockSize()
	iv := make([]byte, 16)
	if c.IsFixedIv {
		iv = c.iv
	} else {
		_, err = io.ReadFull(rand.Reader, iv)
		if err != nil {
			return nil, err
		}
	}
	// 加密模式 CFB
	stream := cipher.NewCFBEncrypter(block, iv)
	// 加密后数据
	encryptData := make([]byte, len(data))
	stream.XORKeyStream(encryptData, data)
	return &AesDecrypt{
		Data: encryptData,
		Iv:   iv,
	}, nil
}

// AesCFBDecrypt AES(CFB模式)解密算法
// data 加密的数据
func AesCFBDecrypt(data []byte, c *AesConfig) (*AesDecrypt, error) {
	if err := c.Check(); err != nil {
		return nil, err
	}
	// 分组秘钥
	block, err := aes.NewCipher(c.key)
	if err != nil {
		return nil, err
	}
	// 获取秘钥块的长度
	//blockSize := block.BlockSize()
	// 加密模式 CFB
	stream := cipher.NewCFBDecrypter(block, c.iv)
	// 解密后数据
	decryptData := make([]byte, len(data))
	stream.XORKeyStream(decryptData, data)
	return &AesDecrypt{
		Data: decryptData,
		Iv:   c.iv,
	}, nil
}

// AecCTRCrypt AES(CTR模式)加解密算法
// data 数据(加解密)
func AecCTRCrypt(data []byte, c *AesConfig) (*AesDecrypt, error) {
	if err := c.Check(); err != nil {
		return nil, err
	}
	// 分组秘钥
	block, err := aes.NewCipher(c.key)
	if err != nil {
		return nil, err
	}
	// 获取秘钥块的长度
	//blockSize := block.BlockSize()
	// 指定计数器,长度必须等于 block 的块尺寸 c.Iv
	// 指定分组模式
	blockMode := cipher.NewCTR(block, c.iv)
	// 执行加密、解密操作
	cryptData := make([]byte, len(data))
	blockMode.XORKeyStream(cryptData, data)
	return &AesDecrypt{
		Data: cryptData,
		Iv:   c.iv,
	}, nil
}
