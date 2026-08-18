package com.laixhe.javalog;

import com.laixhe.javalog.demo.JsonDemo;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/**
 * JSON 序列化测试（对应 Go json_test.go 的核心断言）。
 */
class JsonTest {

    // 练习 1：omitempty —— 空值/空集合忽略（对应 Go TestJson）
    @Test
    void exercise1_omitempty() throws Exception {
        JsonDemo.TJson tJson = new JsonDemo.TJson(null, List.of(), Map.of());
        String json = JsonDemo.MAPPER.writeValueAsString(tJson);
        assertEquals("{}", json); // 全部被忽略

        JsonDemo.TJson full = new JsonDemo.TJson(
                java.time.Instant.parse("2025-06-21T09:18:39Z"), List.of(1, 2), Map.of("a", "1"));
        String jsonFull = JsonDemo.MAPPER.writeValueAsString(full);
        assertTrue(jsonFull.contains("\"time1\":\"2025-06-21T09:18:39Z\""));
        assertTrue(jsonFull.contains("\"array1\":[1,2]"));
    }

    // 练习 2：omitzero 变体 —— 只忽略 null，空集合保留
    @Test
    void exercise2_omitzero() throws Exception {
        JsonDemo.TJsonOmitZero omitZero = new JsonDemo.TJsonOmitZero(List.of(), Map.of());
        String json = JsonDemo.MAPPER.writeValueAsString(omitZero);
        assertTrue(json.contains("\"array2\":[]"));
        assertTrue(json.contains("\"map2\":{}"));
    }

    // 练习 3：string tag —— 数值以字符串序列化（对应 Go TestJsonQuery）
    @Test
    void exercise3_number_as_string() throws Exception {
        JsonDemo.Query query = new JsonDemo.Query("/index/index", "name=laixhe", 18, 88.8, false);
        String json = JsonDemo.MAPPER.writeValueAsString(query);
        assertTrue(json.contains("\"age\":\"18\""));
        assertTrue(json.contains("\"score\":\"88.8\""));
        assertTrue(json.contains("\"is_pass\":\"false\""));
    }

    // 练习 4：反序列化 —— 字符串数字自动转回数值类型
    @Test
    void exercise4_deserialize() throws Exception {
        String input = "{\"path\":\"/index/index\",\"query\":\"name=laixhe&age=19\","
                + "\"age\":\"19\",\"score\":\"99.99\",\"is_pass\":\"true\"}";
        JsonDemo.Query query = JsonDemo.MAPPER.readValue(input, JsonDemo.Query.class);
        assertEquals("/index/index", query.path());
        assertEquals(19, query.age());
        assertEquals(99.99, query.score());
        assertTrue(query.isPass());
    }

    // 练习 5：@JsonIgnore 忽略敏感字段（对应 Go "-" tag）
    @Test
    void exercise5_ignore() throws Exception {
        JsonDemo.Secret secret = new JsonDemo.Secret("laixhe", "123456");
        String json = JsonDemo.MAPPER.writeValueAsString(secret);
        assertEquals("{\"name\":\"laixhe\"}", json);
        assertFalse(json.contains("123456"));
    }

    // 运行完整 Demo
    @Test
    void runJsonDemo() throws Exception {
        JsonDemo.jsonBasic();
        JsonDemo.jsonQuery();
        JsonDemo.jsonPretty();
    }
}
