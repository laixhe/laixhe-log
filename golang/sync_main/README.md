
### Mutex 互斥锁 - 提供了两个方法
```
func (*Mutex) Lock    // 锁定
func (*Mutex) Unlock  // 解锁
```

### RWMutex 读写锁 - 提供了四个方法
```
func (*RWMutex) Lock    // 写锁定
func (*RWMutex) Unlock  // 写解锁
func (*RWMutex) RLock   // 读锁定
func (*RWMutex) RUnlock // 读解锁
```

### Cond 条件等待 - 提供了三个方法
```
// 唤醒所有等待的 Wait，建议在“更改条件”时锁定 c.L，更改完毕再解锁
func (*Cond) Broadcast()
// 唤醒一个等待的 Wait，建议在“更改条件”时锁定 c.L，更改完毕再解锁
func (*Cond) Signal()
// 会解锁 c.L 并进入等待状态，在被唤醒时，会重新锁定 c.L
func (*Cond) Wait()
```

### WaitGroup 等待组 - 提供了三个方法
```
// 计数器增加 delta，delta 可以是负数
func (*WaitGroup) Add(delta int)
// 计数器减少 1
func (*WaitGroup) Done()
// 等待直到计数器归零。如果计数器小于 0，则该操作会引发 panic
func (*WaitGroup) Wait()
```

### Pool 临时对象池 - 提供了两个方法
```
New func() interface{}
func (*Pool).Put(x interface{})
func (*Pool).Get()
```
