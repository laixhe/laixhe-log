
### 错误异常处理

> panic 不应跨域 package
>
> recover 只能在 defer 中起作用
>
> panic 只能由当前 goroutine recover
>
> 如果 panic 没有被 recover，那么有可能会挂起整个程序(panic 并没有被解决，一直抛到最上层，使整个程序崩溃)
