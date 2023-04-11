package router

import (
	"errors"
	"strings"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"

	"golang_log/library/api-demo/core/config"
	"golang_log/library/api-demo/core/httpjwt"
	"golang_log/library/api-demo/core/logx"
)

// Cors 跨域中间件
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

// Authorization
const (
	Authorization = "Authorization"
	Bearer        = "Bearer "
	BearerLen     = 7
)

// JwtAuth 鉴权
func JwtAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		var err error
		token := ctx.Request.Header.Get(Authorization)
		if len(token) > 0 {
			if strings.HasPrefix(token, Bearer) {
				if claims, err1 := httpjwt.ParseToken(token[BearerLen:], config.Get().Jwt); err1 == nil {
					ctx.Set(httpjwt.AuthUid, claims.Uid)
					ctx.Next()
					return
				} else {
					err = err1
				}
			} else {
				err = errors.New("token " + Bearer + "empty")
			}
		} else {
			err = errors.New("token empty")
		}
		logx.Errorf("authorization:%v error:%v", token, err)
		// 返回错误
		ctx.Abort()
	}
}
