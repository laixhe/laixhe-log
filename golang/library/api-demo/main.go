package main

import (
	"golang_log/library/api-demo/core/config"
	"golang_log/library/api-demo/router"
)

func main() {
	if err := router.Router().Run(config.AppIpAddr()); err != nil {
		panic(err)
	}
}
