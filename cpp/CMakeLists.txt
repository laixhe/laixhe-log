# 工程(项目)名
project(test_cpp)
# cmake 最低版本需求
cmake_minimum_required(VERSION 3.17)

# 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD 17)

# ${PROJECT_SOURCE_DIR}  项目目录

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
