package com.laixhe.javalog.demo;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;

import java.time.Instant;
import java.util.List;
import java.util.Map;

/**
 * JSON 序列化/反序列化示例（Jackson，对应 Go encoding/json 与 Rust serde_json）。
 * 对应 Go golog/json_test.go。
 *
 * 常用注解与 Go Tag 对应关系：
 * - @JsonInclude(NON_EMPTY)  → omitempty：空字符串 "" / 0 / false / null / 空集合 忽略
 * - @JsonInclude(NON_NULL)   → omitzero 部分语义：仅 null 忽略，空集合保留
 * - @JsonSerialize(ToStringSerializer.class) → string：数值以 JSON 字符串序列化
 * - @JsonIgnore              → "-"：忽略该字段
 * - @JsonProperty("xxx")     → json:"xxx"：指定字段名
 */
public final class JsonDemo {

    private JsonDemo() {
    }

    // 全局复用 ObjectMapper（线程安全）。findAndRegisterModules 自动注册时间模块
    // 关闭 WRITE_DATES_AS_TIMESTAMPS：Instant 以 ISO-8601 字符串输出（对应 Go time.Time 的 RFC3339 格式）
    public static final ObjectMapper MAPPER = new ObjectMapper()
            .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)
            .findAndRegisterModules();

    // 对应 Go TJson：演示 omitempty（空值/空集合忽略）
    public record TJson(
            @JsonProperty("time1")
            @JsonInclude(JsonInclude.Include.NON_EMPTY)
            Instant time1,

            @JsonProperty("array1")
            @JsonInclude(JsonInclude.Include.NON_EMPTY)
            List<Integer> array1,

            @JsonProperty("map1")
            @JsonInclude(JsonInclude.Include.NON_EMPTY)
            Map<String, String> map1) {
    }

    // 对应 Go TJson 的 omitzero 变体：NON_NULL 只忽略 null，空集合保留
    public record TJsonOmitZero(
            @JsonProperty("array2")
            @JsonInclude(JsonInclude.Include.NON_NULL)
            List<Integer> array2,

            @JsonProperty("map2")
            @JsonInclude(JsonInclude.Include.NON_NULL)
            Map<String, String> map2) {
    }

    // 对应 Go Query：age/score 以字符串形式序列化（string tag）
    public record Query(
            @JsonProperty("path")
            @JsonInclude(JsonInclude.Include.NON_EMPTY)
            String path,

            @JsonProperty("query")
            @JsonInclude(JsonInclude.Include.NON_EMPTY)
            String query,

            @JsonProperty("age")
            @JsonSerialize(using = ToStringSerializer.class)
            int age,

            @JsonProperty("score")
            @JsonSerialize(using = ToStringSerializer.class)
            double score,

            @JsonProperty("is_pass")
            @JsonSerialize(using = ToStringSerializer.class)
            boolean isPass) {
    }

    // 演示 @JsonIgnore（对应 Go 的 "-" tag）
    public record Secret(
            @JsonProperty("name") String name,
            @JsonIgnore String password) {
    }

    public static void jsonBasic() throws JsonProcessingException {
        // 对应 Go TestJson：空值/空集合被 omitempty 忽略
        TJson tJson = new TJson(null, List.of(), Map.of());
        String s1 = MAPPER.writeValueAsString(tJson);
        System.out.println(s1); // 结果：{}（time1/array1/map1 全部被忽略）

        // omitzero 变体：只忽略 null，空数组/空集合保留
        TJsonOmitZero omitZero = new TJsonOmitZero(List.of(), Map.of());
        String s2 = MAPPER.writeValueAsString(omitZero);
        System.out.println(s2); // 结果：{"array2":[],"map2":{}}

        // 非空值正常序列化
        TJson full = new TJson(Instant.parse("2025-06-21T09:18:39Z"), List.of(1, 2), Map.of("a", "1"));
        System.out.println(MAPPER.writeValueAsString(full)); // 结果：{"time1":"2025-06-21T09:18:39Z",...}

        // @JsonIgnore：忽略敏感字段
        System.out.println(MAPPER.writeValueAsString(new Secret("laixhe", "123456"))); // {"name":"laixhe"}
    }

    public static void jsonQuery() throws JsonProcessingException {
        // 对应 Go TestJsonQuery：数值以字符串形式序列化
        Query query = new Query("/index/index", "name=laixhe&age=18", 18, 88.8, false);
        String json = MAPPER.writeValueAsString(query);
        System.out.println(json);
        // 结果：{"path":"/index/index","query":"name=laixhe&age=18","age":"18","score":"88.8","is_pass":"false"}
        // 注意：Jackson 默认不转义 &，与 Go 的 \u0026 略有差异

        // 反序列化：JSON 中的字符串数字会自动转回数值类型
        String input = "{\"path\":\"/index/index\",\"query\":\"name=laixhe&age=19\","
                + "\"age\":\"19\",\"score\":\"99.99\",\"is_pass\":\"true\"}";
        Query query2 = MAPPER.readValue(input, Query.class);
        System.out.println(query2);
        // 结果：Query[path=/index/index, query=name=laixhe&age=19, age=19, score=99.99, isPass=true]
    }

    public static void jsonPretty() throws JsonProcessingException {
        // 美化输出（对应 json.MarshalIndent）
        Query query = new Query("/index/index", "name=laixhe", 18, 88.8, true);
        String pretty = MAPPER.writerWithDefaultPrettyPrinter().writeValueAsString(query);
        System.out.println(pretty);
    }
}
