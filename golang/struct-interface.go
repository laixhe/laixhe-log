package main

import "fmt"

// 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

// Golang并不是一个纯面向对象编程语言，提供了结构体（struct）类型，没有提供类（class）这个关键字，
// 也没有 public、protected、private 语法关键字

// 所有的 struct 的方法都是静态绑定，且没有方法重载、没有构造函数、没有析构函数
// 封装特性：首字母大写的就是公有的(public)，首字母小写的话就是私有的(private)，也就是说开头字母是大写的则在其它包中可以被访问，否则只能在本包中访问
// 继承特性：采用的是 匿名组合(composition)(嵌套) 的方式
// 多态特性：依靠 interface(接口) 隐式实现的方式

//

// BaseInterface 定义接口
// 某个 自定义类型 要 隐式 实现该接口全部方法后就实现这个接口
type BaseInterface interface {
	Show()
}

// Base 定义结构体
type Base struct {
	// 字段
	name string
	age  int
}

// Show 定义结构体 Base 的方法，同时也 隐式 实现了 BaseInterface 接口
func (b *Base) Show() {
	fmt.Printf("Show name=%v, age=%v\n", b.name, b.age)
}

// Derived 组合(继承) Base
type Derived struct {
	Base // 直接嵌入即可
}

// NewDerived 在 golang 中没有构造函数和析构函数，只能自己把握
func NewDerived(name string, age int) *Derived {
	// 实例化对象
	return &Derived{
		Base{
			name: name,
			age:  age,
		},
	}
}

type TestBase struct {
}

func (t *TestBase) Show() {
	fmt.Println("TestBase Show")
}

// Show 利用接口实现多态
func Show(b BaseInterface) {
	b.Show()
}
