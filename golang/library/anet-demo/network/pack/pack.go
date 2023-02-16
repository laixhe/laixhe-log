package pack

import (
	"bytes"
	"encoding/binary"
)

const (
	HeadLength uint32 = 4     // 包头长度字节
	ByteOrder  string = "big" // 消息字节排序 little big
)

// Pack 打包消息
func Pack(data []byte) (packet []byte, err error) {
	var buf bytes.Buffer
	buf.Grow(len(data) + int(HeadLength))

	if err = binary.Write(&buf, byteOrder(), uint32(len(data))); err != nil {
		return
	}
	if err = binary.Write(&buf, byteOrder(), data); err != nil {
		return
	}

	packet = buf.Bytes()
	return
}

// Unpack 解包消息
func Unpack(packet []byte) (data []byte, err error) {
	var (
		buf    = bytes.NewBuffer(packet)
		msgLen uint32
	)

	if err = binary.Read(buf, byteOrder(), &msgLen); err != nil {
		return
	}

	if msgLen > 0 {
		data = make([]byte, msgLen)
		if err = binary.Read(buf, byteOrder(), &data); err != nil {
			return
		}
	}
	return
}

func byteOrder() binary.ByteOrder {
	switch ByteOrder {
	case "big":
		return binary.BigEndian
	default:
		return binary.LittleEndian
	}
}
