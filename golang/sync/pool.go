package sync_main

import (
	"bytes"
	"sync"
)

// 对象池
// 使用 sync.Pool 建立一个对象池效果

// 使用结构体主要是规范对象池的类型
type testPool struct {
	pool sync.Pool
}

func NewTestPool() *testPool {
	return &testPool{
		pool: sync.Pool{
			// 声明对象池的类型，且初始化的类型一致
			New: func() interface{} {
				return bytes.NewBuffer(make([]byte, 50))
			},
		},
	}
}

// Get 从对象池里获取一个对象
func (p *testPool) Get() *bytes.Buffer {
	buffer := p.pool.Get().(*bytes.Buffer)
	buffer.Reset() // 将缓冲区重置为空(清空数据)
	return buffer
}

// Put 往对象池里放回一个对象
func (p *testPool) Put(buf *bytes.Buffer) {
	// 往对象池里放回一个对象
	p.pool.Put(buf)
}

// Pool 使用 sync.Pool 建立一个对象池效果
func Pool() {

	p := NewTestPool()
	buf := p.Get()

	// 业务操作...

	p.Put(buf) // 要记得存回去
}
