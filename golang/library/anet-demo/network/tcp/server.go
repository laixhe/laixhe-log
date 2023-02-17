package tcp

import (
	"errors"
	"fmt"
	"net"

	"golang_log/library/anet-demo/network"
)

type server struct {
	listener *net.TCPListener // 监听器
	manager  network.IManager // 连接管理器
}

func NewServer() network.IServer {
	s := &server{}
	s.manager = newManager(s)
	return s
}

// init 初始化 TCP 服务器
func (s *server) init() error {
	// 获取一个 TCP 的 Addr
	addr, err := net.ResolveTCPAddr("tcp", ":5050")
	if err != nil {
		return err
	}

	// 监听服务器地址
	listener, err := net.ListenTCP(addr.Network(), addr)
	if err != nil {
		return err
	}

	s.listener = listener
	return nil
}

// accept 等待连接
func (s *server) accept() error {
	// 主协程，循环阻塞待用户链接
	for {
		// 阻塞等待客户端建立连接请求
		conn, err := s.listener.AcceptTCP()
		if err != nil {
			if errors.Is(err, net.ErrClosed) {
				// TODO: log
				fmt.Println("tcp listener accept closed error", err)
				return err
			}
			if e, ok := err.(net.Error); ok && e.Timeout() {
				// TODO: log
				fmt.Println("tcp listener accept timeout error", err)
				continue
			}

			// TODO: log
			fmt.Println("tcp listener accept error", err)
			continue
		}

		// 处理用户链接
		s.manager.HandleConn(conn)
	}
}

// Start 启动服务器
func (s *server) Start() error {
	if err := s.init(); err != nil {
		return err
	}
	return s.accept()
}

// Stop 关闭服务器
func (s *server) Stop() error {
	return nil
}

// GetManager 获取连接管理器
func (s *server) GetManager() network.IManager {
	return s.manager
}
