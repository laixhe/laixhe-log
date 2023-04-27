package network

import "net"

type IManager interface {
	HandleConn(conn net.Conn) // 处理(分配)连接
	Add(conn IConn)           // 添加链接
	Remove(conn IConn)        // 删除连接
}
