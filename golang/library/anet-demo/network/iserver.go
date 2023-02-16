package network

type IServer interface {
	Start() error         // 启动服务器
	Stop() error          // 关闭服务器
	GetManager() IManager // 获取连接管理器
}
