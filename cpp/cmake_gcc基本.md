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

#### cmake
```
-G   指定一个明确的编译环境
     -G "MinGW Makefiles"
```

#### CMakeLists 文件
```
# cmake 最低版本需求
cmake_minimum_required(VERSION 3.30)

# 项目名称和版本 ( CXX 代表使用 C++ 语言 )( C 代表使用 C 语言)
project(test_cpp VERSION 1.0 LANGUAGES CXX)

# 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD 20)
# 并要求编译器支持此 C++ 标准
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 项目目录 ${PROJECT_SOURCE_DIR}  

# 定义变量
#SET(BIN_DESTINATION ${PROJECT_SOURCE_DIR}/bin)
# 构建库
#add_library()
# 搜索指定的预构建库，并将该路径存储为一个变量
#find_library()
# 指定库的库应该链接到你的目标库
#target_link_libraries()
# 搜索目录所有的 cpp 文件并将列表存储在一个变量
#aux_source_directory(src/main/cpp/ DIR_LIB_SRCS)
# 设置头文件引用目录位置
#include_directories()
# 设置链接库引用目录位置
#link_directories()

# 添加可执行文件
add_executable(test_cpp main.cpp)
```
