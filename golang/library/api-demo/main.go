package main

import (
	"fmt"
	"io/ioutil"
	"os"

	"golang_log/library/api-demo/config"
	"golang_log/library/api-demo/database/dbx"
	"golang_log/library/api-demo/database/redisx"
	"golang_log/library/api-demo/router"
)

func main() {
	server := router.Router()

	if config.Pid() != "" {
		pid := os.Getpid()
		err := ioutil.WriteFile(config.Pid(), []byte(fmt.Sprintf("%d", pid)), 0666)
		if err != nil {
			panic(err)
		}
	}

	err := dbx.Ping()
	if err != nil {
		panic(err)
	}
	err = redisx.Ping()
	if err != nil {
		panic(err)
	}

	err = server.Run(config.AppAddr())
	if err != nil {
		fmt.Println(err)
	}
}
