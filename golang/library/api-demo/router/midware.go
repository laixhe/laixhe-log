package router

import (
	"errors"
	"strings"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/laixhe/goutil/zaplog"

	"golang_log/library/api-demo/common"
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
	Bearer    = "Bearer "
	BearerLen = len(Bearer)
)

// JwtAuth 鉴权
func JwtAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		var errs error
		token := ctx.Request.Header.Get("Authorization")
		if len(token) > 0 {
			if strings.HasPrefix(token, Bearer) {
				t, err := common.JwtDecode(token[BearerLen:])
				if err == nil {
					ctx.Request.Header.Set(common.AuthUserIDKey, t.Id)
					ctx.Next()
					return
				}
				errs = err
			} else {
				errs = errors.New("token " + Bearer + "empty")
			}
		} else {
			errs = errors.New("token empty")
		}

		zaplog.Errorf("authorization:%v error:%v", token, errs)
		// 返回错误

		ctx.Abort()
	}
}
