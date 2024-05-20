package main

import (
	"log"

	cron "github.com/robfig/cron/v3"
)

type TestJob struct {
}

func (this TestJob) Run() {
	log.Println("每隔2分钟执行一次...")
}

func main() {

	// 使用默认的配置
	c := cron.New()

	// 官方也提供了旧版本的秒级的定义
	// 这个注意你需要传入的 cron 表达式不再是标准 cron 表达式
	//c := cron.New(cron.WithSeconds())

	// 添加任务-传入函数
	// 每分钟执行一次
	i, err := c.AddFunc("* * * * *", Testfunc)
	if err != nil {
		log.Println(err)
	}
	log.Println("i=", i)

	// 每隔2分钟执行一次...
	i, err = c.AddJob("*/2 * * * *", TestJob{})
	if err != nil {
		log.Println(err)
	}
	log.Println("i=", i)

	// 启动计划任务
	c.Start()

	// 关闭着计划任务, 但是不能关闭已经在执行中的任务
	defer c.Stop()

	select {}
}

func Testfunc() {
	log.Println("每分钟执行一次...")
}
