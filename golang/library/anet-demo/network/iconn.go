package network

import "context"

type IConn interface {
	Start()                   // 启动连接，让当前连接开始工作
	Stop()                    // 停止连接，结束当前连接状态M
	Context() context.Context // 返回该连接上下文
	GetConnID() int64         // 获取当前连接ID
}
