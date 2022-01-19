package main

import (
	"fmt"
	"testing"
)

func TestWeightRoundRobinBalance(t *testing.T) {

	rb := &WeightRoundRobinBalance{}
	rb.Add("127.0.0.1:2001", 1)
	rb.Add("127.0.0.1:2002", 1)
	rb.Add("127.0.0.1:2003", 1)

	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())
	fmt.Println(rb.Next())

}
