package main

import (
	"fmt"
	"testing"
)

/*
结构体嵌入（embedding）：Go 的"继承"替代方案（对应 Java/C# 继承 / Python 继承）

Go 没有 class 继承，用"组合"代替：
- 结构体里嵌入匿名字段（类型名不带字段名）
- 内嵌结构体的字段和方法被"提升"到外层（可像自己的字段一样访问）
- 内嵌类型实现接口 → 外层自动满足该接口
*/

// 基础结构体：动物
type Animal struct {
	Name string
}

func (a Animal) Speak() string {
	return a.Name + " 发出声音"
}

func (a Animal) Move() string {
	return a.Name + " 在移动"
}

// 嵌入：Husky 嵌入 Animal（对应继承 Animal，但没有 is-a 关系，是 has-a）
type Husky struct {
	Animal // 匿名字段：嵌入（字段名就是类型名）
	Breed  string
}

// 覆盖方法：Husky 重新定义 Speak（对应 Java @Override）
func (h Husky) Speak() string {
	return h.Name + " 汪汪叫"
}

// 多层嵌入：Puppy 嵌入 Husky → 间接获得 Animal 的字段与方法
type Puppy struct {
	Husky
}

// 嵌入接口：实现接口的嵌入
type Car struct {
	Engine // 嵌入接口（Car 必须实现 Engine 的方法）
	Model  string
}

type Engine interface {
	Start() string
	Stop() string
}

// Car 实现 Engine 接口的方法
func (c Car) Start() string { return c.Model + " 引擎启动" }
func (c Car) Stop() string  { return c.Model + " 引擎熄火" }

func TestEmbed(t *testing.T) {
	// 嵌入后：Animal 的字段被提升，可直接访问
	d := Husky{Animal: Animal{Name: "旺财"}, Breed: "金毛"}
	fmt.Println("提升的字段:", d.Name)   // 旺财（等价 d.Animal.Name）
	fmt.Println("提升的方法:", d.Move()) // 旺财 在移动（Animal.Move 被提升）

	// 覆盖：Husky 自己的 Speak 优先于 Animal.Speak
	fmt.Println("覆盖方法:", d.Speak())       // 旺财 汪汪叫
	fmt.Println("原方法:", d.Animal.Speak()) // 旺财 发出声音（显式访问内嵌）
}

func TestEmbedChain(t *testing.T) {
	// 多层嵌入：Puppy 直接访问最底层 Animal 的字段
	p := Puppy{Husky: Husky{Animal: Animal{Name: "小狗"}}}
	fmt.Println("多层提升字段:", p.Name) // 小狗

	// 赋值时也可只写内层
	p2 := Puppy{Husky: Husky{Animal: Animal{Name: "二狗"}, Breed: "拉布拉多"}}
	fmt.Println("Breed:", p2.Breed) // 拉布拉多
}

func TestEmbedInterface(t *testing.T) {
	// 嵌入接口：Car 结构体包含 Engine 接口
	c := Car{Model: "Tesla"}

	// 由于 Car 实现了 Engine 的方法，可赋值给接口变量
	var e Engine = c
	fmt.Println(e.Start()) // Tesla 引擎启动
	fmt.Println(e.Stop())  // Tesla 引擎熄火
}
