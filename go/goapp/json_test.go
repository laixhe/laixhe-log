package main

import (
	"encoding/json"
	"fmt"
	"testing"
)

// 常用 Tag 参数说明
// omitempty 将字段值为空值 "" 0 nil false 时，忽略该字段，不参与序列化
// omitzero  将字段值为零值 "" 0 nil false 与包括自定义类型和嵌套结构体的零值时，忽略该字段，不参与序列化（Go 1.24 新特性）
// string    将数值类型 int、float、bool 转换为 JSON 字符串类型，反序列化时自动反向转换
// inline    将嵌套结构体字段时，将嵌套结构体的字段“扁平化”到父结构体 JSON 中
// -         将忽略该字段，不参与序列化与反序列化

// 使用 omitzero 的场景
// 处理时间类型：time.Time 的零值处理是 omitzero 最典型的应用场景
// 区分 nil 和空集合：需要保留空数组 [] 但忽略 nil 时使用
// 自定义零值判断：通过 IsZero() 方法实现业务特定的零值逻辑
//
// 使用 omitempty 的场景
// 常规字符串字段：空字符串需要被忽略的场景
// 指针类型：nil 指针需要被忽略的场景
// 简单的布尔和数字字段：false 和 0 需要被忽略的场景
//
// 需要精确控制零值用 omitzero，常规空值忽略用 omitempty

type Query struct {
	Path   string  `json:"path,omitempty"`
	Query  string  `json:"query,omitempty"`
	Age    int     `json:"age,string,omitempty"` // 可以是 {"age":"18"} 但不能是  {"age":18}
	Score  float64 `json:"score,string,omitempty"`
	IsPass bool    `json:"is_pass,string"`
}

func TestJson(t *testing.T) {
	query := Query{
		Path:  "/index/index",
		Query: "name=laixhe&age=18",
		Age:   18,
		Score: 88.8,
	}
	jsonBytes, err := json.Marshal(query)
	fmt.Println(string(jsonBytes), err)
	// 结果 {"path":"/index/index","query":"name=laixhe\u0026age=18","age":"18","score":"88.8","is_pass":"false"}

	query2 := Query{}
	jsonBytes2 := []byte(`{"path":"/index/index","query":"name=laixhe&age=19","age":"19","score":"99.99","is_pass":"true"}`)
	err = json.Unmarshal(jsonBytes2, &query2)
	fmt.Println(query2, err)
}
