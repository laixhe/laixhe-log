package app

import (
	"encoding/json"
	"net/http"

	"github.com/gin-gonic/gin"
	validator "github.com/go-playground/validator/v10"

	"golang_log/library/api-demo/api/gen/api"
	"golang_log/library/api-demo/core/errorx"
)

// ResponseModel 响应请求的公共模型
type ResponseModel struct {
	Code api.ECode `json:"code"` // 响应码
	Msg  string    `json:"msg"`  // 响应信息
	Data any       `json:"data"` // 数据
}

func Response(c *gin.Context, code api.ECode, data any, err string) {
	c.JSON(http.StatusOK, &ResponseModel{
		Code: code,
		Msg:  err,
		Data: data,
	})
}

func ResponseCode(c *gin.Context, code api.ECode, err string) {
	c.JSON(http.StatusOK, &ResponseModel{
		Code: code,
		Msg:  err,
	})
}

func ResponseOK(c *gin.Context, data any) {
	c.JSON(http.StatusOK, &ResponseModel{
		Code: api.ECode_EOK,
		Data: data,
	})
}

func ResponseError(c *gin.Context, err error) {
	if e, ok := err.(*errorx.Error); ok {
		c.JSON(http.StatusOK, &ResponseModel{
			Code: e.Code,
			Msg:  e.Error(),
		})
		return
	}
	if e, ok := err.(validator.ValidationErrors); ok {
		c.JSON(http.StatusOK, &ResponseModel{
			Code: api.ECode_EParam,
			Msg:  e.Error(),
		})
		return
	}
	if e, ok := err.(*json.UnmarshalTypeError); ok {
		c.JSON(http.StatusOK, &ResponseModel{
			Code: api.ECode_EParam,
			Msg:  e.Error(),
		})
		return
	}
	if e, ok := err.(*json.SyntaxError); ok {
		c.JSON(http.StatusOK, &ResponseModel{
			Code: api.ECode_EParam,
			Msg:  e.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, &ResponseModel{
		Code: api.ECode_EUnknown,
		Msg:  err.Error(),
	})
}
