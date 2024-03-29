package main

import (
	"fmt"

	"golang.org/x/crypto/bcrypt"
)

func BcryptPassword() {

	// 因为 bcrypt 加密使用了随机的盐，所以同一个字串每次 hash 的结果也是不一样的
	// 不接受长度超过 72 字节的密码
	// 同一字串的加密结果都是等价
	password := []byte("12345678")

	data, err := bcrypt.GenerateFromPassword(password, bcrypt.DefaultCost)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(data))

	data = []byte("$2a$10$kQQaW02kiB/1yWomfqM8x./3B0cWvwovhDMN.CKJKBf/RTdZc8aUy")

	err = bcrypt.CompareHashAndPassword(data, password)
	fmt.Println(err)

}
