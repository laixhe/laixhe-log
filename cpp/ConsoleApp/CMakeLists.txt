# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 3.13)

# 工程(项目)名
set(MAIN_NAME ConsoleApp)
# 项目名称，LANGUAGES 项目用于，这里 CXX 代表使用 C++ 语言
project(${MAIN_NAME} VERSION 0.0.1 LANGUAGES C CXX)

# 指定 C C++ 编译版本
set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 17)

# WIN32 UNIX
if(WIN32)
    message("当前系统为：Windows...")
    include_directories(${PROJECT_SOURCE_DIR}/lib/mysql/win32/include/)
    link_directories(${PROJECT_SOURCE_DIR}/lib/mysql/win32/lib/)
    
    # 设置库文件(如lib目录下的libmysql.lib)
    link_libraries(libmysql)

elseif(UNIX)
    message("当前系统为：Unix...")
    include_directories(${PROJECT_SOURCE_DIR}/lib/mysql/linux/include/)
    link_directories(${PROJECT_SOURCE_DIR}/lib/mysql/linux/lib/)
endif()

# ${PROJECT_SOURCE_DIR}  项目目录
#
# 自定义变量，使用 ${} 进行变量的引用
#SET(BIN_DESTINATION ${PROJECT_SOURCE_DIR}/bin)
# 构建库
#add_library()
# 搜索指定的预构建库，并将该路径存储为一个变量
#find_library()
# 指定库的库应该链接到你的目标库(添加链接库，相同于指定-l参数)
#target_link_libraries()
# 搜索目录所有的 cpp 文件并将列表存储在一个变量
#aux_source_directory(src/main/cpp/ DIR_LIB_SRCS)
# 设置头文件引用目录位置
#include_directories()
# 设置链接库引用目录位置
#link_directories()
# 查找包
# find_package()
#
# 打印
# message("T---")
#
# 例子：
#
# 将 native-lib.cpp 构建出 so 共享库，并命名为 hello
# add_library( # 构建的库的名字
#              hello
#              # 共享库
#              SHARED
#              # 库的原文件，这里与 CMakeLists.txt 同目录，直接就写 hello_lib.cpp 了
#              hello_lib.cpp )
#
# 通过 find_library 来找到需要关联的第三方库
# find_library( # 设置路径变量的名称
#               log-lib
#               # 需要关联的 so 名字
#               log )
#
# 通过 link 可将源文件构建的库和三方库都加载进来
# target_link_libraries( # 源文件库的名字
#                        hello
#                        # 第三方库
#                        ${log-lib} )

# mysql
set(mysqlx mysqlx/Mysqlx.h mysqlx/Mysqlx.cpp)
#
set(strings base/Strings.h base/Strings.cpp)
set(config_file base/ConfigFile.h base/ConfigFile.cpp)

# 添加可执行文件
add_executable(${MAIN_NAME} 
${strings} 
${config_file} 
${mysqlx} 
main.cpp)

target_link_libraries(${MAIN_NAME} mysqlclient)
