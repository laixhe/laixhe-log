package match

import "regexp"

var (
	emailExp  = regexp.MustCompile(`^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$`)
	mobileExp = regexp.MustCompile(`^1[0-9]{10}$`)
	numberExp = regexp.MustCompile(`^[0-9]+$`)
)

// IsEmail 是否为邮箱
func IsEmail(s string) bool {
	if s == "" {
		return false
	}
	return emailExp.MatchString(s)
}

// IsMobile 是否为手机号
func IsMobile(s string) bool {
	if s == "" {
		return false
	}
	return mobileExp.MatchString(s)
}

// IsNumber 是否为数整数
func IsNumber(s string) bool {
	if s == "" {
		return false
	}
	return numberExp.MatchString(s)
}
