package coding

import (
	"fmt"
	"testing"
)

func TestBytesToInt(t *testing.T) {

	data, err := Int32ToBytes(123)
	if err != nil {
		fmt.Println("IntToBytes : ", err)
		return
	}

	i, err := BytesToInt32(data)
	if err != nil {
		fmt.Println("BytesToInt : ", err)
		return
	}

	fmt.Println(i)

}

func TestBytesToInt64(t *testing.T) {

	data, err := Int64ToBytes(456)
	if err != nil {
		fmt.Println("Int64ToBytes : ", err)
		return
	}

	i, err := BytesToInt64(data)
	if err != nil {
		fmt.Println("BytesToInt64 : ", err)
		return
	}

	fmt.Println(i)

}
