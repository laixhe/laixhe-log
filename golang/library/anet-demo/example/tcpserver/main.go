package main

import "golang_log/library/anet-demo/network/tcp"

func main() {
	s := tcp.NewServer()
	s.Start()
}
