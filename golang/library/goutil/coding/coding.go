package coding

import (
	"bytes"
	"encoding/binary"
)

// Int32ToBytes 32整型转换成字节
func Int32ToBytes(n int32) ([]byte, error) {

	bytesBuffer := bytes.NewBuffer(nil)

	// binary.BigEndian 进行大端对齐
	err := binary.Write(bytesBuffer, binary.BigEndian, n)
	if err != nil {
		return nil, err
	}

	//binary.BigEndian.PutUint32()

	return bytesBuffer.Bytes(), nil
}

// BytesToInt32 字节转换成整型32
func BytesToInt32(b []byte) (int32, error) {

	bytesBuffer := bytes.NewBuffer(b)

	var n int32

	// binary.BigEndian 进行大端对齐
	err := binary.Read(bytesBuffer, binary.BigEndian, &n)
	if err != nil {
		return 0, err
	}

	//binary.BigEndian.Uint32(b)

	return n, nil
}

// Int64ToBytes 64整型转换成字节
func Int64ToBytes(n int64) ([]byte, error) {

	bytesBuffer := bytes.NewBuffer(nil)

	// binary.BigEndian 进行大端对齐
	err := binary.Write(bytesBuffer, binary.BigEndian, n)
	if err != nil {
		return nil, err
	}

	//binary.BigEndian.PutUint64()

	return bytesBuffer.Bytes(), nil
}

// BytesToInt64 字节转换成整型64
func BytesToInt64(b []byte) (int64, error) {

	bytesBuffer := bytes.NewBuffer(b)

	var n int64

	// binary.BigEndian 进行大端对齐
	err := binary.Read(bytesBuffer, binary.BigEndian, &n)
	if err != nil {
		return 0, err
	}

	//binary.BigEndian.Uint64(b)

	return n, nil
}
