package ctx

import "testing"

// 通过 cancel 主动关闭
func TestCtxCancel(t *testing.T) {
	CtxCancel()

	// 收到关闭通知: context canceled
}

// 通过超时，自动触发
func TestCtxTimeout(t *testing.T) {
	CtxTimeout()

	// 收到关闭通知2: context deadline exceeded
	// 收到关闭通知1: context deadline exceeded
}

// 通过设置截止时间，触发超时
func TestCtxDeadline(t *testing.T) {
	CtxDeadline()

	// 收到关闭通知: context deadline exceeded
}

// 用 key/value 传递参数
func TestCtxValue(t *testing.T) {
	CtxValue()
}
