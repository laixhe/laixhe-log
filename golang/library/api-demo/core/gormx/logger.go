package gormx

import (
	"fmt"

	"gorm.io/gorm/logger"

	"golang_log/library/api-demo/core/logx"
)

type writer struct {
	logger.Writer
}

// NewWriter writer 构造函数
func NewWriter(w logger.Writer) *writer {
	return &writer{Writer: w}
}

// Printf 格式化打印日志
func (w *writer) Printf(message string, data ...interface{}) {
	logx.Info(fmt.Sprintf(message, data...))
}
