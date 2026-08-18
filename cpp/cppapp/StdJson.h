#ifndef CPPAPP_STDJSON_H
#define CPPAPP_STDJSON_H

#include <map>
#include <string>
#include <variant>
#include <vector>

// JSON 序列化/反序列化（手写最小实现，纯标准库）。
// 对应 Go golog/json_test.go 与 Rust serde_json。
// C++23 标准库尚未提供 JSON 类型（C++26 将引入 std::json），
// 这里用 std::variant 实现一个足够演示用的 JSON 值类型。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdJson
{
    public:
    struct Value;

    using Object = std::map<std::string, Value>;
    using Array = std::vector<Value>;

    struct Value
    {
        // null / bool / double / string / array / object
        std::variant<std::nullptr_t, bool, double, std::string, Array, Object> data;

        Value() : data(nullptr) {}
        Value(bool b) : data(b) {}
        Value(double d) : data(d) {}
        Value(int i) : data(static_cast<double>(i)) {}
        Value(const char* s) : data(std::string(s)) {}
        Value(const std::string& s) : data(s) {}
        Value(const Array& a) : data(a) {}
        Value(const Object& o) : data(o) {}

        bool IsNull() const { return std::holds_alternative<std::nullptr_t>(data); }
        bool IsArray() const { return std::holds_alternative<Array>(data); }
        bool IsObject() const { return std::holds_alternative<Object>(data); }
    };

    StdJson();

    // 序列化：json → 字符串（pretty=true 时美化输出，对应 json.MarshalIndent）
    static std::string Serialize(const Value& value, bool pretty = false);
    // 反序列化：字符串 → json（格式错误抛 std::runtime_error，对应 JSONDecodeError）
    static Value Parse(std::string_view json);

    private:
    static void SerializeInto(const Value& value, std::string& out, bool pretty, int indent);
    static Value ParseValue(std::string_view json, std::size_t& pos);
};


#endif //CPPAPP_STDJSON_H
