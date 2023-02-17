package pack

import (
	"bytes"
	"encoding/binary"
)

const (
	DefaultHeaderLen uint32 = 8     // 包头长度方法
	ByteOrder        string = "big" // 消息字节排序 little big
)

// Pack 打包消息
func Pack(msg *Message) (packet []byte, err error) {
	var buf bytes.Buffer
	buf.Grow(len(msg.Data) + int(DefaultHeaderLen))

	if err = binary.Write(&buf, byteOrder(), msg.DataLen); err != nil {
		return
	}
	if err = binary.Write(&buf, byteOrder(), msg.ID); err != nil {
		return
	}
	if err = binary.Write(&buf, byteOrder(), msg.Data); err != nil {
		return
	}
	packet = buf.Bytes()
	return
}

// Unpack 解包消息
func Unpack(packet []byte) (msg *Message, err error) {
	msg = &Message{}
	var buf = bytes.NewBuffer(packet)
	if err = binary.Read(buf, byteOrder(), &msg.DataLen); err != nil {
		return
	}
	if err = binary.Read(buf, byteOrder(), &msg.ID); err != nil {
		return
	}
	if msg.DataLen > 0 {
		msg.Data = make([]byte, msg.DataLen)
		if err = binary.Read(buf, byteOrder(), &msg.Data); err != nil {
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
