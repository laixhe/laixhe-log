package router

import (
	"net/http"
	"runtime/debug"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"

	"golang_log/library/api-demo/app"
	"golang_log/library/api-demo/core/logx"
)

// gin中间件

// Cors 跨域
func Cors() gin.HandlerFunc {
	return cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"GET", "POST", "DELETE", "PUT", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Origin", "Content-Type", "Content-Length", "Authorization"},
		AllowCredentials: true,
		AllowOriginFunc: func(origin string) bool {
			return true
		},
		MaxAge: 12 * time.Hour,
	})
}

// GinLogger 日志
func GinLogger() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Next()
		logx.Info("gin",
			zap.String(app.XRequestID, c.GetHeader(app.XRequestID)),
			zap.Int("status", c.Writer.Status()),
			zap.String("method", c.Request.Method),
			zap.String("path", c.Request.URL.Path),
			zap.String("query", c.Request.URL.RawQuery),
			zap.String("ip", c.ClientIP()),
			zap.String("agent", c.Request.UserAgent()),
		)
	}
}

// GinRecovery 出现 panic 恢复正常
func GinRecovery() gin.HandlerFunc {
	return gin.CustomRecovery(func(c *gin.Context, err interface{}) {
		logx.Error("gin",
			zap.Int("status", c.Writer.Status()),
			zap.String("method", c.Request.Method),
			zap.String("path", c.Request.URL.Path),
			zap.String("query", c.Request.URL.RawQuery),
			zap.String("ip", c.ClientIP()),
			zap.String("agent", c.Request.UserAgent()),
			zap.Any("error", err),
			zap.String("stack", string(debug.Stack())),
		)
		c.AbortWithStatus(http.StatusInternalServerError)
	})
}
