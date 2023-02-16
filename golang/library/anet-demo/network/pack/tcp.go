package pack

import (
	"bytes"
	"encoding/binary"
	"io"
	"net"
)

// TcpRead 读取 TCP 解包消息
func TcpRead(conn net.Conn) (data []byte, err error) {
	packet := make([]byte, HeadLength)
	if _, err = io.ReadFull(conn, packet); err != nil {
		return
	}
	var headLen uint32
	if err = binary.Read(bytes.NewBuffer(packet), byteOrder(), &headLen); err != nil {
		return
	}
	if headLen > 0 {
		data = make([]byte, headLen)
		if _, err = io.ReadFull(conn, data); err != nil {
			return
		}
	}
	return
}
