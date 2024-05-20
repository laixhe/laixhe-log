package cryptos

import (
	"fmt"
	"testing"
)

const KEY32 = "1234567812345623fqwefwefweq34fwe"
const KEY24 = "1234567812345623fqwefwef"
const KEY16 = "1234567812345623"

const DATA = "hello world 对称加密算法"

// 16 21c40beaf6af12f3318335456d330f9736aaa3ed38fa1e3fdfb8b1d8f5ce9c2d
//    IcQL6vavEvMxgzVFbTMPlzaqo+04+h4/37ix2PXOnC0=
// 24 61e51c02341b52a588248cd8bd5b4c8202fe84fdcba6f46c092410247a942d0b
//    YeUcAjQbUqWIJIzYvVtMggL+hP3LpvRsCSQQJHqULQs=
// 32 d1bdffac962ca8380954634129911cd72d34616f3ab81ca8af428a1f06906307
//    0b3/rJYsqDgJVGNBKZEc1y00YW86uByor0KKHwaQYwc=

func TestAesCBCEncrypt(t *testing.T) {
	data, err := AesCBCEncrypt([]byte(DATA), &AesConfig{
		Key: KEY16,
	})
	if err != nil {
		fmt.Println("AesEncrypt Error:", err)
		return
	}
	fmt.Printf("%v\n", data.DataHex())
	fmt.Printf("%v\n", data.DataBase64())

	data, err = AesCBCDecrypt(data.Data, &AesConfig{
		Key: KEY16,
	})
	if err != nil {
		fmt.Println("AesDecrypt Error:", err)
		return
	}
	fmt.Printf("%s\n", data)
}

// 16 58ed6d61656943b3881e555f3179def1afa4c2fcb60c14cbdc2febc772a0
//    WO1tYWVpQ7OIHlVfMXne8a+kwvy2DBTL3C/rx3Kg+eU8MMiD1UCaL7
// 32 a6bbae6ddbea1e416053eba377ac87fd18af7a14ab15d4cb01833134dc96
//    pruubdvqHkFgU+ujd6yH/RivehSrFdTLAYMxNNyW
func TestAesCFBEncrypt(t *testing.T) {
	data, err := AesCFBEncrypt([]byte(DATA), &AesConfig{
		Key:       KEY32,
		IsFixedIv: true,
	})
	if err != nil {
		fmt.Println("AesCFBEncrypt Error:", err)
		return
	}
	fmt.Printf("data: %v\n", data.DataHex())
	fmt.Printf("data: %v\n", data.DataBase64())
	fmt.Printf("IvDataBase64: %v\n", data.IvDataBase64())
	fmt.Printf("IvDataHex: %v\n", data.IvDataHex())
	fmt.Printf("DataIvBase64: %v\n", data.DataIvBase64())
	fmt.Printf("DataIvHex: %v\n", data.DataIvHex())

	data, err = AesCFBDecrypt(data.Data, &AesConfig{
		Key: KEY32,
	})
	if err != nil {
		fmt.Println("AesCFBDecrypt Error:", err)
		return
	}
	fmt.Printf("%s [%v %v]\n", data, len(data.Data), len(DATA))
}

// 16 58ed6d61656943b3881e555f3179def1e26c00ef9e53c30c883d5409a2fb
//    WO1tYWVpQ7OIHlVfMXne8eJsAO+eU8MMiD1UCaL7
// 24 b9aad529903d16c9240effc21e08c244b9039abb7864878af4205a7d7e81
//    uarVKZA9FskkDv/CHgjCRLkDmrt4ZIeK9CBafX6B
// 32 a6bbae6ddbea1e416053eba377ac87fd15512067d6ad345d1ad68f5824ba
//    pruubdvqHkFgU+ujd6yH/RVRIGfWrTRdGtaPWCS6

func TestAecCtrCrypt(t *testing.T) {
	data, err := AecCTRCrypt([]byte(DATA), &AesConfig{
		Key: KEY24,
	})
	if err != nil {
		fmt.Println("AecCtrCrypt Error:", err)
		return
	}
	fmt.Printf("%v\n", data.DataHex())
	fmt.Printf("%v\n", data.DataBase64())

	data, err = AecCTRCrypt(data.Data, &AesConfig{
		Key: KEY24,
	})
	if err != nil {
		fmt.Println("AecCtrCrypt Error:", err)
		return
	}
	fmt.Printf("%s\n", data)
}
