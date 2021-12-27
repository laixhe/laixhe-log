package rand_main

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
	int64 := result.Int64() + min

	return int64
}
