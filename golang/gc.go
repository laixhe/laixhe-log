package main

import "runtime"

func GoBallast() {
	// Go Ballast 其实就是初始化一个生命周期贯穿整个 GO 应用生命周期的超大 slice
	// 利用这个特性，就能保证 GC 在多少内存时的一部时才能被触发，
	// 这样就能够比较精准控制 GC 的触发时机
	ballast := make([]byte, 1024*1024*500) // 其实这里申请的是虚拟内存，不是物理内存
	runtime.KeepAlive(ballast)
}
