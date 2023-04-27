package logx

// LogRun 日志模式
type LogRun string

const (
	LogRunConsole LogRun = "console"
	LogRunFile    LogRun = "file"
)

// LogLevel 日志级别
type LogLevel string

const (
	LogLevelDebug LogLevel = "debug"
	LogLevelInfo  LogLevel = "info"
	LogLevelWarn  LogLevel = "warn"
	LogLevelError LogLevel = "error"
)

// 堆栈帧数
const callerSkip = 1
