package main

import (
	"fmt"

	gofrsUuid "github.com/gofrs/uuid/v5"
	googleUuid "github.com/google/uuid"
)

func main() {
	fmt.Println(googleUuid.New().String())
	fmt.Println(googleUuid.NewString())

	fmt.Println(gofrsUuid.Must(gofrsUuid.NewV4()).String())
}
