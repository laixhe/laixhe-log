cmake_minimum_required(VERSION 4.1)
project(sfmlapp VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Debug)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# 禁用 HTTPS 证书验证，解决国内或代理环境下下载 Github 源码失败的问题
set(CMAKE_TLS_VERIFY OFF)

include(FetchContent)

FetchContent_Declare(
        json
        GIT_REPOSITORY https://github.com/nlohmann/json.git
        GIT_TAG v3.12.0
        #TLS_VERIFY OFF # 解决 SSL 证书错误
)
FetchContent_MakeAvailable(json)

FetchContent_Declare(
        spdlog
        GIT_REPOSITORY https://github.com/gabime/spdlog.git
        GIT_TAG v1.17.0
        #TLS_VERIFY OFF # 解决 SSL 证书错误
)
FetchContent_MakeAvailable(spdlog)

FetchContent_Declare(
        SFML
        GIT_REPOSITORY https://github.com/SFML/SFML.git
        GIT_TAG 3.1.0
        #TLS_VERIFY OFF # 解决 SSL 证书错误
)
# 本示例只用 Graphics/Audio, 关闭 Network 避免额外下载 mbedtls
option(SFML_BUILD_AUDIO "Build audio" ON)
option(SFML_BUILD_NETWORK "Build network" OFF)
FetchContent_MakeAvailable(SFML)

add_executable(${PROJECT_NAME}
        src/main.cpp
        src/core/audio.cpp
        src/core/collision.cpp
        src/core/mouse.cpp
        src/core/particles.cpp
        src/core/player.cpp
        src/core/shapes.cpp
        src/core/sprite.cpp
        src/core/text.cpp
        src/core/vertex_array.cpp
        src/core/view.cpp)

target_include_directories(${PROJECT_NAME} PRIVATE src)

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_23)
target_link_libraries(${PROJECT_NAME} PRIVATE nlohmann_json::nlohmann_json)
target_link_libraries(${PROJECT_NAME} PRIVATE spdlog::spdlog)
target_link_libraries(${PROJECT_NAME} PRIVATE SFML::Graphics)
target_link_libraries(${PROJECT_NAME} PRIVATE SFML::Audio)

# cmake -B Build -D CMAKE_BUILD_TYPE=Debug -G "MinGW Makefiles"
# cmake --build Build --config Debug -j