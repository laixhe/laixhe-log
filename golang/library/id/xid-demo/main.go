package main

import (
	"fmt"

	"github.com/rs/xid"
)

func main() {
	for i:=0; i< 100; i++ {
		fmt.Println(xid.New().String())
	}
}
