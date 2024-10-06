
> 相关源代码: str/runtime/runtime2.go
```
# 对应的类似 type xxx interface {}
#
type iface struct {
	tab  *itab
	data unsafe.Pointer
}

# 对应的类似 var xxx interface
#
type eface struct {
	_type *_type
	data  unsafe.Pointer
}
```

