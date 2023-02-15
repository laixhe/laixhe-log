package crypto_rsa

import (
	"encoding/base64"
	"fmt"
	"testing"
)

// 公钥和私钥可以从文件中读取

// 私钥
var PrivateKey = []byte(`
-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDZsfv1qscqYdy4vY+P4e3cAtmvppXQcRvrF1cB4drkv0haU24Y
7m5qYtT52Kr539RdbKKdLAM6s20lWy7+5C0DgacdwYWd/7PeCELyEipZJL07Vro7
Ate8Bfjya+wltGK9+XNUIHiumUKULW4KDx21+1NLAUeJ6PeW+DAkmJWF6QIDAQAB
AoGBAJlNxenTQj6OfCl9FMR2jlMJjtMrtQT9InQEE7m3m7bLHeC+MCJOhmNVBjaM
ZpthDORdxIZ6oCuOf6Z2+Dl35lntGFh5J7S34UP2BWzF1IyyQfySCNexGNHKT1G1
XKQtHmtc2gWWthEg+S6ciIyw2IGrrP2Rke81vYHExPrexf0hAkEA9Izb0MiYsMCB
/jemLJB0Lb3Y/B8xjGjQFFBQT7bmwBVjvZWZVpnMnXi9sWGdgUpxsCuAIROXjZ40
IRZ2C9EouwJBAOPjPvV8Sgw4vaseOqlJvSq/C/pIFx6RVznDGlc8bRg7SgTPpjHG
4G+M3mVgpCX1a/EU1mB+fhiJ2LAZ/pTtY6sCQGaW9NwIWu3DRIVGCSMm0mYh/3X9
DAcwLSJoctiODQ1Fq9rreDE5QfpJnaJdJfsIJNtX1F+L3YceeBXtW0Ynz2MCQBI8
9KP274Is5FkWkUFNKnuKUK4WKOuEXEO+LpR+vIhs7k6WQ8nGDd4/mujoJBr5mkrw
DPwqA3N5TMNDQVGv8gMCQQCaKGJgWYgvo3/milFfImbp+m7/Y3vCptarldXrYQWO
AQjxwc71ZGBFDITYvdgJM1MTqc8xQek1FXn1vfpy2c6O
-----END RSA PRIVATE KEY-----
`)

// 公钥
var PublicKey = []byte(`
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDZsfv1qscqYdy4vY+P4e3cAtmv
ppXQcRvrF1cB4drkv0haU24Y7m5qYtT52Kr539RdbKKdLAM6s20lWy7+5C0Dgacd
wYWd/7PeCELyEipZJL07Vro7Ate8Bfjya+wltGK9+XNUIHiumUKULW4KDx21+1NL
AUeJ6PeW+DAkmJWF6QIDAQAB
-----END PUBLIC KEY-----
`)

const ORIG = "hello world 这是一个非对称加密算法，一般通过公钥加密，私钥解密"

// 加密
func TestRsaEncrypt(t *testing.T) {

	data, err := RsaEncrypt(PublicKey, []byte(ORIG))
	if err != nil {
		panic(err)
	}
	dataBase64 := base64.StdEncoding.EncodeToString(data)
	fmt.Println("加密:", dataBase64)

}

// 解密
func TestRsaDecrypt(t *testing.T) {

	dataBase64 := "V7Txae0GCdr7EHW7L7Z+mUM+Y6sdKWrvBFxiSfENelCK2u/CpWGkSLfcUQw5NgQWQLQs2+u9v1fd04U8IohENb51pmAvEQfFfd2EQZUq/PVQEhITjUo2R/2+yYjwL/WXEb1Ep5uNgizV3GF3XnCcE/pDa6tZbn45WSpy7NGErj8="

	data, err := base64.StdEncoding.DecodeString(dataBase64)
	if err != nil {
		panic(err)
	}
	origData, err := RsaDecrypt(PrivateKey, data)
	if err != nil {
		panic(err)
	}
	fmt.Println("解密:", string(origData))

}

func TestVerifySign(t *testing.T) {

	//生成签名
	sign := GetSign([]byte(ORIG), PrivateKey)

	//验证签名
	result := VerifySign([]byte(ORIG), sign, PublicKey)
	fmt.Println("验证结果：", result)

}
