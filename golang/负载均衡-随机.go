package main

import (
	"math/rand"
)

// 随机负载均衡：意味没有规律，随机在服务器队列中获得一台服务器处理请求

// LoadBalance 接口定义
type LoadBalance interface {
	Next(remove []string) *Server    // 选择一个后端Server，参数remove是需要排除选择的后端Server
	UpdateServers(servers []*Server) // 更新可用Server列表
}

// Server 后端定义
type Server struct {
	Host   string // 主机地址
	Name   string // 主机名
	Id     int    //
	Online bool   // 主机是否在线
}

type LoadBalanceRandom struct {
	servers []*Server
}

// NewLoadBalanceRandom 实例化 随机均衡负载
func NewLoadBalanceRandom(servers []*Server) *LoadBalanceRandom {
	newBalance := &LoadBalanceRandom{}
	newBalance.UpdateServers(servers)
	return newBalance
}

// Next 选择一个后端Server
func (r *LoadBalanceRandom) Next() *Server {
	if len(r.servers) == 0 {
		return nil
	}

	curIndex := rand.Intn(len(r.servers))
	return r.servers[curIndex]
}

func (r *LoadBalanceRandom) Get(key string) (*Server, error) {
	return r.Next(), nil
}

// UpdateServers 系统运行过程中，后端可用Server会更新
func (r *LoadBalanceRandom) UpdateServers(servers []*Server) {
	newServers := make([]*Server, 0)
	for _, e := range servers {
		if e.Online == true {
			newServers = append(newServers, e)
		}
	}
	r.servers = newServers
}
