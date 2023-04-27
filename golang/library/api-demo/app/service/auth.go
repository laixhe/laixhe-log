package service

import (
	"github.com/gin-gonic/gin"

	"golang_log/library/api-demo/api/gen/v1api"
	"golang_log/library/api-demo/app"
	"golang_log/library/api-demo/core/logx"
)

// AuthRegister 注册
//
// @Summary	注册用户
// @Accept   json
// @Produce  json
// @Param    body   body      v1api.AuthRegisterRequest   ture "请求body参数"
// @Success  200    {object}  v1api.AuthRegisterResponse
// @Router   /v1/auth/register [post]
func (s *Service) AuthRegister(c *gin.Context) {
	req := &v1api.AuthRegisterRequest{}
	if err := c.ShouldBindJSON(req); err != nil {
		app.ResponseError(c, err)
		return
	}
	logx.Infof("req:%s", req)

	resp, err := s.biz.AuthRegister(c, req)
	if err != nil {
		app.ResponseError(c, err)
		return
	}

	logx.Infof("resp:%s", resp)
	app.ResponseOK(c, resp)
}

// AuthLogin 登录
//
// @Summary	登录用户
// @Accept   json
// @Produce  json
// @Param    body   body      v1api.AuthLoginRequest   ture "请求body参数"
// @Success  200    {object}  v1api.AuthLoginResponse
// @Router   /v1/auth/login [post]
func (s *Service) AuthLogin(c *gin.Context) {
	req := &v1api.AuthLoginRequest{}
	if err := c.ShouldBindJSON(req); err != nil {
		app.ResponseError(c, err)
		return
	}
	logx.Infof("req:%s", req)

	resp, err := s.biz.AuthLogin(c, req)
	if err != nil {
		app.ResponseError(c, err)
		return
	}

	logx.Infof("resp:%s", resp)
	app.ResponseOK(c, resp)
}

// AuthRefresh 刷新Jwt
//
// @Summary	刷新Jwt
// @Accept   json
// @Produce  json
// @Param Authorization header string false "Bearer token令牌"
// @Success  200    {object}  v1api.AuthRefreshResponse
// @Success  400    {object}  app.ResponseModel
// @Router   /v1/auth/refresh [post]
func (s *Service) AuthRefresh(c *gin.Context) {
	req := &v1api.AuthRefreshRequest{}
	//if err := c.ShouldBindJSON(req); err != nil {
	//	app.ResponseError(c, err)
	//	return
	//}
	//logx.Infof("req:%s", req)

	resp, err := s.biz.AuthRefresh(c, req)
	if err != nil {
		app.ResponseError(c, err)
		return
	}

	logx.Infof("resp:%s", resp)
	app.ResponseOK(c, resp)
}
