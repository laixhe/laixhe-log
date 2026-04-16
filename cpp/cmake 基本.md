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

cmake -B build -G "MinGW Makefiles"    生成构建配置目录与文件
cmake --build build                    构建项目
```

#### CMakeLists 文件
```
# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 4.1)
# 项目名称和版本 ( CXX 代表使用 C++ 语言 )( C 代表使用 C 语言)
project(cppapp VERSION 1.0 LANGUAGES CXX)

#set(CMAKE_C_STANDARD 99)             # 指定 C 编译版本
set(CMAKE_CXX_STANDARD 23)            # 指定 C++ 编译版本
set(CMAKE_CXX_STANDARD_REQUIRED ON)   # 并要求编译器支持此 C++ 标准
#set(CMAKE_CXX_MODULE_STD ON)          # 自动编译 std 模块
set(CMAKE_BUILD_TYPE Debug)           # Debug Release

# 用于在构建过程中自动下载和管理外部项目
# include(FetchContent)
# 下载的外部库
#FetchContent_Declare(SFML
#    GIT_REPOSITORY https://github.com/SFML/SFML.git
#    GIT_TAG 3.0.2
#    GIT_SHALLOW ON
#    EXCLUDE_FROM_ALL
#    SYSTEM)
# 将外部库添加到项目中
#FetchContent_MakeAvailable(SFML)

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

# 指定所需的 C++ 标准
#target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_23)
# 链接外部库
#target_link_libraries(${PROJECT_NAME} template)

message("project_name: ${PROJECT_NAME}")

# 设置 Visual Studio 启动项目为 cppapp
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT ${PROJECT_NAME})

# PROJECT_NAME              项目名称
# CMAKE_BINARY_DIR          构建目录
# PROJECT_SOURCE_DIR        工程的根目录
# CMAKE_CURRENT_SOURCE_DIR  当前 CMakeLists.txt 文件所在目录
# PRIVATE
# PUBLIC
# INTERFACE
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
#
# 构建项目
# cmake -B build -G "MinGW Makefiles"    生成构建配置目录与文件
# cmake --build build                    构建项目
#
```
