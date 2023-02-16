package tcp

import (
	"net"
	"sync"

	"golang_log/library/anet-demo/network"
)

// manager 服务器连接管理器
type manager struct {
	server network.IServer         // 服务器
	pool   sync.Pool               // 连接池
	mx     sync.Mutex              // 管理连接锁
	conns  map[int64]network.IConn // 管理连接集合
}

func newManager(server *server) network.IManager {
	return &manager{
		server: server,
		pool:   sync.Pool{New: func() interface{} { return &Conn{} }},
		conns:  make(map[int64]network.IConn),
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
	m.mx.Lock()
	// 将新创建的 Conn 添加到链接管理中
	m.conns[conn.GetSessionID()] = conn
	m.mx.Unlock()
}

// Remove 删除连接
func (m *manager) Remove(conn network.IConn) {
	m.mx.Lock()
	m.mx.Unlock()

	// 删除连接
	delete(m.conns, conn.GetSessionID())
	m.pool.Put(conn.(*Conn))
}
