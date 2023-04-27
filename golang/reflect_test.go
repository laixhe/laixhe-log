package main

import (
	"fmt"
	"testing"
)

type User struct {
	ID   int
	Name string
	Age  int
}

func (u User) Hello(str string) int {
	fmt.Println("Hello world.", str)
	return len(str)
}

type Manager struct {
	Title string
	User
}

func TestStructInfo(t *testing.T) {

	u := User{1, "ok", 2}

	// 传入一个空接口，取出整个结构的信息
	StructInfo(u)

	fmt.Println()

}

func TestStructSetVal(t *testing.T) {

	u := User{1, "ok", 2}

	// 结构的反射操作-修改字段
	StructSetVal(&u, "Name", "laiki")

	fmt.Println(u)
	fmt.Println()
}

func TestStructSetFunc(t *testing.T) {

	u := User{}

	// 结构的反射操作-调用方法
	StructSetFunc(&u, "Hello", "laiki")

	fmt.Println()

}

func TestStructSetManager(t *testing.T) {

	m := Manager{
		Title: "manager",
		User:  User{100, "lai", 12},
	}

	StructSetManager(m)

}

func TestSetInt64(t *testing.T) {

	var i int64 = 12

	// 修改一个 int64 类型的值
	SetInt64(&i, 1200)

	fmt.Println(i)
	fmt.Println()

}
