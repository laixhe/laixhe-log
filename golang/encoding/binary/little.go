package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
)

// 大端、小端

// Int32ToBytes 32整型转换成字节
func Int32ToBytes(n int32) ([]byte, error) {

	bytesBuffer := bytes.NewBuffer(nil)

	// binary.BigEndian 进行 大端 对齐
	err := binary.Write(bytesBuffer, binary.BigEndian, n)
	if err != nil {
		return nil, err
	}

	return bytesBuffer.Bytes(), nil
}

// BytesToInt32 字节转换成整型32
func BytesToInt32(b []byte) (int32, error) {

	bytesBuffer := bytes.NewBuffer(b)
	var n int32

	// binary.BigEndian 进行 大端 对齐
	err := binary.Read(bytesBuffer, binary.BigEndian, &n)
	if err != nil {
		return 0, err
	}

	return n, nil
}

// Uint64ToBytes 64整型转换成字节
func Uint64ToBytes(n uint64) ([]byte, error) {
	buffer := make([]byte, 8)
	// binary.LittleEndian 进行 小端 对齐
	binary.LittleEndian.PutUint64(buffer, n)
	return buffer, nil
}

// BytesToUint64 字节转换成整型64
func BytesToUint64(b []byte) (uint64, error) {
	// binary.LittleEndian 进行 小端 对齐
	return binary.LittleEndian.Uint64(b), nil
}

func main() {
	fmt.Println(Int32ToBytes(-123))
	fmt.Println(BytesToInt32([]byte{255, 255, 255, 133}))
	fmt.Println(Uint64ToBytes(133))
	fmt.Println(BytesToUint64([]byte{133, 0, 0, 0, 0, 0, 0, 0}))
}
