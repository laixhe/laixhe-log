package router

import (
	"golang_log/library/api-demo/app/controllers"

	"github.com/gin-gonic/gin"
)

// UserRouter 用户相关
func UserRouter(rf *gin.RouterGroup) {
	r := rf.Group("/user")

	// not token

	r.POST("/regist", controllers.UserRegister) // 52.注册
	r.POST("/login", controllers.UserLogin)     // 53.登陆

	// token
	//ru := r.Use(JwtAuth())
}
