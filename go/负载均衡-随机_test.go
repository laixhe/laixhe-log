package main

import (
	"fmt"
	"testing"
)

func TestLoadBalanceRandom(t *testing.T) {

	count := make([]int, 4)
	servers := make([]*Server, 0)
	servers = append(servers, &Server{Host: "1", Id: 0, Online: true})
	servers = append(servers, &Server{Host: "2", Id: 1, Online: true})
	servers = append(servers, &Server{Host: "3", Id: 2, Online: true})
	servers = append(servers, &Server{Host: "4", Id: 3, Online: true})
	lb := NewLoadBalanceRandom(servers)

	// 创建4个Server，随机选择100000次。查看4台机器 被选中次数
	for i := 0; i < 100000; i++ {
		c := lb.Next()
		count[c.Id]++
	}
	fmt.Println(count)

}
