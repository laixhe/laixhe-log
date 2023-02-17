package pack

// Message 消息
type Message struct {
	DataLen uint32 // 消息的长度
	ID      uint32 // 消息的ID
	Data    []byte // 消息的内容
}

// NewMessage 创建一个 Message 消息包
func NewMessage(ID uint32, data []byte) *Message {
	return &Message{
		DataLen: uint32(len(data)),
		ID:      ID,
		Data:    data,
	}
}

func (msg *Message) Init(ID uint32, data []byte) {
	msg.ID = ID
	msg.Data = data
	msg.DataLen = uint32(len(data))
}
