package main

import (
	"encoding/base64"
	"fmt"
)

// Encode 编码
func Encode(src string) string {
	return base64.StdEncoding.EncodeToString([]byte(src))
}

// Decode base解码
func Decode(src string) string {
	a, err := base64.StdEncoding.DecodeString(src)
	if err != nil {
		fmt.Println("decode error", err)
	}
	return string(a)
}

func main() {
	encodeBase64 := Encode("123456编码")
	decodeBase64 := Decode(encodeBase64)
	fmt.Println(encodeBase64)
	fmt.Println(decodeBase64)
}
