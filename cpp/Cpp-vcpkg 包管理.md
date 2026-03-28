#### 安装vcpkg
> https://github.com/microsoft/vcpkg

#### 环境变量
```
VCPKG_ROOT=xxx
VCPKG_DEFAULT_TRIPLET=x64-mingw-dynamic
VCPKG_DEFAULT_HOST_TRIPLET=x64-mingw-dynamic
```

#### 基本
```
# 安装对应的包 (需要注意的是：不指定triplet(可以认为是架构)，默认安装的是x86，大概率还是动态库版本)
vcpkg install pakage_name:triplet (vcpkg.exe install jsoncpp:x64-windows-static)
# 搜索对应名称的包
vcpkg search package_name
# 列出已经安装的包
vcpkg list
# 删除对应的包
vcpkg remove pakage_name
# 导出包
vcpkg export pakage_name --zip
# 列出编译(系统)架构
vcpkg help triplet
```

```
# x64-linux x64-windows x64-osx x64-mingw-dynamic x64-mingw-static

# 这里 :x64-windows 指定了对应的64位版本，不加的话默认时32位的
vcpkg install opencv:x64-windows

# jsoncp libcurl boost gtest
```

#### 集成到全局
> 适用于 Visual Studio 开发环境和 msbuild 命令行
```
vcpkg integrate install
vcpkg integrate remove
```

#### cmake
- vcpkg install yaml-cpp:x64-mingw-dynamic
```
if(DEFINED ENV{VCPKG_ROOT})

  file(TO_CMAKE_PATH "$ENV{VCPKG_ROOT}" ENV_VCPKG_ROOT)
  set(VCPKG_TARGET_TRIPLET "x64-mingw-dynamic" CACHE STRING "" FORCE)
  set(CMAKE_TOOLCHAIN_FILE "${ENV_VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")

  set(yaml-cpp_DIR "${ENV_VCPKG_ROOT}/installed/${VCPKG_TARGET_TRIPLET}/share/yaml-cpp")

endif()

# yaml cpp
find_package(yaml-cpp CONFIG REQUIRED)
include_directories(${YAML_CPP_INCLUDE_DIR})
link_libraries(${YAML_CPP_LIBRARIES})

```
