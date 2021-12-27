package router

import (
	"github.com/gin-gonic/gin"
	"github.com/laixhe/goutil/zaplog"

	"golang_log/library/api-demo/config"
)

// Router gin 路由
func Router() *gin.Engine {

	if config.IsAppDebug() {
		gin.SetMode(gin.DebugMode)
	}

	r := gin.New()

	// 中间件
	r.Use(zaplog.GinLogger())
	r.Use(zaplog.GinRecovery())
	r.Use(Cors())

	V1 := r.Group("/api/v1")
	UserRouter(V1)

	return r
}
