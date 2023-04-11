package httpjwt

import (
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v4"
)

var ErrTokenInvalid = errors.New("token invalid")

const (
	AuthUid = "AuthUid" // CustomClaims.Uid
)

// Config jwt
type Config struct {
	SecretKey  string // jwt secret key
	ExpireTime int    // 过期时长(单位秒)
}

// CustomClaims 自定义声明类型 并内嵌 jwt.RegisteredClaims
// jwt 包自带的 jwt.RegisteredClaims 只包含了官方字段
type CustomClaims struct {
	// 可根据需要自行添加字段
	Uid int64 `json:"uid"`
	jwt.RegisteredClaims
}

// GenToken 生成JWT
func GenToken(uid int64, c *Config) (string, error) {
	claims := CustomClaims{
		Uid: uid,
	}
	if c.ExpireTime > 0 {
		claims.ExpiresAt = jwt.NewNumericDate(time.Now().Add(time.Duration(c.ExpireTime) * time.Second))
	}
	// 使用指定的签名方法创建签名对象
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	// 使用指定的 secret 签名并获得完整的编码后的字符串 token
	return token.SignedString(c.SecretKey)
}

// ParseToken 解析JWT
func ParseToken(tokenString string, c *Config) (*CustomClaims, error) {
	// 如果是自定义Claim结构体则需要使用 ParseWithClaims 方法
	token, err := jwt.ParseWithClaims(tokenString, &CustomClaims{}, func(token *jwt.Token) (i interface{}, err error) {
		return c.SecretKey, nil
	})
	if err != nil {
		return nil, err
	}
	// 对 token 对象中的 Claim 进行类型断言
	if claims, ok := token.Claims.(*CustomClaims); ok && token.Valid { // 校验 token
		return claims, nil
	}
	return nil, ErrTokenInvalid
}
