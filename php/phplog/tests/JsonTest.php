<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\JsonDemo;
use PHPUnit\Framework\TestCase;

/**
 * JSON 序列化测试（对应 Go json_test.go 的核心断言）。
 */
final class JsonTest extends TestCase
{
    // 练习 1：omitempty —— 空值/空集合忽略（对应 Go TestJson）
    public function testExercise1OmitEmpty(): void
    {
        $tJson = ['time1' => null, 'array1' => [], 'map1' => []];
        $filtered = array_filter($tJson, fn ($v) => $v !== null && $v !== [] && $v !== '');
        $this->assertSame('{}', json_encode((object) $filtered, JSON_UNESCAPED_UNICODE));

        $full = ['time1' => '2025-06-21T09:18:39Z', 'array1' => [1, 2], 'map1' => ['a' => '1']];
        $json = json_encode($full, JSON_UNESCAPED_UNICODE);
        $this->assertStringContainsString('"time1":"2025-06-21T09:18:39Z"', $json);
        $this->assertStringContainsString('"array1":[1,2]', $json);
    }

    // 练习 2：数值以字符串序列化（对应 Go string tag）
    public function testExercise2NumberAsString(): void
    {
        $query = ['age' => '18', 'score' => '88.8', 'is_pass' => 'false'];
        $json = json_encode($query, JSON_UNESCAPED_UNICODE);
        $this->assertStringContainsString('"age":"18"', $json);
        $this->assertStringContainsString('"score":"88.8"', $json);
    }

    // 练习 3：反序列化（对应 Go TestJsonQuery）
    public function testExercise3Deserialize(): void
    {
        $input = '{"path":"/index/index","query":"name=laixhe&age=19",'
            . '"age":"19","score":"99.99","is_pass":"true"}';
        $query = json_decode($input, true);
        $this->assertSame('/index/index', $query['path']);
        $this->assertSame('19', $query['age']);
        $this->assertSame('99.99', $query['score']);
    }

    // 练习 4：错误处理（对应 json_last_error_msg）
    public function testExercise4Errors(): void
    {
        $this->assertFalse(json_encode(['data' => "\xB1\x31"])); // 非法 UTF-8
        $this->assertNotSame(JSON_ERROR_NONE, json_last_error());

        $this->assertNull(json_decode('{invalid json', true));
        $this->assertNotSame(JSON_ERROR_NONE, json_last_error());
    }

    // 练习 5：合法 JSON "null" 与解析失败的区别
    public function testExercise5NullJson(): void
    {
        $decoded = json_decode('null', true);
        $this->assertNull($decoded);
        $this->assertSame(JSON_ERROR_NONE, json_last_error()); // 合法 JSON
    }

    // 运行完整 Demo
    public function testRunJsonDemo(): void
    {
        $this->expectNotToPerformAssertions();
        JsonDemo::jsonBasic();
        JsonDemo::jsonQuery();
        JsonDemo::jsonPretty();
        JsonDemo::jsonErrors();
    }
}
