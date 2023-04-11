package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// UserLogin 登陆
func UserLogin(c *gin.Context) {
	c.String(http.StatusOK, "登陆。。。")
}
