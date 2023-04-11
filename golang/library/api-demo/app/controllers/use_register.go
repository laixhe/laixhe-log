package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// UserRegister 注册
func UserRegister(c *gin.Context) {
	c.String(http.StatusOK, "注册。。。")
}
