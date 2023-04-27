package tcp

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"net"
	"sync/atomic"
	"time"

	"golang_log/library/anet-demo/network"
	"golang_log/library/anet-demo/network/pack"
)

var (
	ErrorConnectionClosed = errors.New("connection is closed")
)

// Connection 用户链接
type Connection struct {
	connID     int64              // 连接ID，全局唯一
	conn       net.Conn           // TCP源连接
	connReader *bufio.Reader      // TCP源连接读缓冲
	server     network.IServer    // 服务器
	ctx        context.Context    // 管理该连接上下文
	cancel     context.CancelFunc // 管理该连接上下文
	writeChan  chan []byte        // 写入队列
	closeFlag  int32              // 当前连接是否关闭(原子锁)
}

func (c *Connection) init(conn net.Conn, server network.IServer) {
	c.conn = conn
	c.connReader = bufio.NewReader(conn)
	c.server = server
	c.closeFlag = 0
	c.writeChan = make(chan []byte, 100)

	// TODO: log
	fmt.Println("tcp accept init", conn.RemoteAddr(), c.connID)

	go c.Start()
}

// Start 启动连接，让当前连接开始工作
func (c *Connection) Start() {
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
func (c *Connection) Stop() {
	c.cancel()
}

// Context 返回该连接上下文
func (c *Connection) Context() context.Context {
	return c.ctx
}

func (c *Connection) GetConnID() int64 {
	return c.connID
}

// IsClosed 是否连接关闭
func (c *Connection) IsClosed() bool {
	return atomic.LoadInt32(&c.closeFlag) == 1
}

// close 关闭连接
func (c *Connection) close() {
	// TODO: log
	fmt.Println("tcp close", c.conn.RemoteAddr(), c.connID)

	// 如果当前链接已经关闭
	if c.IsClosed() {
		// TODO: log
		fmt.Println("tcp close true", c.conn.RemoteAddr(), c.connID)
		return
	}
	// 设置连接关闭标志位
	if atomic.CompareAndSwapInt32(&c.closeFlag, 0, 1) {
		// 关闭写入队列
		close(c.writeChan)
		// 关闭 socket 链接
		_ = c.conn.Close()
		// 将链接从连接管理器中删除
		c.server.GetManager().Remove(c)
	}
}

// read 读取消息
func (c *Connection) read() {
	defer c.Stop()

	for {
		select {
		case <-c.ctx.Done():
			// TODO: log
			fmt.Println("tcp read ctx done", c.conn.RemoteAddr(), c.connID)
			return
		default:
			packMessage, err := pack.TcpBufRead(c.connReader)
			if err != nil {
				// TODO: log
				fmt.Println("tcp read error", c.conn.RemoteAddr(), c.connID, err)
				return
			}

			c.writeChan <- []byte(string(packMessage.Data) + fmt.Sprintf("--%v", time.Now().UnixMilli()))
			fmt.Println(string(packMessage.Data))
		}
	}
}

// write 写入消息
func (c *Connection) write() {
	defer c.Stop()

	for {
		select {
		case <-c.ctx.Done():
			// TODO: log
			fmt.Println("tcp write ctx done", c.conn.RemoteAddr(), c.connID)
			return
		case data, ok := <-c.writeChan:
			if !ok {
				// TODO: log
				fmt.Println("tcp write chan", c.conn.RemoteAddr(), c.connID)
				return
			}
			if c.IsClosed() {
				// TODO: log
				fmt.Println("tcp write closed error", c.conn.RemoteAddr(), c.connID)
				return
			}
			packet, err := pack.Pack(pack.NewMessage(100, data))
			if err != nil {
				// TODO: log
				fmt.Println("tcp write packet message error", c.conn.RemoteAddr(), c.connID, err)
				continue
			}
			_, err = c.conn.Write(packet)
			if err != nil {
				// TODO: log
				fmt.Println("tcp write message error", c.conn.RemoteAddr(), c.connID, err)
				continue
			}
		}
	}
}
