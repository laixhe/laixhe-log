package md5_main

import (
	"fmt"
	"testing"
)

const Key = "123456789"
const Data = "123456"

// e10adc3949ba59abbe56e057f20f883e
func TestMd5(t *testing.T) {
	fmt.Println(Md5(Data))
}

// e10adc3949ba59abbe56e057f20f883e
func TestMd5Sun(t *testing.T) {
	fmt.Println(Md5Sun(Data))
}

// 2fa5a2a2d2e9d68a2cfb9821e2415464
func TestHmacMd5(t *testing.T) {
	fmt.Println(HmacMd5(Key, Data))
}
