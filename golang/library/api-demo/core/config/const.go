package config

// AppMode 项目运行模式
type AppMode string

const (
	AppModeDebug   AppMode = "debug"
	AppModeRelease AppMode = "release"
)
