package encoding_binary

import (
	"fmt"
	"testing"
)

func TestToInt32(t *testing.T) {
	fmt.Println(Int32ToBytes(-123))
	fmt.Println(BytesToInt32([]byte{255, 255, 255, 133}))
}

func TestToUint64(t *testing.T) {
	fmt.Println(Uint64ToBytes(133))
	fmt.Println(BytesToUint64([]byte{133, 0, 0, 0, 0, 0, 0, 0}))
}
