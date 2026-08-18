#include "config.hpp"

#include "shape_props.hpp"

#include <spdlog/spdlog.h>

#include <nlohmann/json.hpp>

#include <algorithm>
#include <array>
#include <cstring>
#include <fstream>

namespace {
using json = nlohmann::json;

// 学习点: nlohmann/json 序列化 - 结构体 -> JSON
json propsToJson(const ShapeProps& p)
{
    return json{
        {"radius", p.radius},
        {"x", p.x},
        {"y", p.y},
        {"rotationSpeed", p.rotationSpeed},
        {"shapeType", p.shapeType},
        {"filled", p.filled},
        {"color", {p.color[0], p.color[1], p.color[2], p.color[3]}},
        {"showGrid", p.showGrid},
        {"showText", p.showText},
        {"showParticles", p.showParticles},
        {"text", std::string(p.text)},
    };
}

// 学习点: nlohmann/json 反序列化 - JSON -> 结构体(缺字段时保持默认值, 便于向前兼容)
void propsFromJson(const json& j, ShapeProps& p)
{
    if (j.contains("radius")) {
        p.radius = j.at("radius").get<float>();
    }
    if (j.contains("x")) {
        p.x = j.at("x").get<float>();
    }
    if (j.contains("y")) {
        p.y = j.at("y").get<float>();
    }
    if (j.contains("rotationSpeed")) {
        p.rotationSpeed = j.at("rotationSpeed").get<float>();
    }
    if (j.contains("shapeType")) {
        p.shapeType = j.at("shapeType").get<int>();
    }
    if (j.contains("filled")) {
        p.filled = j.at("filled").get<bool>();
    }
    if (j.contains("color")) {
        const auto c = j.at("color").get<std::array<float, 4>>();
        std::copy(c.begin(), c.end(), p.color);
    }
    if (j.contains("showGrid")) {
        p.showGrid = j.at("showGrid").get<bool>();
    }
    if (j.contains("showText")) {
        p.showText = j.at("showText").get<bool>();
    }
    if (j.contains("showParticles")) {
        p.showParticles = j.at("showParticles").get<bool>();
    }
    if (j.contains("text")) {
        const std::string s = j.at("text").get<std::string>();
        std::strncpy(p.text, s.c_str(), sizeof(p.text) - 1);
        p.text[sizeof(p.text) - 1] = '\0';
    }
}
}  // namespace

bool saveConfig(const ShapeProps& props, const std::filesystem::path& path)
{
    std::ofstream file(path);
    if (!file) {
        spdlog::error("无法打开配置文件 {} 进行写入", path.string());
        return false;
    }
    file << propsToJson(props).dump(2);
    spdlog::info("配置已保存到 {}", path.string());
    return true;
}

bool loadConfig(ShapeProps& props, const std::filesystem::path& path)
{
    std::ifstream file(path);
    if (!file) {
        spdlog::warn("配置文件 {} 不存在, 保持默认设置", path.string());
        return false;
    }
    json j;
    try {
        file >> j;
    } catch (const json::parse_error& e) {
        spdlog::error("配置文件 {} 解析失败: {}", path.string(), e.what());
        return false;
    }
    propsFromJson(j, props);
    spdlog::info("已从 {} 加载配置", path.string());
    return true;
}
