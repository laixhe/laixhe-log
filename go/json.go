package main

import (
	"encoding/json"
	"fmt"

	"github.com/bytedance/sonic"
)

type JsonMain struct {
	ID       int64  `json:"id"`
	Name     string `json:"name"`
	TestData string `json:"test_data"`
}

// StdJson 标准库
func StdJson() {
	jsonMain1 := JsonMain{
		ID:       1,
		Name:     "laixhe",
		TestData: "Test Data",
	}

	// 序列化
	data, err := json.Marshal(jsonMain1)
	if err != nil {
		fmt.Println("Std Marshal", err)
		return
	}
	fmt.Printf("Std data=%s\n", string(data))

	// 反序列化
	var jsonMain2 = &JsonMain{} // 比须为以初始化的指针
	err = json.Unmarshal(data, jsonMain2)
	if err != nil {
		fmt.Println("Std Unmarshal", err)
		return
	}
	fmt.Printf("Std jsonMain2=%v\n", jsonMain2)
}

// SonicJson 字节跳动
func SonicJson() {
	jsonMain1 := JsonMain{
		ID:       2,
		Name:     "lai",
		TestData: "Test data",
	}

	// 序列化
	data, err := sonic.Marshal(jsonMain1)
	if err != nil {
		fmt.Println("Sonic Marshal", err)
		return
	}
	fmt.Printf("Sonic data=%s\n", string(data))

	// 反序列化
	var jsonMain2 = &JsonMain{} // 比须为以初始化的指针
	err = sonic.Unmarshal(data, jsonMain2)
	if err != nil {
		fmt.Println("Sonic Unmarshal", err)
		return
	}
	fmt.Printf("Sonic jsonMain2=%v\n", jsonMain2)
}
