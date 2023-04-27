package biz

import (
	"errors"

	"github.com/gin-gonic/gin"
	"golang_log/library/api-demo/app/models"
	"gorm.io/gorm"

	"golang_log/library/api-demo/api/gen/api"
	"golang_log/library/api-demo/api/gen/v1api"
	"golang_log/library/api-demo/app"
	"golang_log/library/api-demo/core/config"
	"golang_log/library/api-demo/core/errorx"
	"golang_log/library/api-demo/core/jwtx"
	"golang_log/library/api-demo/core/logx"
	"golang_log/library/api-demo/core/utils"
)

// AuthRegister 注册
func (b *Biz) AuthRegister(c *gin.Context, req *v1api.AuthRegisterRequest) (*v1api.AuthRegisterResponse, error) {
	u, err := b.data.User.FirstEmail(req.Email)
	if err != nil {
		if !errors.Is(err, gorm.ErrRecordNotFound) {
			logx.Errorf("FirstEmail %v", err)
			return nil, errorx.ServiceError(err)
		}
	}
	if u.Uid > 0 {
		return nil, errorx.NewError(api.ECode_EEmailExist, nil)
	}

	password, err := utils.BcryptPasswordHash(req.Password)
	if err != nil {
		logx.Errorf("FirstEmail %v", err)
		return nil, errorx.ServiceError(err)
	}
	user := &models.User{
		Password: password,
		Email:    req.Email,
		Name:     "",
		Age:      0,
		Score:    0,
	}
	err = b.data.User.Create(user)
	if err != nil {
		logx.Errorf("Create %v", err)
		return nil, errorx.ServiceError(err)
	}

	token, err := jwtx.GenToken(user.Uid, config.Get().Jwt)
	if err != nil {
		logx.Errorf("GenToken %v", err)
		return nil, errorx.ServiceError(err)
	}
	resp := &v1api.AuthRegisterResponse{
		Token: token,
		User: &v1api.User{
			Uid:   user.Uid,
			Email: user.Email,
		},
	}
	return resp, nil
}

// AuthLogin 登录
func (b *Biz) AuthLogin(c *gin.Context, req *v1api.AuthLoginRequest) (*v1api.AuthLoginResponse, error) {
	user, err := b.data.User.FirstEmail(req.Email)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, errorx.NewError(api.ECode_EEmailNotExist, err)
		}
		logx.Errorf("FirstEmail %v", err)
		return nil, errorx.ServiceError(err)
	}
	if !utils.BcryptPasswordCheck(req.Password, user.Password) {
		return nil, errorx.NewError(api.ECode_EPasswordError, err)
	}
	token, err := jwtx.GenToken(user.Uid, config.Get().Jwt)
	if err != nil {
		logx.Errorf("GenToken %v", err)
		return nil, errorx.ServiceError(err)
	}
	resp := &v1api.AuthLoginResponse{
		Token: token,
		User: &v1api.User{
			Uid:   user.Uid,
			Email: user.Email,
		},
	}
	return resp, nil
}

// AuthRefresh 刷新Jwt
func (b *Biz) AuthRefresh(c *gin.Context, req *v1api.AuthRefreshRequest) (*v1api.AuthRefreshResponse, error) {
	uid, errx := app.Uid(c)
	if errx != nil {
		return nil, errx
	}
	token, err := jwtx.GenToken(uid, config.Get().Jwt)
	if err != nil {
		logx.Errorf("GenToken %v", err)
		return nil, errorx.ServiceError(err)
	}
	resp := &v1api.AuthRefreshResponse{
		Token: token,
	}
	return resp, nil
}
