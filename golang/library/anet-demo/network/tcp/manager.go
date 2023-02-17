package tcp

import (
	"net"
	"sync"

	"golang_log/library/anet-demo/network"
)

// manager 服务器连接管理器
type manager struct {
	server network.IServer // 服务器
	pool   sync.Pool       // 连接池
	conns  *sync.Map       // 管理连接集合
}

func newManager(server *server) network.IManager {
	return &manager{
		server: server,
		pool:   sync.Pool{New: func() interface{} { return &Conn{} }},
		conns:  new(sync.Map),
	}
}

// HandleConn 处理(分配)连接
func (m *manager) HandleConn(conn net.Conn) {
	connPool := m.pool.Get().(*Conn)
	connPool.init(conn, m.server)
	m.Add(connPool)
}

// Add 添加链接
func (m *manager) Add(conn network.IConn) {
	m.conns.Store(conn.GetConnID(), conn)
}

// Remove 删除连接
func (m *manager) Remove(conn network.IConn) {
	m.conns.Delete(conn.GetConnID())
}

// GetConn 获取连接
func (m *manager) GetConn(connID int64) (network.IConn, bool) {
	if connAny, ok := m.conns.Load(connID); ok {
		if conn, is := connAny.(network.IConn); is {
			return conn, true
		}
	}
	return nil, false
}
