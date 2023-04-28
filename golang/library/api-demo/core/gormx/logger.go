package gormx

import (
	"fmt"

	"gorm.io/gorm/logger"

	"golang_log/library/api-demo/core/logx"
)

type Writer struct {
	logger.Writer
}

// NewWriter writer 构造函数
func NewWriter(w logger.Writer) *Writer {
	return &Writer{Writer: w}
}

// Printf 格式化打印日志
func (w *Writer) Printf(message string, data ...interface{}) {
	logx.Info(fmt.Sprintf(message, data...))
}
