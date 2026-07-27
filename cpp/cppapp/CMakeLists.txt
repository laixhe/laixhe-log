# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 4.1)
# 项目名称和版本 ( CXX 代表使用 C++ 语言 )
project(cppapp VERSION 1.0 LANGUAGES CXX)

set(CMAKE_C_STANDARD 99)              # 指定 C 编译版本
set(CMAKE_CXX_STANDARD 23)            # 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD_REQUIRED ON)   # 并要求编译器支持此 C++ 标准
#set(CMAKE_CXX_MODULE_STD ON)          # 自动编译 std 模块
set(CMAKE_BUILD_TYPE Debug)           # Debug Release

# 指定可执行文件的输出目录
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
# 设置库文件的输出目录
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
# 设置静态库文件的输出目录
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# 查找当前目录下的所有源文件
# 并将名称保存到 DIR_SRCS 变量
aux_source_directory(. DIR_SRCS)

# 添加 template 子目录
#add_subdirectory(template)

# 添加可执行文件或库
add_executable(${PROJECT_NAME} ${DIR_SRCS})

# 链接外部库
#target_link_libraries(${PROJECT_NAME} template)

message("project_name: ${PROJECT_NAME}")

#
# 构建项目
# cmake -B build -G "MinGW Makefiles"    生成构建配置目录与文件
# cmake --build build                    构建项目
#
