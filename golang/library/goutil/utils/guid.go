package utils

import (
	"crypto/rand"
	"encoding/binary"
	"encoding/hex"
	"math/big"
	"os"
	"sync"
	"sync/atomic"
	"time"

	"github.com/laixhe/goutil/date"
	"github.com/sony/sonyflake"
)

// 以原子方式递增-计数器
var guidCounter uint32 = 0

// GetGUID 获取 GUID
func GetGUID() string {
	var b [12]byte
	var i uint32

	binary.BigEndian.PutUint32(b[:], uint32(time.Now().Unix()))
	// 获取系统主机名
	host, err := os.Hostname()
	if err != nil || len(host) < 3 {
		// 以原子方式递增-计数器
		i = atomic.AddUint32(&guidCounter, 1)
		b[4] = byte(i >> 16)
		b[5] = byte(i >> 8)
		b[6] = byte(i)
	} else {
		b[4] = host[0]
		b[5] = host[1]
		b[6] = host[2]
	}
	// 获取进程PID
	pid := os.Getpid()
	b[7] = byte(pid >> 8)
	b[8] = byte(pid)
	// 以原子方式递增-计数器
	i = atomic.AddUint32(&guidCounter, 1)
	b[9] = byte(i >> 16)
	b[10] = byte(i >> 8)
	b[11] = byte(i)

	return hex.EncodeToString(b[:])
}

// 类以雪花算法的常量
const (
	saltBit       = uint(8)             // 随机因子二进制位数
	saltShift     = uint(8)             // 随机因子移位数
	increaseShift = saltBit + saltShift // 自增数移位数
)

// XUID 类以雪花算法
type XUID struct {
	sync.Mutex       // 互斥锁
	increase   int64 // 自增数
	saltA      int64 // 随机因子一
	saltB      int64 // 随机因子二
}

// Generate 类以雪花算法生成分布式唯一ID
func (c *XUID) Generate() int64 {
	c.Lock()
	c.increase++
	// 获取随机因子数值
	randA, err := rand.Int(rand.Reader, big.NewInt(255))
	if err != nil {
		return int64(GetSnowFlakeID())
	}
	c.saltA = randA.Int64()
	randB, err := rand.Int(rand.Reader, big.NewInt(255))
	if err != nil {
		return int64(GetSnowFlakeID())
	}
	c.saltB = randB.Int64()
	// 通过位运算实现自动占位
	id := int64((c.increase << increaseShift) | (c.saltA << saltShift) | c.saltB)
	c.Unlock()
	return id
}

// xuid 类以雪花算法
var xuid = XUID{increase: date.TimeUnix}

// GetXUID 类以雪花算法生成分布式唯一ID
func GetXUID() int64 {
	return xuid.Generate()
}

// snowFlake 雪花算法
var snowFlake = sonyflake.NewSonyflake(sonyflake.Settings{})

// GetSnowFlakeID 雪花算法生成分布式唯一ID
func GetSnowFlakeID() uint64 {
	id, err := snowFlake.NextID()
	if err != nil {
		return uint64(GetXUID())
	}
	return id
}
