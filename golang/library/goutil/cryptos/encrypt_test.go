package cryptos

import (
	"fmt"
	"testing"
)

func TestMd5(t *testing.T) {
	// e10adc3949ba59abbe56e057f20f883e
	fmt.Println(Md5("123456"))
}

func TestSha1(t *testing.T) {
	// 7c4a8d09ca3762af61e59520943dc26494f8941b
	fmt.Println(Sha1("123456"))
}

func TestSha256(t *testing.T) {
	// 8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92
	fmt.Println(Sha256("123456"))
}

func TestSha512(t *testing.T) {
	// ba3253876aed6bc22d4a6ff53d8406c6ad864195ed144ab5c87621b6c233b548baeae6956df346ec8c17f5ea10f35ee3cbc514797ed7ddd3145464e2a0bab413
	fmt.Println(Sha512("123456"))
}

func TestHmacMd5(t *testing.T) {
	// 69632c9682f85442010afe6a54f23d8a
	fmt.Println(HmacMd5("1234", "123456"))
}

func TestHmacSha1(t *testing.T) {
	// cd7e6d8f355e54da7aab14500ce90e4451ead137
	fmt.Println(HmacSha1("1234", "123456"))
}

func TestHmacSha256(t *testing.T) {
	// 5d5eb6222b8436aaa9ed05598de5c2f72de4b7a9805fd746992a2b31c5754570
	fmt.Println(HmacSha256("1234", "123456"))
}

func TestHmacSha512(t *testing.T) {
	// 2c8ae33bf798dc8b8baa1c49f54d9c1e168f1b7c3e1b660bb189309eb79ee22a69ef2a85f2d5805562f52413df538b281d66a8bba33dd88c40b739e66b6b83df
	fmt.Println(HmacSha512("1234", "123456"))
}
