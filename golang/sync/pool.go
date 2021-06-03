package sync_main

import (
	"bytes"
	"sync"
)

type testPool struct {
	pool sync.Pool
}

func New() *testPool {
	return &testPool{
		pool: sync.Pool{
			New: func() interface{} {
				return bytes.NewBuffer(make([]byte, 10))
			},
		},
	}
}

// 使用 sync.Pool 建立一个缓冲池效果
func Pool() {

	p := New()

	buffer := p.pool.Get().(*bytes.Buffer)
	buffer.Reset()

	// .......

	p.pool.Put(buffer)
	buffer = nil

}
