package zaplog

import (
	"os"
	"sync"
	"time"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	lumberjack "gopkg.in/natefinch/lumberjack.v2"
)

// 日志等级
const (
	DebugLevel = "debug"
	InfoLevel  = "info"
	WarnLevel  = "warn"
	ErrorLevel = "error"
)

// 堆栈帧数
const callerSkip = 1

var (
	once          sync.Once
	zapLogger     *zap.Logger
	sugaredLogger *zap.SugaredLogger
)

// Config 配置
type Config struct {
	ServiceName string // 服务名
	LogPath     string // 日志文件路径
	LogLevel    string // 日志级别：debug  info warn error
	CallerSkip  int    // 提升的堆栈帧数,默认 0，0=当前函数，1=上一层函数，....
	Filename    string // 日志文件路径，默认 os.TempDir()
	MaxSize     int    // 每个日志文件保存大小
	MaxBackups  int    // 保留N个备份，默认不限
	MaxAge      int    // 保留N天，默认不限
	Compress    bool   // 是否压缩，默认不压缩
}

// InitSizeFile 初始 zap 日志，按大小切割和备份个数、文件有效期
func InitSizeFile(c Config) {
	once.Do(func() {
		// 日志分割
		hook := lumberjack.Logger{
			Filename:   c.LogPath,
			MaxSize:    c.MaxSize,
			MaxBackups: c.MaxBackups,
			MaxAge:     c.MaxAge,
			Compress:   c.Compress,
		}
		// 打印到文件
		write := zapcore.AddSync(&hook)
		// 初始 zap 日志
		zapInit(write, c.ServiceName, c.LogLevel, c.CallerSkip+callerSkip)
	})
}

// InitConsole 初始 zap 日志，输出到终端
func InitConsole(c Config) {
	once.Do(func() {
		// 打印到控制台
		write := zapcore.AddSync(os.Stdout)
		// 初始 zap 日志
		zapInit(write, c.ServiceName, c.LogLevel, c.CallerSkip+callerSkip)
	})
}

// zapInit 初始化 zap 基本信息
// write       文件描述符
// serviceName 服务名
// logLevel    日志级别 debug  info  error
// callerSkip 提升的堆栈帧数，0=当前函数，1=上一层函数，....
func zapInit(write zapcore.WriteSyncer, serviceName, logLevel string, callerSkip int) {
	// 设置日志级别
	var level zapcore.Level
	switch logLevel {
	case DebugLevel:
		level = zap.DebugLevel
	case InfoLevel:
		level = zap.InfoLevel
	case WarnLevel:
		level = zap.WarnLevel
	case ErrorLevel:
		level = zap.ErrorLevel
	default:
		level = zap.DebugLevel
	}

	// 编码器配置
	encoderConfig := zapcore.EncoderConfig{
		TimeKey:        "ts",
		LevelKey:       "level",
		NameKey:        "logger",
		CallerKey:      "call",
		MessageKey:     "msg",
		StacktraceKey:  "stacktrace",
		LineEnding:     zapcore.DefaultLineEnding,
		EncodeLevel:    zapcore.LowercaseLevelEncoder,  // 小写编码器
		EncodeTime:     zapTimeEncoder,                 // 日志时间格式
		EncodeDuration: zapcore.SecondsDurationEncoder, // 执行消耗的时间转化成浮点型的秒
		EncodeCaller:   zapcore.ShortCallerEncoder,     // 短路径编码器,格式化调用堆栈
		EncodeName:     zapcore.FullNameEncoder,
	}

	core := zapcore.NewCore(
		zapcore.NewJSONEncoder(encoderConfig),
		write,
		level,
	)

	// 开启开发模式，堆栈跟踪
	caller := zap.AddCaller()
	// 提升打印的堆栈帧数
	addCallerSkip := zap.AddCallerSkip(callerSkip)
	// 开启文件及行号
	development := zap.Development()
	// 添加字段-服务器名称
	filed := zap.Fields(zap.String("serviceName", serviceName))
	// 构造日志
	zapLogger = zap.New(core, caller, addCallerSkip, development, filed)
	sugaredLogger = zapLogger.Sugar()
}

// zapTimeEncoder 日志时间格式
func zapTimeEncoder(t time.Time, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString(t.Format("2006-01-02 15:04:05.000"))
}

// IsInitNil 是否没有初始化
func IsInitNil() {
	if zapLogger == nil {
		InitConsole(Config{ServiceName: "goutil", LogLevel: DebugLevel, CallerSkip: callerSkip})
	}
}

// Debug 调试
func Debug(msg string, args ...zap.Field) {
	IsInitNil()
	zapLogger.Debug(msg, args...)
}

// Debugf 调试
func Debugf(template string, args ...interface{}) {
	IsInitNil()
	sugaredLogger.Debugf(template, args...)
}

// Info 信息
func Info(msg string, args ...zap.Field) {
	IsInitNil()
	zapLogger.Info(msg, args...)
}

// Infof 信息
func Infof(template string, args ...interface{}) {
	IsInitNil()
	sugaredLogger.Infof(template, args...)
}

// Warn 警告
func Warn(msg string, args ...zap.Field) {
	IsInitNil()
	zapLogger.Warn(msg, args...)
}

// Warnf 警告
func Warnf(template string, args ...interface{}) {
	IsInitNil()
	sugaredLogger.Warnf(template, args...)
}

// Error 错误
func Error(msg string, args ...zap.Field) {
	IsInitNil()
	zapLogger.Error(msg, args...)
}

// Errorf 错误
func Errorf(template string, args ...interface{}) {
	IsInitNil()
	sugaredLogger.Errorf(template, args...)
}
