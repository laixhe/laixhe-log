module mymodule

// 没有 全局变量，不能没有未初始化变量
// 使用 := 声明和初始化变量，默认不能修改变量，需要加 mut 关键字显式声明
// 这是在 V语言 中声明变量的 唯一 方法 

pub fn basic_type(){
	world := "World"
	println("Hello, " + world)
	println("Hello, $world")
	// 结果： Hello, World

	a := i64(3)
	b := "中文"
	c := true   // 推断为 bool 类型
	d := 1.1    // 推断为 f64 类型
	e := 2      // 推断为 int 类型

	println(typeof(a).name) // 输出 i64
	println(typeof(b).name) // 输出 string
	println(typeof(c).name)	// 输出 bool
	println(typeof(d).name)	// 输出 f64
	println(typeof(e).name)	// 输出 int
	println("a=${a} b=${b} c=${c} d=${d} e=${e}")
	// 结果： a=3 b=中文 c=true d=1.1 e=2
}