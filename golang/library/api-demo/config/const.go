package config

// 项目运行模式
const (
	ModeDebug   = "debug"
	ModeRelease = "release"
)

// redis 运行模式
const (
	RunTypeSingle   = 1 // 单机
	RunTypeSentinel = 2 // 哨兵主从
	RunTypeCluster  = 3 // 分布式集群
)

// log 日志
const (
	// 日志模式
	LogRunType = "console"

	// 日志级别
	LogDebug = "debug"
	LogInfo  = "info"
	LogError = "error"
)
