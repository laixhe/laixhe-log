<?php

namespace Laixhe\Phplog;

/**
 * JSON 序列化/反序列化示例（json_encode / json_decode，内置扩展）。
 * 对应 Go golog/json_test.go 与 Java JsonDemo。
 *
 * 常用参数与 Go Tag 对应关系：
 * - JSON_UNESCAPED_UNICODE → 中文不转义（Go 默认转义 \uXXXX）
 * - JSON_PRETTY_PRINT      → 美化输出（对应 json.MarshalIndent）
 * - JSON_UNESCAPED_SLASHES → 不转义 /
 * - omitempty 语义         → 手动过滤 null/空值后编码（PHP 无内置 tag）
 * - "-" 忽略字段           → unset() 掉再编码
 * - string 数字            → PHP 数字自动序列化为 JSON number（可加引号手动转字符串）
 */
final class JsonDemo
{
    // 对应 Go TJson：演示 omitempty（空值忽略）与 null/空集合的区别
    public static function jsonBasic(): void
    {
        // 关联数组 -> JSON（对应 struct 序列化）
        $tJson = [
            'time1' => null,     // 对应 omitempty：null 应被忽略
            'array1' => [],      // 对应 omitempty：空数组应被忽略
            'map1' => [],        // 对应 omitempty：空字典应被忽略
        ];

        // 手动实现 omitempty：过滤掉 null / 空数组 / 空字符串
        $filtered = array_filter($tJson, fn ($v) => $v !== null && $v !== [] && $v !== '');

        // 转为对象再编码：空对象输出 {}（PHP 空数组默认输出 []）
        $s1 = json_encode((object) $filtered, JSON_UNESCAPED_UNICODE);
        echo $s1, PHP_EOL; // 结果：{}（全部被忽略）

        // 非空值正常序列化
        $full = [
            'time1' => '2025-06-21T09:18:39Z',
            'array1' => [1, 2],
            'map1' => ['a' => '1'],
        ];
        echo json_encode($full, JSON_UNESCAPED_UNICODE), PHP_EOL;
        // 结果：{"time1":"2025-06-21T09:18:39Z","array1":[1,2],"map1":{"a":"1"}}

        // 对象序列化（对应 Go struct / Java record）
        $obj = (object) ['name' => 'laixhe', 'age' => 18];
        echo json_encode($obj, JSON_UNESCAPED_UNICODE), PHP_EOL; // {"name":"laixhe","age":18}
    }

    // 对应 Go TestJsonQuery：数值以字符串形式序列化（string tag）
    public static function jsonQuery(): void
    {
        // PHP 数字默认序列化为 JSON number；如需字符串形式手动转字符串
        $query = [
            'path' => '/index/index',
            'query' => 'name=laixhe&age=18',
            'age' => '18',      // 对应 string tag：以字符串形式序列化
            'score' => '88.8',
            'is_pass' => 'false',
        ];

        $json = json_encode($query, JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES);
        echo $json, PHP_EOL;
        // 结果：{"path":"/index/index","query":"name=laixhe&age=18","age":"18","score":"88.8","is_pass":"false"}
        // 注意：PHP 默认不转义 &，与 Go 的 \u0026 略有差异

        // 反序列化：JSON 中的字符串数字会按需转回数值
        $input = '{"path":"/index/index","query":"name=laixhe&age=19",'
            . '"age":"19","score":"99.99","is_pass":"true"}';
        $query2 = json_decode($input, true); // true = 关联数组
        echo '反序列化 age=', $query2['age'], ' score=', $query2['score'], PHP_EOL;
        // 反序列化 age=19 score=99.99
        var_dump(is_string($query2['age'])); // true（字符串保持字符串，不自动转数值）
    }

    // 对应 Go json.MarshalIndent：美化输出
    public static function jsonPretty(): void
    {
        $query = [
            'path' => '/index/index',
            'query' => 'name=laixhe',
            'age' => 18,
            'score' => 88.8,
            'is_pass' => true,
        ];
        echo json_encode($query, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES), PHP_EOL;
    }

    // 错误处理与常见陷阱
    public static function jsonErrors(): void
    {
        // 序列化失败：资源/非法 UTF-8 等
        $bad = ['data' => "\xB1\x31"]; // 非法 UTF-8 字节
        $result = json_encode($bad);
        if ($result === false) {
            echo 'json_encode 失败: ', json_last_error_msg(), PHP_EOL; // Malformed UTF-8 characters
        }

        // 解析失败：返回 null + 错误码
        $decoded = json_decode('{invalid json', true);
        if ($decoded === null) {
            echo 'json_decode 失败: ', json_last_error_msg(), PHP_EOL; // Syntax error
        }

        // 注意：json_decode 返回 null 也可能是合法 JSON "null"，
        // 严格判断应使用 json_last_error() === JSON_ERROR_NONE
        var_dump(json_decode('null') === null && json_last_error() === JSON_ERROR_NONE); // true（合法 JSON）
    }
}
