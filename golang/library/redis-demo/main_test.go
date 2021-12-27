package main

import (
	"fmt"
	"testing"
	"time"
)

func TestRedisClient_SetGetDel(t *testing.T) {
	err := NewClient().SetGetDel("age", 18, time.Second*2)
	fmt.Printf("err:%v", err)
}

func TestRedisClient_IncrDecr(t *testing.T) {
	err := NewClient().IncrDecr("add")
	fmt.Printf("err:%v", err)
}

func TestRedisClient_List(t *testing.T) {
	err := NewClient().List()
	fmt.Printf("err:%v", err)
}

func TestRedisClient_Set(t *testing.T) {
	err := NewClient().Set()
	fmt.Printf("err:%v", err)
}