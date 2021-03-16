package main

import (
	"context"
	"fmt"
	"time"

	clientv3 "go.etcd.io/etcd/client/v3"
)

func main() {
	SetGet()
	//WatchGet()
}

// 写入读取
func SetGet() {

	// 配置 etcd ,创建客户端
	cli, err := clientv3.New(clientv3.Config{
		Endpoints:   []string{"127.0.0.1:2379"},
		DialTimeout: 5 * time.Second,
	})

	if err != nil {
		fmt.Println("connect failed, err:", err)
		return
	}
	defer cli.Close()

	// 存储
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	_, err = cli.Put(ctx, "/sample_key/name/", "sample_value_name")
	defer cancel()
	if err != nil {
		fmt.Println("put name failed, err:", err)
		return
	}

	// 存储
	ctx, cancel = context.WithTimeout(context.Background(), time.Second)
	_, err = cli.Put(ctx, "/sample_key/name/laixhe", "sample_value_name_laixhe")
	defer cancel()
	if err != nil {
		fmt.Println("put name failed, err:", err)
		return
	}

	// 存储 - 租约时间
	del, _ := cli.Grant(context.TODO(), 10) // 申请租约,10s后就会自动移除
	ctx, cancel = context.WithTimeout(context.Background(), time.Second)
	_, err = cli.Put(ctx, "/sample_key/name/del", "sample_value_name_del", clientv3.WithLease(del.ID))
	defer cancel()
	if err != nil {
		fmt.Println("put name failed, err:", err)
		return
	}

	// 获取
	ctx, cancel = context.WithTimeout(context.Background(), time.Second)
	resp, err := cli.Get(ctx, "/sample_key/name/")
	defer cancel()
	if err != nil {
		fmt.Println("get failed, err:", err)
		return
	}

	for _, ev := range resp.Kvs {
		fmt.Printf("get (/sample_key/name/) %s : %s\n", ev.Key, ev.Value)
	}

	fmt.Println("-----------------")

	// 获取前缀的 /sample_key/name/ 都返回
	ctx, cancel = context.WithTimeout(context.Background(), time.Second)
	resp, err = cli.Get(ctx, "/sample_key/name/", clientv3.WithPrefix())
	defer cancel()
	if err != nil {
		fmt.Println("get failed, err:", err)
		return
	}

	for _, ev := range resp.Kvs {
		fmt.Printf("get all (/sample_key/name/) - %s : %s\n", ev.Key, ev.Value)
	}

	fmt.Println("-----------------")

	// 续租约时间
	ch, err := cli.KeepAlive(context.TODO(), del.ID)
	if err != nil {
		fmt.Println("KeepAlive err:", err)
		return
	}

	for {
		ka := <-ch
		fmt.Println("ttl:", ka.TTL)
	}

	fmt.Println("-----------------")
}

// 监听某个KEY的变化
func WatchGet() {
	// 配置 etcd
	cli, err := clientv3.New(clientv3.Config{
		Endpoints:   []string{"127.0.0.1:2379"},
		DialTimeout: 5 * time.Second,
	})

	if err != nil {
		fmt.Println("connect failed, err:", err)
		return
	}
	defer cli.Close()

	rch := cli.Watch(context.Background(), "/sample_key/name/del")
	for {

		for wresp := range rch {
			for _, ev := range wresp.Events {
				fmt.Printf("%s %q : %q\n", ev.Type, ev.Kv.Key, ev.Kv.Value)
			}
		}
	}

}
