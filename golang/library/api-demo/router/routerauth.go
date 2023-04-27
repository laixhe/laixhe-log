package router

import (
	"github.com/gin-gonic/gin"

	"golang_log/library/api-demo/app/service"
)

// AuthRouter 鉴权相关
func AuthRouter(r *gin.RouterGroup, s *service.Service) {
	authGroup := r.Group("/auth")

	// not token

	authGroup.POST("/register", s.AuthRegister) // 注册
	//authGroup.POST("/login", s.UserLogin)       // 登陆

	// token

	//authGroupToken := authGroup.Use(JwtAuth())
}
