package main

import "testing"

func TestStructInterface(t *testing.T) {

	derived := NewDerived("laixhe", 18)
	derived.Show() // 结果： Show name=laixhe, age=18

	// 实例化对象
	testBase := &TestBase{}
	testBase.Show() // 结果： TestBase Show

	Show(derived)  // 结果： Show name=laixhe, age=18
	Show(testBase) // 结果： TestBase Show

}
