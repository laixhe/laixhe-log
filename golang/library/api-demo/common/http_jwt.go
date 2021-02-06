package common

import (
	"errors"
	"time"

	"github.com/dgrijalva/jwt-go"

	"golang_log/library/api-demo/config"
)

// token error
var (
	ErrTokenCreate      = errors.New("token create error")
	ErrTokenExpired     = errors.New("token expired error")
	ErrTokenNotValidYet = errors.New("token not valid yet error")
	ErrTokenMalformed   = errors.New("token malformed error")
	ErrTokenInvalid     = errors.New("token invalid error")
)

// JwtCreate 创建
func JwtCreate(claims jwt.StandardClaims) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	signedToken, err := token.SignedString([]byte(config.Get().Jwt.SecretKey))
	if err != nil {
		return "", ErrTokenCreate
	}
	return signedToken, nil
}

// JwtDecode 解码
func JwtDecode(token string) (*jwt.StandardClaims, error) {
	t, err := jwt.ParseWithClaims(token, &jwt.StandardClaims{}, func(token *jwt.Token) (interface{}, error) {
		return []byte(config.Get().Jwt.SecretKey), nil
	})
	if err != nil {
		if ve, ok := err.(*jwt.ValidationError); ok {
			if ve.Errors&jwt.ValidationErrorMalformed != 0 {
				return nil, ErrTokenMalformed
			} else if ve.Errors&jwt.ValidationErrorExpired != 0 {
				return nil, ErrTokenExpired
			} else if ve.Errors&jwt.ValidationErrorNotValidYet != 0 {
				return nil, ErrTokenNotValidYet
			}
		}
		return nil, ErrTokenInvalid
	}

	claims, ok := t.Claims.(*jwt.StandardClaims)
	if ok && t.Valid {
		return claims, nil
	}

	return nil, ErrTokenInvalid
}

// JwtToken 获取
func JwtToken(id, mobile string) (string, error) {
	claims := jwt.StandardClaims{Id: id, Audience: mobile, ExpiresAt: int64(config.Get().Jwt.ExpTime) + time.Now().Unix()}
	return JwtCreate(claims)
}
