package main

import "testing"

func TestTimeRun(t *testing.T) {
	TimeRun()
}

// 自定义时区
func TestInitTime(t *testing.T) {
	InitTime()
}

// utc
func TestTimeUTC(t *testing.T) {
	TimeUTC()
}

// 时间字符串转换时间
func TestTimeParse(t *testing.T) {
	TimeParse()
}

// 定时器
func TestTimeTick1(t *testing.T) {
	TimeTick1()
}

// 定时器
func TestTimeTick2(t *testing.T) {
	TimeTick2()
}
