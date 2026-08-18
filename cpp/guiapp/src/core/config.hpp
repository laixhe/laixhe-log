#pragma once

#include <filesystem>

struct ShapeProps;

// 学习点: 用 nlohmann/json 把 ShapeProps 序列化到文件, 实现配置的保存/加载
bool saveConfig(const ShapeProps& props, const std::filesystem::path& path);
bool loadConfig(ShapeProps& props, const std::filesystem::path& path);
