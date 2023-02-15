package crypto_rand

import (
	"crypto/rand"
	"math/big"
)

// RandInt64 生成区间随机数
func RandInt64(min, max int64) int64 {

	if min >= max || min == 0 || max == 0 {
		return max
	}

	result, _ := rand.Int(rand.Reader, big.NewInt(max-min))
	data := result.Int64() + min

	return data
}
