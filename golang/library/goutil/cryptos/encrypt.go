package cryptos

import (
	"crypto/hmac"
	"crypto/md5"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/hex"
)

// Md5 加密
func Md5(s string) string {
	// 进行 MD5 算计，返回 16 进制的 byte 数组
	b := md5.Sum([]byte(s))
	return hex.EncodeToString(b[:])
}

// Sha1 加密
func Sha1(s string) string {
	b := sha1.Sum([]byte(s))
	return hex.EncodeToString(b[:])
}

// Sha256 加密
func Sha256(s string) string {
	b := sha256.Sum256([]byte(s))
	return hex.EncodeToString(b[:])
}

// Sha512 加密
func Sha512(s string) string {
	b := sha512.Sum512([]byte(s))
	return hex.EncodeToString(b[:])
}

// HmacMd5 加密
func HmacMd5(k, s string) string {
	b := hmac.New(md5.New, []byte(k))
	_, _ = b.Write([]byte(s))
	return hex.EncodeToString(b.Sum(nil))
}

// HmacSha1 加密
func HmacSha1(k, s string) string {
	b := hmac.New(sha1.New, []byte(k))
	_, _ = b.Write([]byte(s))
	return hex.EncodeToString(b.Sum(nil))
}

// HmacSha256 加密
func HmacSha256(k, s string) string {
	b := hmac.New(sha256.New, []byte(k))
	_, _ = b.Write([]byte(s))
	return hex.EncodeToString(b.Sum(nil))
}

// HmacSha512 加密
func HmacSha512(k, s string) string {
	b := hmac.New(sha512.New, []byte(k))
	_, _ = b.Write([]byte(s))
	return hex.EncodeToString(b.Sum(nil))
}
