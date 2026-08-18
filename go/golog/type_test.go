package main

import (
	"fmt"
	"testing"
)

/*
结构体 / 方法 / 接口（对应 TS classes.test.ts + interfaces.test.ts / Python classes.py）
*/

// 结构体（对应 class 的"数据部分" / Python dataclass / Java POJO）
type Person struct {
	Name string
	Age  int
}

// 方法：值接收者（不会修改原对象，对应 Java 普通方法）
func (p Person) Greet() string {
	return "你好，我是 " + p.Name
}

// 方法：指针接收者（可以修改原对象，对应 C++ 类方法里改 this）
func (p *Person) Birthday() {
	p.Age++ // 修改的是原对象
}

// 构造函数约定（Go 没有构造函数，用 NewXxx 函数约定）
func NewPerson(name string, age int) *Person {
	return &Person{Name: name, Age: age}
}

// 接口：定义一组方法（对应 Java interface / TS interface，Go 是隐式实现）
type Speaker interface {
	Speak() string
}

// Person 实现了 Speak 方法 → 自动满足 Speaker 接口（无需 implements 关键字）
func (p Person) Speak() string {
	return fmt.Sprintf("我叫 %s，今年 %d 岁", p.Name, p.Age)
}

// 鸭子类型：另一个与 Speaker 无关的类型也可实现
type Dog struct{ Name string }

func (d Dog) Speak() string {
	return d.Name + " 汪汪叫"
}

func TestStruct(t *testing.T) {
	// 零值初始化 / 字段赋值
	var p1 Person
	p1.Name = "laixhe"
	p1.Age = 18
	fmt.Println(p1, p1.Greet()) // {laixhe 18} 你好，我是 laixhe

	// 字面量初始化（按字段名 / 按顺序）
	p2 := Person{Name: "Alice", Age: 20}
	fmt.Println("字面量:", p2)

	// 指针接收者方法修改原对象
	p2.Birthday()
	fmt.Println("Birthday 后:", p2.Age) // 21

	// 构造函数
	p3 := NewPerson("Bob", 30)
	fmt.Println("NewPerson:", p3.Name, p3.Age)
}

func TestInterface(t *testing.T) {
	// 接口变量可以持有任何实现了方法的类型（对应 Java 多态）
	var s Speaker
	s = Person{Name: "laixhe", Age: 18}
	fmt.Println("Person:", s.Speak())

	s = Dog{Name: "旺财"}
	fmt.Println("Dog:", s.Speak()) // 运行时替换为 Dog 实现（多态）

	// 空接口 interface{} = 任意类型（对应 Java Object / C# object / TS unknown）
	var any any = 42 // any 是 interface{} 的别名（Go 1.18+）
	fmt.Println("空接口 int:", any)
	any = "字符串"
	fmt.Println("空接口 string:", any)

	// 类型断言：从接口取出具体值（对应 Java 强转 / C# as）
	if v, ok := s.(Dog); ok {
		fmt.Println("断言为 Dog:", v.Name)
	}
}

// 类型 switch：根据接口动态类型分派（对应 Java instanceof + 强转）
func describe(x any) string {
	switch v := x.(type) { // 类型 switch
	case int:
		return fmt.Sprintf("int: %d", v)
	case string:
		return fmt.Sprintf("string: %q", v)
	case Person:
		return fmt.Sprintf("Person: %s", v.Name)
	default:
		return fmt.Sprintf("未知类型 %T", v)
	}
}

func TestTypeSwitch(t *testing.T) {
	fmt.Println(describe(42))
	fmt.Println(describe("hello"))
	fmt.Println(describe(Person{Name: "laixhe"}))
	fmt.Println(describe(3.14))
}
