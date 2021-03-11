package main

import (
	"golang_log/library/api-demo/config"
	"golang_log/library/api-demo/router"
)

func main() {
	if err := router.Router().Run(config.AppAddr()); err != nil {
		panic(err)
	}
}
