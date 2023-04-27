package main

import (
	"flag"

	"golang_log/library/api-demo/core/config"
	"golang_log/library/api-demo/docs"
	"golang_log/library/api-demo/router"
)

var (
	// Version 指定版本号 ( go build -ldflags "-X main.Version=10000" )
	Version string
)

// flagConfigFile 指定配置文件
var flagConfigFile string

// @title	API接口
func main() {
	// init config
	flag.Parse()
	flag.StringVar(&flagConfigFile, "config", "./config/config.yaml", "config path eg: -config config/config.yaml")
	config.Init(flagConfigFile)
	// api doc
	docs.SwaggerInfo.Description = docs.ErrorDescription()
	if Version != "" {
		docs.SwaggerInfo.Version = Version
		config.Get().App.Version = Version
	}
	// init data db
	//gormx.Init(config.Get().Mysql)
	//redisx.Init(config.Get().Redis)
	// init http
	if err := router.Router().Run(config.Get().Servers.Http.Addr()); err != nil {
		panic(err)
	}
}
