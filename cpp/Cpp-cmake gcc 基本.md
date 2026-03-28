
#### GCC

> 预处理-编译-汇编-链接

```
-E     预处理(预编译)(处理宏定义和include，去除注释，不会对语法进行检查) # gcc -E a.c -o a.i
-S     编译过程(这个阶段，检查语法，生成汇编代码) # gcc -S a.i -o a.s
-c     汇编过程(这个阶段，生成目标代码(机器码)) # gcc -c a.s -o a.o
-o     链接过程(生成可执行文件) # gcc a.o -o a.exe

-g     可执行程序包含调试信息(用于gdb)
-Wall  输出警告信息(让编译器工作时输出更多详细信息)
-w     关闭警告信息

-std=c++17 设置编译标准

-O[0~3] 优化代码 # gcc -O2 
-I      指定头文件目录搜索 ( -I/laixhe/code ) 一般在 /usr/include 下搜索
-L      指定库文件目录搜索
-l      指定需要链接的库(会在 /lib、/usr/lib、/usr/local/lib 下搜索)
-D      定义必要的宏 # gcc -DDEBUG 

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

#### cmake
```
-G   指定一个明确的编译环境
     -G "MinGW Makefiles"
     -G "Visual Studio 17 2022"
     -G "Visual Studio 16 2019"
     -G "Visual Studio 15 2017"
-A 
     -A Win32
     -A x64
cmake . -G "MinGW Makefiles" -DCMAKE_MAKE_PROGRAM=mingw32-make -Bbuild/mingw64
cmake --build build/mingw64
```

#### CMakeLists 文件
```
# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 3.17)

# 项目名称和版本 ( CXX 代表使用 C++ 语言 )( C 代表使用 C 语言)
project(CppApp VERSION 1.0 LANGUAGES CXX)


#set(CMAKE_C_STANDARD 99)             # 指定 C 编译版本
set(CMAKE_CXX_STANDARD 17)           # 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD_REQUIRED ON)  #
#set(CMAKE_BUILD_TYPE Debug)          # Debug Release

# 生成可执行文件
add_executable(CppApp main.cpp)

# set()                   定义变量
# include_directories()   指定头文件目录搜索
# link_directories()      指定库文件目录搜索
# add_library()           生成库文件 (生成共享库 add_library(hello SHARED xxx.cpp) )
# add_compiler_options()  添加编译参数 ( add_compiler_options(-Wall -o2) )
# add_executable()        生成可执行文件
# target_link_libraries() 指定需要链接的库
#
# add_subdirectory()      添加子目录，子目录需要有一个 CMakeLists.txt

```
