package main

import "testing"

// 不带缓冲区
func TestChanInt(t *testing.T) {
	ChanInt()
}

// 不带缓冲区，但限定了发送
func TestChanIntSend(t *testing.T) {
	ChanIntSend()
}

// 不带缓冲区，主动关闭chan
func TestChanIntClose(t *testing.T) {
	ChanIntClose()
}

// 不带缓冲区，多个 chan 任务
func TestChanTask(t *testing.T) {
	ChanTask()
}

// 带缓冲区
func TestChanBuffer(t *testing.T) {
	ChanBuffer()
}

// 带缓冲区 for range
func TestChanBufferRange(t *testing.T) {
	ChanBufferRange()
}
