package main

import (
	"fmt"

	elasticsearch "github.com/elastic/go-elasticsearch/v8"
)

var client *elasticsearch.TypedClient

func init() {
	// ES 配置
	cfg := elasticsearch.Config{
		Addresses: []string{
			"http://localhost:9200",
		},
	}
	// 创建客户端连接
	var err error
	client, err = elasticsearch.NewTypedClient(cfg)
	if err != nil {
		fmt.Printf("elasticsearch.NewTypedClient failed, err:%v\n", err)
		return
	}
}
