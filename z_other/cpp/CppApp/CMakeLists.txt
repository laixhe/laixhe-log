# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 3.17)

# 项目名称和版本 ( CXX 代表使用 C++ 语言 )( C 代表使用 C 语言)
project(cppapp VERSION 1.0 LANGUAGES CXX)


#set(CMAKE_C_STANDARD 99)             # 指定 C 编译版本
set(CMAKE_CXX_STANDARD 17)           # 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD_REQUIRED ON)  #
set(CMAKE_BUILD_TYPE Debug)          # Debug Release

# 生成可执行文件
add_executable(cppapp main.cpp std_string.h std_string.cpp std_function.h std_function.cpp std_const.h std_const.cpp)

# set()                   定义变量
# include_directories()   指定头文件目录搜索
# link_directories()      指定库文件目录搜索
# add_library()           生成库文件 (生成共享库 add_library(hello SHARED xxx.cpp) )
# add_compiler_options()  添加编译参数 ( add_compiler_options(-Wall -o2) )
# add_executable()        生成可执行文件
# target_link_libraries() 指定需要链接的库
#
# add_subdirectory()      添加子目录，子目录需要有一个 CMakeLists.txt
