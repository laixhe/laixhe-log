# 用于构建 CMake 项目 CMake 最低版本
cmake_minimum_required(VERSION 3.20)

# 项目名称和版本 ( CXX 代表使用 C++ 语言 )
project(QtApp VERSION 0.1 LANGUAGES CXX)

# QT编译时独有的参数，默认是关闭的
set(CMAKE_AUTOUIC ON) # 启用自动UI编译(以 .ui 文件)
set(CMAKE_AUTOMOC ON) # 自动元对象编译
set(CMAKE_AUTORCC ON) # 自动资源编译(以 .qrc 文件)

# 指定 C++ 编译版本，并要求编译器支持此 C++ 标准
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

#set(CMAKE_PREFIX_PATH "D:/Qt/6.5.0/mingw_64")
message("T---------- CMAKE_PREFIX_PATH: [${CMAKE_PREFIX_PATH}]")

# 查找环境变量 VCPKG_ROOT
if(DEFINED ENV{VCPKG_ROOT})
    file(TO_CMAKE_PATH "$ENV{VCPKG_ROOT}" ENV_VCPKG_ROOT)
    set(VCPKG_TARGET_TRIPLET "x64-mingw-dynamic" CACHE STRING "" FORCE)
    set(CMAKE_TOOLCHAIN_FILE "${ENV_VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE STRING "")

    # 通过 vcpkg install yaml-cpp:x64-mingw-dynamic 安装后导入
    #set(yaml-cpp_DIR "${ENV_VCPKG_ROOT}/installed/${VCPKG_TARGET_TRIPLET}/share/yaml-cpp")

    message("T---------- CMAKE_TOOLCHAIN_FILE: [${CMAKE_TOOLCHAIN_FILE}] [${VCPKG_TARGET_TRIPLET}]")
endif()

# 寻找 QT 的库
find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets)

# 查找代码文件
file(GLOB SOURCE_FILES
        src/mainwindow/*.h
        src/mainwindow/*.cpp
        src/mainwindow/*.ui
        src/widgets/*.h
        src/widgets/*.cpp
)
#set(RCC_FILES rs.qrc)

set(PROJECT_SOURCES
        ${SOURCE_FILES}
        src/main.cpp
)

if(${QT_VERSION_MAJOR} GREATER_EQUAL 6)
    # 从 Qt5.14 版本开始，QT 推荐使用 qt_add_executable 命令代替 add_executable
    # 以更好地集成 Qt 的构建系统
    qt_add_executable(${PROJECT_NAME}
        MANUAL_FINALIZATION
        ${PROJECT_SOURCES}
    )
else()
    add_executable(${PROJECT_NAME}
        ${PROJECT_SOURCES}
    )
endif()

# # 将 Qt 模块链接到目标可执行文件或动态库
target_link_libraries(${PROJECT_NAME}
    PRIVATE
    Qt${QT_VERSION_MAJOR}::Widgets
)

# 设置目标可执行文件的属性，如 MacOSX 的 Bundle 标识符、版本号和短版本字符串，以及在 Windows 下作为可执行文件运行
set_target_properties(${PROJECT_NAME} PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER my.example.com
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

# 安装目标可执行文件到指定的目录
install(TARGETS ${PROJECT_NAME}
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

# 进行最后的可执行文件处理
if(QT_VERSION_MAJOR EQUAL 6)
    qt_finalize_executable(${PROJECT_NAME})
    # 在使用 Qt6 的情况下，qt_add_executable 命令会生成一个中间目标文件，该文件需要经过最后的处理步骤才能成为可执行文件。
endif()
