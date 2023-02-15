package crypto_bcrypt

import (
	"fmt"
	"golang.org/x/crypto/bcrypt"
)

func BcryptPassword(){

	// 因为 bcrypt 加密使用了随机的盐，所以同一个字串每次 hash 的结果也是不一样的
	// 同一字串的加密结果都是等价
	password := []byte("123456")

	data, err := bcrypt.GenerateFromPassword(password, bcrypt.DefaultCost)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(data))

	err = bcrypt.CompareHashAndPassword(data, password)
	fmt.Println(err)

}