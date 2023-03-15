package main

import (
	"fmt"

	"github.com/sony/sonyflake"
)

// 雪花算法

var snowFlake = sonyflake.NewSonyflake(sonyflake.Settings{})

func main() {
	for i:=0; i< 100; i++ {
		id, _ := snowFlake.NextID()
		fmt.Println(id)
	}
}
