#### Makefile
```
.PHONY: init
init:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
	go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway@latest
	go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2@latest
	go install github.com/go-kratos/kratos/cmd/kratos/v2@latest
	go install github.com/go-kratos/kratos/cmd/protoc-gen-go-http/v2@latest
	go install github.com/go-kratos/kratos/cmd/protoc-gen-go-errors/v2@latest
	go install github.com/google/gnostic/cmd/protoc-gen-openapi@latest
	go install github.com/favadi/protoc-go-inject-tag@latest
	go install github.com/swaggo/swag/cmd/swag@latest
	go install entgo.io/ent/cmd/ent@latest
	go install github.com/gogf/gf/cmd/gf/v2@latest
	go install github.com/go-eagle/eagle/cmd/eagle@latest
	go install github.com/zeromicro/go-zero/tools/goctl@latest
	go install github.com/sqlc-dev/sqlc/cmd/sqlc@latest
	go install github.com/air-verse/air@latest
	go install github.com/magefile/mage@latest
	
	go install github.com/wailsapp/wails/v2/cmd/wails@latest
	
	go install golang.org/x/telemetry/cmd/gotelemetry@latest
	go install golang.org/x/tools/gopls@latest
	go install github.com/cweill/gotests/gotests@latest
	go install github.com/fatih/gomodifytags@latest
	go install github.com/josharian/impl@latest
	go install github.com/haya14busa/goplay/cmd/goplay@latest
	go install github.com/go-delve/delve/cmd/dlv@latest
	go install honnef.co/go/tools/cmd/staticcheck@latest
	
	go install golang.org/x/mobile/cmd/gobind@latest
	go install golang.org/x/mobile/cmd/gomobile@latest
```

#### -
```
package main

import (
	"fmt"
	"unsafe"
)

type TestUnsafe struct {
	Name     string // 16字节
	IsAdmin  bool   // 1字节
	Age      int32  // 4字节
	IsVerify bool   // 1字节
}

func main() {
	tu := TestUnsafe{
		Name:     "laixhe",
		IsAdmin:  true,
		Age:      18,
		IsVerify: false,
	}

	fmt.Println("TestUnsafe总内存=", unsafe.Sizeof(tu))       // 32
	fmt.Println("Name内存=", unsafe.Sizeof(tu.Name))         // 16
	fmt.Println("IsAdmin内存=", unsafe.Sizeof(tu.IsAdmin))   // 1
	fmt.Println("Age内存=", unsafe.Sizeof(tu.Age))           // 4
	fmt.Println("IsVerify内存=", unsafe.Sizeof(tu.IsVerify)) // 1
}

```

#### -
```
一个字节 8 位,一个 int32 有4个字节共有 32 位
func main(){
    // 32 位，代表 32 种可能
	var flags uint32 = 0

	flags |= 1 << 0
	fmt.Println("flags |= 1 << 0", flags)

	flags |= 1 << 1
	fmt.Println("flags |= 1 << 1", flags)

	flags |= 1 << 2
	fmt.Println("flags |= 1 << 2", flags)

	flags |= 1 << 3
	fmt.Println("flags |= 1 << 3", flags)

	flags |= 1 << 4
	fmt.Println("flags |= 1 << 4", flags)

	flags |= 1 << 5
	fmt.Println("flags |= 1 << 5", flags)

	flags |= 1 << 6
	fmt.Println("flags |= 1 << 6", flags)

	flags |= 1 << 7
	fmt.Println("flags |= 1 << 7", flags)

	flags |= 1 << 8
	fmt.Println("flags |= 1 << 8", flags)

	flags |= 1 << 9
	fmt.Println("flags |= 1 << 9", flags)

	flags |= 1 << 10
	fmt.Println("flags |= 1 << 10", flags)

	flags |= 1 << 11
	fmt.Println("flags |= 1 << 11", flags)

	flags |= 1 << 12
	fmt.Println("flags |= 1 << 12", flags)

	flags |= 1 << 13
	fmt.Println("flags |= 1 << 13", flags)

	flags |= 1 << 14
	fmt.Println("flags |= 1 << 14", flags)

	flags |= 1 << 15
	fmt.Println("flags |= 1 << 15", flags)

	fmt.Println((flags&1) != 0, 0, "(flags & 1) != 0", 1<<0)
	fmt.Println((flags&2) != 0, 1, "(flags & 2) != 0", 1<<1)
	fmt.Println((flags&4) != 0, 2, "(flags & 4) != 0", 1<<2)
	fmt.Println((flags&8) != 0, 3, "(flags & 8) != 0", 1<<3)
	fmt.Println((flags&16) != 0, 4, "(flags & 16) != 0", 1<<4)
	fmt.Println((flags&32) != 0, 5, "(flags & 32) != 0", 1<<5)
	fmt.Println((flags&64) != 0, 6, "(flags & 64) != 0", 1<<6)
	fmt.Println((flags&128) != 0, 7, "(flags & 128) != 0", 1<<7)
	fmt.Println((flags&256) != 0, 8, "(flags & 256) != 0", 1<<8)
	fmt.Println((flags&512) != 0, 9, "(flags & 512) != 0", 1<<9)
	fmt.Println((flags&1024) != 0, 10, "(flags & 1024) != 0", 1<<10)
	fmt.Println((flags&2048) != 0, 11, "(flags & 2048) != 0", 1<<11)
	fmt.Println((flags&4096) != 0, 12, "(flags & 4096) != 0", 1<<12)
	fmt.Println((flags&8192) != 0, 13, "(flags & 8192) != 0", 1<<13)
	fmt.Println((flags&16384) != 0, 14, "(flags & 16384) != 0", 1<<14)
	fmt.Println((flags&32768) != 0, 15, "(flags & 32768) != 0", 1<<15)
}
```
