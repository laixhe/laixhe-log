package main

import (
	"fmt"

	"github.com/go-playground/validator/v10"
)

// 参数校验库

type User struct {
	Id       int    `validate:"required"`          // 非空
	UserName string `validate:"min=6,max=10"`      // 字符串长度为[6,10]之间
	Age      uint8  `validate:"gte=1,lte=10"`      // 大于等于1 小于等于10
	Sex      string `validate:"oneof=female male"` // 类似枚举
	Email    string `validate:"email"`             // mail
	UName    string `validate:"checkName"`         // 自定义验证函数
}

func main() {

	// 创建验证器
	validate := validator.New()

	// 注册自定义函数，与 struct tag 关联起来
	err := validate.RegisterValidation("checkName", checkName)
	if err != nil {
		fmt.Println("RegisterValidation err:", err)
		return
	}

	user1 := User{
		Id:       1,
		UserName: "laixhe",
		Age:      10,
		Sex:      "female",
		Email:    "laixhe@laixhe.com",
		UName:    "laixhe",
	}

	// 验证各种结构对象的字段是否符合定义的约束
	err = validate.Struct(user1)
	if err != nil {
		fmt.Println("user1 err:", err)
		return
	}

	fmt.Println(user1)

}

// 自定义验证函数
func checkName(fl validator.FieldLevel) bool {

	uname := fl.Field().String()

	fmt.Printf("checkName uname:%v\n", uname)

	if uname == "laixhe" {
		return true
	}

	return false
}
