package tcp

import (
	"context"
	"fmt"
	"golang_log/library/anet-demo/network/pack"
	"net"
	"sync"
	"time"

	"golang_log/library/anet-demo/network"
)

// Conn 用户链接
type Conn struct {
	sessionID int64              // 连接ID，全局唯一
	conn      net.Conn           // TCP源连接
	server    network.IServer    // 服务器
	ctx       context.Context    // 管理该连接上下文
	cancel    context.CancelFunc // 管理该连接上下文
	isClosed  bool               // 当前连接是否关闭
	mx        sync.Mutex         // 管理连接锁
	writeChan chan []byte        // 写入队列
}

func (c *Conn) init(conn net.Conn, server network.IServer) {
	c.conn = conn
	c.server = server
	c.isClosed = false
	c.writeChan = make(chan []byte, 100)

	go c.Start()
}

// Start 启动连接，让当前连接开始工作
func (c *Conn) Start() {
	c.ctx, c.cancel = context.WithCancel(context.Background())
	go c.read()
	go c.write()

	select {
	case <-c.ctx.Done():
		c.close()
		return
	}
}

// Stop 停止连接，结束当前连接
func (c *Conn) Stop() {
	c.cancel()
}

// Context 返回该连接上下文
func (c *Conn) Context() context.Context {
	return c.ctx
}

func (c *Conn) GetSessionID() int64 {
	return c.sessionID
}

// close 关闭连接
func (c *Conn) close() {
	c.mx.Lock()
	defer c.mx.Unlock()

	// 如果当前链接已经关闭
	if c.isClosed {
		return
	}
	// 关闭 socket 链接
	_ = c.conn.Close()
	// 将链接从连接管理器中删除
	c.server.GetManager().Remove(c)
	// 设置连接关闭标志位
	c.isClosed = true
}

// read 读取消息
func (c *Conn) read() {
	defer c.Stop()

	for {
		select {
		case <-c.ctx.Done():
			return
		default:
			data, err := pack.TcpRead(c.conn)
			if err != nil {
				fmt.Println("read error ", err)
				return
			}

			c.writeChan <- []byte(string(data) + fmt.Sprintf("--%v", time.Now().UnixMilli()))
			fmt.Println(string(data))
		}
	}
}

// write 写入消息
func (c *Conn) write() {
	defer c.Stop()

	for {
		select {
		case <-c.ctx.Done():
			return
		case data, ok := <-c.writeChan:
			if !ok {
				return
			}
			packet, err := pack.Pack(data)
			if err != nil {
				fmt.Printf("packet message error: %v\n", err)
				continue
			}
			_, err = c.conn.Write(packet)
			if err != nil {
				fmt.Printf("write message error: %v\n", err)
				continue
			}
		}
	}
}
