package pack

import (
	"bufio"
	"encoding/binary"
	"net"
)

// TcpRead 读取 TCP 解包消息
func TcpRead(conn net.Conn) (msg *Message, err error) {
	buf := bufio.NewReader(conn)
	return TcpBufRead(buf)
}

// TcpBufRead 读取 TCP 解包消息
func TcpBufRead(buf *bufio.Reader) (msg *Message, err error) {
	msg = &Message{}

	if err = binary.Read(buf, byteOrder(), &msg.ID); err != nil {
		return
	}
	if err = binary.Read(buf, byteOrder(), &msg.DataLen); err != nil {
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
