#### GCC

> 预处理-编译-汇编-链接

```
-E     预处理(预编译)(处理宏定义和include，去除注释，不会对语法进行检查) # gcc -E a.c -o a.i
-S     编译过程(这个阶段，检查语法，生成汇编代码) # gcc -S a.i -o a.s
-c     汇编过程(这个阶段，生成目标代码(机器码)) # gcc -c a.s -o a.o
-o     链接过程(生成可执行文件) # gcc a.o -o a.exe

-g     可执行程序包含调试信息(用于gdb)
-std   指定 CPP/C 的版本 -std=c++17

-Wall  让编译器工作时输出更多详细信息
-I     指定头文件目录 ( -I/laixhe/code ) (设置地临时环境变量set C_INCLUDE_PATH=/laixhe/code)
-L     指定库文件路径
-l     指定需要链接的库
-D     定义必要的宏
```

> 生成动态库

```
gcc xxx.c -fPIC -shared -o libxxx.so
#gcc xxx.c -c -fPIC -o xxx.o
#gcc -shared -o libxxx.so xxx.o
```

> 生成静态库

```
gcc xxx.c -c -o xxx.o
ar rs libxxx.a xxx.o
```
