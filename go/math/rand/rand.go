package main

import (
	"fmt"
	"math/rand"
	"time"
)

// 产生随机数主要有两个包
// math/rand   的实现了伪随机数生成器
// crypto/rand 的实现了用于加解密的更安全的随机数生成器

// math/rand：实现伪随机数生成器
// 获取随机数，不加随机种子，每次遍历获取都是重复的一些随机数据
// 虽然单次输出的结果是随机数，但是多次输出的结果却是相同的，这样很容易被找出规律，因此是伪随机数

// RandInt 简单的随机数生成，结合时间模块初始化种子
func RandInt() {

	// 以时间作为初始化种子
	rand.Seed(time.Now().UnixNano())

	// 随机一个 int 范围
	for i := 0; i < 10; i++ {
		a := rand.Int()
		fmt.Println("随机一个 int 范围:", a)
	}

	// 随机一个 0 ~ 100 之间的数
	for i := 0; i < 10; i++ {
		a := rand.Intn(100)
		fmt.Println("随机一个 0 ~ 100 之间的数:", a)
	}

	//  随机一个 float 范围
	for i := 0; i < 10; i++ {
		a := rand.Float32()
		fmt.Println("随机一个 float 范围:", a)
	}

	// 生成区间随机数，由于 for 过快，导致 time.Now().UnixNano() 重复过多
	// 可以使用 crypto/rand 的
	for i := 0; i < 10; i++ {
		time.Sleep(time.Microsecond)
		a := RandInt64(10, 15)
		fmt.Println("RandInt64:", a)
	}

}

// RandInt64 生成区间随机数
func RandInt64(min, max int64) int64 {

	if min >= max || min == 0 || max == 0 {
		return max
	}

	return rand.New(rand.NewSource(time.Now().UnixNano())).Int63n(max-min) + min
}

func main() {
	RandInt()
}
