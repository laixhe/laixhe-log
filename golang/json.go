package main

import (
	"encoding/json"
	"fmt"
)

type JsonMain struct {
	ID   int64  `json:"id"`
	Name string `json:"name"`
}

func JsonMainRW() {
	jsonMain1 := JsonMain{
		ID:   1,
		Name: "laixhe",
	}

	// 序列化
	data, err := json.Marshal(jsonMain1)
	if err != nil {
		fmt.Println("Marshal", err)
		return
	}
	fmt.Printf("data=%s\n", string(data))

	// 反序列化
	var jsonMain2 = &JsonMain{} // 比须为以初始化的指针
	err = json.Unmarshal(data, jsonMain2)
	if err != nil {
		fmt.Println("Unmarshal", err)
		return
	}
	fmt.Printf("jsonMain2=%v\n", jsonMain2)
}
