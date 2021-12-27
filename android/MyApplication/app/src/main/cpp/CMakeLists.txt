# cmake 最低版本需求
cmake_minimum_required(VERSION 3.10.2)

# 工程(项目)名
set(PROJECT_APP_NAME myapplication)
project(${PROJECT_APP_NAME})

# 构建库
# 将 native-lib.cpp 构建出 so 共享库，并命名为 myapplication
add_library( # 构建的库的名字
        ${PROJECT_APP_NAME}

        # 共享库
        SHARED

        # 库的原文件，这里与 CMakeLists.txt 同目录，直接就写 native-lib.cpp 了
        native-lib.cpp)

# 搜索指定的预构建库，并将该路径存储为一个变量
# 通过 find_library 来找到需要关联的第三方库
find_library( # 设置路径变量的名称
        log-lib

        # Specifies the name of the NDK library that
        # 需要关联的 so 名字
        log)

# 指定库的库应该链接到你的目标库
# 通过 link 可将源文件构建的库和三方库都加载进来
target_link_libraries( # 指定目标库
        ${PROJECT_APP_NAME}

        # 链接到目标库
        ${log-lib})

