package reflect_type

import (
	"fmt"
	"reflect"
)

/**
反射 reflection

反射可大大提高程序的灵活性，使得 interface{} 有更大的发挥余地
反射使用 TypeOf 和 ValueOf 函数从接口中获取目标对象信息
反射会将匿名字段作为独立字段（匿名字段本质）
想要利用反射修改对象状态，前提是 interface.data 是 settable，

即 pointer-interface
- 通过反射可以“动态”调用方法
*/

// StructInfo 传入一个空接口，取出整个结构的信息
func StructInfo(o interface{}) {

	// 获得接收接口的类型(获取到一个类型信息)
	t := reflect.TypeOf(o)
	// 接口的名(类型名)
	fmt.Println("Type:", t.Name())

	// 获取类型 (int string struct ...)
	k := t.Kind()

	// 判断是否为结构类型
	if k != reflect.Struct {
		fmt.Println("传入的不是结构类型")
		return
	}

	// 包含所有的字段值信息(获取到一个类型的值信息)
	v := reflect.ValueOf(o)
	fmt.Println("Fields:")

	// 包含所有字段的数量(取所有字段的信息)
	for i := 0; i < t.NumField(); i++ {

		// 取出当前字段
		f := t.Field(i)
		// 取出当前字段的值
		val := v.Field(i).Interface()
		// 取出当前字段名、字段类型、和值
		fmt.Printf("%6s: %v = %v\n", f.Name, f.Type, val)
	}

	// 所有的方法
	for i := 0; i < t.NumMethod(); i++ {

		// 取出当前方法
		m := t.Method(i)
		// 取出当前方法名、方法类型
		fmt.Printf("%6s: %v\n", m.Name, m.Type)

	}

}

// StructSetVal 结构的反射操作-修改字段
func StructSetVal(o interface{}, key string, value string) {

	// 包含所有的字段值信息(获取到一个类型的值信息)
	v := reflect.ValueOf(o)
	// 是否是指针
	if v.Kind() != reflect.Ptr {
		fmt.Println("传入不是指针类型")
		return
	}

	// 是否能被修改
	if !v.Elem().CanSet() {
		fmt.Println("传入不能被修改")
		return
	}

	// 如果是指针，则获取其所指向的元素,代表的是这个指针所指向的值
	v = v.Elem()

	// 获取字段的名称
	f := v.FieldByName(key)
	if !f.IsValid() { // f.IsValid 方法判断一个 Value 类型是否有效地代表了一个值
		fmt.Println("找不到为Name的字段")
		return
	}

	// 修改为 Name 的字段
	if f.Kind() == reflect.String {

		// 设置了这个类型所代表的对象的值
		f.SetString(value)
	}
}

// StructSetFunc 结构的反射操作-调用方法
func StructSetFunc(o interface{}, funcName, value string) {

	// 包含所有的字段值信息(获取到一个类型的值信息)
	v := reflect.ValueOf(o)

	// 要调用的方法
	mv := v.MethodByName(funcName)

	// 动态地调用方法传递的参数必须是 reflect.Value 组成的一个切片
	// 可以通过 reflect.ValueOf 将任意值转换成 Value 类型
	args := []reflect.Value{reflect.ValueOf(value)}

	// mv.Call 函数执行了实际的方法调用
	// 它返回的是一个由 Value 类型变量组成的数组( []reflect.Value )，代表了所有的返回值
	returnVals := mv.Call(args)

	returnLen := len(returnVals)
	if returnLen > 0 {

		for i := 0; i < returnLen; i++ {
			fmt.Println("返回的第", i+1, "个参数: ", returnVals[i])
		}

	}

}

// StructSetManager 嵌入组合结构
func StructSetManager(o interface{}) {

	v := reflect.ValueOf(o)

	// v.FidleByIndex 传入的是一个 int 的 slice ，
	// 第一个数字表示在大的结构体中的索引，
	// 第二个数字表示在 嵌入组合 结构体中的索引

	// 打印 User 结构体的 ID 字段
	fmt.Printf("Type: %#v\n", v.FieldByIndex([]int{1, 0}))
	// 这里获取的是User结构体的 Name 字段
	fmt.Printf("Type: %#v\n", v.FieldByIndex([]int{1, 1}))

	// 是否是指针
	if v.Kind() != reflect.Ptr {
		fmt.Println("传入不是指针类型")
		return
	}

	// 是否能被修改
	if !v.Elem().CanSet() {
		fmt.Println("传入不能被修改")
		return
	}

}

// SetInt64 修改一个 int64 类型的值
func SetInt64(x *int64, value int64) {

	// 获取到一个类型的值信息
	v := reflect.ValueOf(x)

	// 是否是指针
	if v.Kind() != reflect.Ptr {
		fmt.Println("传入不是指针类型")
		return
	}

	// 如果是指针，则判断是否能被修改，并修改
	if v.Elem().CanSet() {
		v.Elem().SetInt(value)
	}
}
