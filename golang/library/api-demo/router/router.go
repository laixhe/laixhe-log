package router

import (
	"github.com/gin-gonic/gin"

	"golang_log/library/api-demo/core/config"
	"golang_log/library/api-demo/core/logx"
)

// Router gin 路由
func Router() *gin.Engine {

	if config.IsAppDebug() {
		gin.SetMode(gin.DebugMode)
	}

	r := gin.New()

	// 中间件
	r.Use(logx.GinLogger())
	r.Use(logx.GinRecovery())
	r.Use(Cors())

	apiV1 := r.Group("/v1")
	UserRouter(apiV1)

	return r
}
