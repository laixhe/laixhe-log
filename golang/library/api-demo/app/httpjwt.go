package app

import (
	"errors"
	"strings"

	"github.com/gin-gonic/gin"

	"golang_log/library/api-demo/api/gen/api"
	"golang_log/library/api-demo/core/config"
	"golang_log/library/api-demo/core/errorx"
	"golang_log/library/api-demo/core/jwtx"
	"golang_log/library/api-demo/core/logx"
)

// Authorization
const (
	Authorization = "Authorization"
	Bearer        = "Bearer "
	BearerLen     = 7
)

// JwtAuth 鉴权
func JwtAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		ecode := api.ECode_EAuthNotLogin
		token := ctx.Request.Header.Get(Authorization)
		if len(token) > 0 {
			if strings.HasPrefix(token, Bearer) {
				claims, err := jwtx.ParseToken(token[BearerLen:], config.Get().Jwt)
				if err == nil {
					ctx.Set(jwtx.AuthUid, claims.Uid)
					ctx.Next()
					return
				} else {
					if errors.Is(err, jwtx.ErrTokenExpired) {
						ecode = api.ECode_EAuthExpire
					}
					logx.Errorf("authorization:%v error:%v", token, err)
				}
			}
		}
		ResponseError(ctx, errorx.NewError(ecode, nil))
		// 返回错误
		ctx.Abort()
	}
}

func Uid(c *gin.Context) (uint64, *errorx.Error) {
	value, exists := c.Get(jwtx.AuthUid)
	if exists {
		return value.(uint64), nil
	}
	return 0, errorx.NewError(api.ECode_EAuthNotLogin, nil)
}
