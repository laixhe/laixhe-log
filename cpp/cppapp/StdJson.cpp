#include "StdJson.h"

#include <charconv>   // std::from_chars（double 解析）
#include <cmath>      // std::isfinite
#include <cstddef>
#include <cstdint>
#include <format>     // std::println
#include <iostream>
#include <stdexcept>  // std::runtime_error
#include <string_view>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

namespace
{
    // 跳过空白字符
    void SkipWhitespace(std::string_view json, std::size_t& pos)
    {
        while (pos < json.size() && (json[pos] == ' ' || json[pos] == '\t' || json[pos] == '\n' || json[pos] == '\r')) {
            pos++;
        }
    }

    // 解析 JSON 字符串（含转义）
    std::string ParseString(std::string_view json, std::size_t& pos)
    {
        if (json[pos] != '"') throw std::runtime_error("expected '\"'");
        pos++;
        std::string out;
        while (pos < json.size()) {
            const char c = json[pos++];
            if (c == '"') return out;
            if (c == '\\') {
                if (pos >= json.size()) throw std::runtime_error("unexpected end");
                const char esc = json[pos++];
                switch (esc) {
                    case '"': out += '"'; break;
                    case '\\': out += '\\'; break;
                    case '/': out += '/'; break;
                    case 'n': out += '\n'; break;
                    case 't': out += '\t'; break;
                    case 'r': out += '\r'; break;
                    default: throw std::runtime_error("unsupported escape");
                }
            } else {
                out += c;
            }
        }
        throw std::runtime_error("unterminated string");
    }
} // namespace

// 递归下降解析：value → object / array / string / number / bool / null
StdJson::Value StdJson::ParseValue(std::string_view json, std::size_t& pos)
{
    SkipWhitespace(json, pos);
    if (pos >= json.size()) throw std::runtime_error("unexpected end");

    const char c = json[pos];
    if (c == '{') { // object
        pos++;
        Object obj;
        SkipWhitespace(json, pos);
        if (pos < json.size() && json[pos] == '}') { pos++; return Value(obj); }
        while (true) {
            SkipWhitespace(json, pos);
            const std::string key = ParseString(json, pos);
            SkipWhitespace(json, pos);
            if (json[pos] != ':') throw std::runtime_error("expected ':'");
            pos++;
            obj[key] = ParseValue(json, pos);
            SkipWhitespace(json, pos);
            if (json[pos] == ',') { pos++; continue; }
            if (json[pos] == '}') { pos++; break; }
            throw std::runtime_error("expected ',' or '}'");
        }
        return Value(obj);
    }
    if (c == '[') { // array
        pos++;
        Array arr;
        SkipWhitespace(json, pos);
        if (pos < json.size() && json[pos] == ']') { pos++; return Value(arr); }
        while (true) {
            arr.push_back(ParseValue(json, pos));
            SkipWhitespace(json, pos);
            if (json[pos] == ',') { pos++; continue; }
            if (json[pos] == ']') { pos++; break; }
            throw std::runtime_error("expected ',' or ']'");
        }
        return Value(arr);
    }
    if (c == '"') return Value(ParseString(json, pos));
    if (json.substr(pos, 4) == "true") { pos += 4; return Value(true); }
    if (json.substr(pos, 5) == "false") { pos += 5; return Value(false); }
    if (json.substr(pos, 4) == "null") { pos += 4; return Value(); }

    // number：from_chars 支持整数与小数
    double d = 0;
    const auto [ptr, ec] = std::from_chars(json.data() + pos, json.data() + json.size(), d);
    if (ec != std::errc{}) throw std::runtime_error("invalid number");
    pos = static_cast<std::size_t>(ptr - json.data());
    return Value(d);
}

StdJson::Value StdJson::Parse(std::string_view json)
{
    std::size_t pos = 0;
    Value v = ParseValue(json, pos);
    SkipWhitespace(json, pos);
    if (pos != json.size()) throw std::runtime_error("trailing characters");
    return v;
}

// 序列化（内部递归）
void StdJson::SerializeInto(const Value& value, std::string& out, bool pretty, int indent)
{
    // 缩进：容器内元素比容器深一级，闭合括号与容器同级
    const auto pad = [&](int level) {
        if (pretty) {
            out.append(static_cast<std::size_t>(level * 2), ' ');
        }
    };

    if (value.IsNull()) {
        out += "null";
    } else if (const bool* b = std::get_if<bool>(&value.data)) {
        out += *b ? "true" : "false";
    } else if (const double* d = std::get_if<double>(&value.data)) {
        // 整数形式不输出小数点（对应 JSON 数字序列化）
        if (std::isfinite(*d) && *d == static_cast<long long>(*d)) {
            out += std::to_string(static_cast<long long>(*d));
        } else {
            out += std::to_string(*d);
        }
    } else if (const std::string* s = std::get_if<std::string>(&value.data)) {
        out += '"';
        out += *s; // 简化：不转义内部引号（演示场景不涉及）
        out += '"';
    } else if (const Array* arr = std::get_if<Array>(&value.data)) {
        if (arr->empty()) {
            out += "[]";
            return;
        }
        out += '[';
        for (std::size_t i = 0; i < arr->size(); i++) {
            if (i > 0) out += ',';
            if (pretty) out += '\n';
            pad(indent + 1);
            SerializeInto((*arr)[i], out, pretty, indent + 1);
        }
        if (pretty) out += '\n';
        pad(indent);
        out += ']';
    } else if (const Object* obj = std::get_if<Object>(&value.data)) {
        if (obj->empty()) {
            out += "{}";
            return;
        }
        out += '{';
        std::size_t i = 0;
        for (const auto& [k, v] : *obj) {
            if (i++ > 0) out += ',';
            if (pretty) out += '\n';
            pad(indent + 1);
            out += '"';
            out += k;
            out += "\":";
            if (pretty) out += ' ';
            SerializeInto(v, out, pretty, indent + 1);
        }
        if (pretty) out += '\n';
        pad(indent);
        out += '}';
    }
}

std::string StdJson::Serialize(const Value& value, bool pretty)
{
    std::string out;
    SerializeInto(value, out, pretty, 0);
    return out;
}

StdJson::StdJson()
{
    std::cout << "--- JSON 序列化 / 反序列化 ---" << std::endl;

    // ===== 1. 基础序列化 / 反序列化 =====
    Object user{
        {"name", Value("laixhe")},
        {"age", Value(18)},
        {"tags", Value(Array{Value("go"), Value("rust")})},
    };
    const std::string s = Serialize(Value(user));
    PRINT("序列化: {}", s);
    // {"name":"laixhe","age":18,"tags":["go","rust"]}

    const Value parsed = Parse(s);
    const auto& user_obj = std::get<Object>(parsed.data);
    PRINT("反序列化 name = {}", std::get<std::string>(user_obj.at("name").data));
    PRINT("反序列化 age = {}", std::get<double>(user_obj.at("age").data));

    // ===== 2. omitempty：空值忽略（对应 Go TestJson，手动过滤）=====
    Object t_json{
        {"time1", Value()},          // null → 忽略
        {"array1", Value(Array{})},  // 空数组 → 忽略
        {"name", Value("ok")},
    };
    // 手动过滤 null 与空数组
    Object filtered;
    for (auto& [k, v] : t_json) {
        if (v.IsNull()) continue;
        if (v.IsArray() && std::get<Array>(v.data).empty()) continue;
        filtered[k] = v;
    }
    PRINT("omitempty 后: {}", Serialize(Value(filtered))); // {"name":"ok"}

    // ===== 3. 数值以字符串形式序列化（对应 Go string tag）=====
    Object query{
        {"path", Value("/index/index")},
        {"age", Value("18")},       // 数字转字符串形式
        {"score", Value("88.8")},
        {"is_pass", Value("false")},
    };
    PRINT("query: {}", Serialize(Value(query)));
    // {"path":"/index/index","age":"18","score":"88.8","is_pass":"false"}

    // ===== 4. 美化输出（对应 json.MarshalIndent）=====
    std::cout << "美化输出:" << std::endl;
    std::cout << Serialize(Value(user), true) << std::endl;

    // ===== 5. 解析失败抛异常（对应 JSONDecodeError）=====
    try {
        Parse("{invalid json");
    } catch (const std::runtime_error& e) {
        PRINT("解析失败: {}", e.what());
    }
}
