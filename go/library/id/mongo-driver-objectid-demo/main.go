package main

import (
	"fmt"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

func main() {
	for i:=0; i< 100; i++ {
		fmt.Println(primitive.NewObjectID().Hex())
	}
}
