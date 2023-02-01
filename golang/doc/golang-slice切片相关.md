### 切片的深浅拷贝
> 切片拷贝有三种方式
- 使用 `=` 操作符拷贝切片，这种就是浅拷贝
- 使用 `[:]` 下标的方式复制切片，这种也是浅拷贝
- 使用内置函数 `copy()` 进行切片拷贝，这种就是深拷贝

### 零切片、空切片、`nil`切片
> 零切片
- 使用 make 创建长度不为 `0` 的切片就是零值切片
```
slice := make([]int,5)  // 0 0 0 0 0
slice := make([]*int,5) // nil nil nil nil nil
```
> 空切片
- 空切片的长度和容量也都为 `0`
- 所有的空切片的数据指针都指向同一个地址(zerobase) `0xc42003bda0`
```
var slice = []int{}
var slice = make([]int, 0)
```
> `nil`切片
- 直接创建切片的方式
```
var slice []int
```

### 问题例子

```
# 例子1
func example_1()  {

	x := []int{1, 2, 3}
	y := x[:2] // 截取 x 的下标 0 1 不包括 2 的下标

	fmt.Println(x, len(x), cap(x)) // 结果: [1 2 3] len=3 cap=3
	fmt.Println(y, len(y), cap(y)) // 结果: [1 2]   len=2 cap=3
	
	// 此时 x y 是公用 底层的数组内存空间
	// y 的容量来至于 x 剩余的容量空间(从截取位置开始算起)，因为 y 是从 x 下标 0 开始截取的，所以 3-0=3 的容量
	// 如:
	// y := x[1:3] // 结果: [2 3]   len=2 cap=2

	y = append(y, 50) // 此时公用底层的数组，因为是公用的所以追加 y 也会 覆盖 x 下标为 2 的值
	y = append(y, 60) // 此时的 y 追加后，已经超出容量会重新分配空间，此时 x y 不在公用内存空间了

	fmt.Println(x, len(x), cap(x)) // 结果: [1 2 50]    len=3 cap=3
	fmt.Println(y, len(y), cap(y)) // 结果: [1 2 50 60] len=4 cap=6
}
```