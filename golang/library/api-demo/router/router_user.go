package router

import (
	"github.com/gin-gonic/gin"

	"golang_log/library/api-demo/app/controllers"
)

// UserRouter 用户相关
func UserRouter(rf *gin.RouterGroup) {
	userGroup := rf.Group("/user")

	// not token

	userGroup.POST("/register", controllers.UserRegister) // 注册
	userGroup.POST("/login", controllers.UserLogin)       // 登陆

	// token

	//userGroupToken := userGroup.Use(JwtAuth())
}
