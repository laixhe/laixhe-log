package main

import (
	"github.com/sirupsen/logrus"
	lumberjack "gopkg.in/natefinch/lumberjack.v2"
)

// logrus 内置了两种日志格式，JSONFormatter 和 TextFormatter

func main() {

	var log *logrus.Logger

	// 日志分割
	hook := lumberjack.Logger{
		Filename:   "./all.log", // 日志文件路径，默认 os.TempDir()
		MaxSize:    10,          // 每个日志文件保存10M，默认 100M
		MaxBackups: 30,          // 保留30个备份，默认不限
		MaxAge:     7,           // 保留7天，默认不限
		Compress:   true,        // 是否压缩，默认不压缩
	}

	// logrus 提供了 New() 函数来创建一个 logrus 的实例
	// 项目中，可以创建任意数量的 logrus 实例
	log = logrus.New()

	// 设置 logrus 实例的输出到任意 io.writer ，比如文件 *os.File
	log.Out = &hook

	// 为当前 logrus 实例设置消息输出格式为 json 格式。
	log.Formatter = &logrus.JSONFormatter{}

	// 设置日志级别为 Debug 以上
	log.SetLevel(logrus.DebugLevel)

	log.Info("T-----------I")

	log.WithFields(logrus.Fields{
		"test": "WithFields",
	}).Warn("T-----------W")

}

// 将日志输出到本地文件系统
// github.com/rifflock/lfshook
// github.com/natefinch/lumberjack

// 将日志发送到 Elasticsearch
// github.com/sohlich/elogrus

// 将日志发送到 AMQP
// github.com/vladoatanasov/logrus_amqp
