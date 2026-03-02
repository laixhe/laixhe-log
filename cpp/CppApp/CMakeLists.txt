# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 3.28)

# 项目名称和版本 ( CXX 代表使用 C++ 语言 )( C 代表使用 C 语言)
project(cppapp VERSION 1.0 LANGUAGES CXX)

#set(CMAKE_C_STANDARD 99)             # 指定 C 编译版本
set(CMAKE_CXX_STANDARD 23)            # 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD_REQUIRED ON)   # 并要求编译器支持此 C++ 标准
set(CMAKE_CXX_MODULE_STD 1)           # 自动编译 std 模块
set(CMAKE_BUILD_TYPE Debug)           # Debug Release

# 查找当前目录下的所有源文件
# 并将名称保存到 DIR_SRCS 变量
aux_source_directory(. DIR_SRCS)

# 添加 template 子目录
#add_subdirectory(template)

# 生成可执行文件
add_executable(${PROJECT_NAME} ${DIR_SRCS})

# 添加链接库
target_link_libraries(${PROJECT_NAME} template)

message("project_name: ${PROJECT_NAME}")

# PROJECT_NAME              项目名称
# PROJECT_SOURCE_DIR        工程的根目录
# CMAKE_CURRENT_SOURCE_DIR  当前 CMakeLists.txt 文件所在目录
#
# set()                   定义变量
# include_directories()   指定头文件目录搜索
# link_directories()      指定库文件目录搜索
# find_package()          查找第三方库
# find_package(yaml-cpp CONFIG REQUIRED) 查找第三方库(使用 config 模式)
# add_library()           生成库文件 (生成共享库 add_library(hello SHARED xxx.cpp) )
# add_compiler_options()  添加编译参数 ( add_compiler_options(-Wall -o2) )
# add_executable()        生成可执行文件
# target_link_libraries() 指定需要链接的库
#
# add_subdirectory()      添加子目录，子目录需要有一个 CMakeLists.txt
