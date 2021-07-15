
#### GCC

> 预处理-编译-汇编-链接

```
-o     指定生成的文件名
-E     参数预处理   (预处理).i
-S     生成汇编源码 (编译).s
-c     生成机器码   (汇编).o
-Wall  让编译器工作时输出更多详细信息
-I     指定头文件目录 ( -I/laixhe/code ) (设置地临时环境变量set C_INCLUDE_PATH=/laixhe/code)
-L     指定库文件路径
-l     指定需要链接的库
-D     定义必要的宏

-std   指定CPP/C的版本 -std=c++17
```


#### cmake
```
-G   指定一个明确的编译环境
     -G "MinGW Makefiles"
```
