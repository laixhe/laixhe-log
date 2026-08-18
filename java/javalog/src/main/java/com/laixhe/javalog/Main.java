package com.laixhe.javalog;

import com.laixhe.javalog.demo.*;

/**
 * JavaLog 主入口：串联所有基础类型、集合、流、时间、并发、JSON、HTTP 示例。
 * 对应 Rust rustlog/src/main.rs（cargo run）与 Go golog 的测试用例（go test -v）。
 */
public class Main {

    public static void main(String[] args) throws Exception {
        // ===== number：数值类型（对应 Rust number.rs）=====
        System.out.println("============= number::number_to_string =============");
        NumberDemo.numberToString();
        System.out.println("============= number::overflow =============");
        NumberDemo.overflow();
        System.out.println("============= number::type_conversion =============");
        NumberDemo.typeConversion();

        // ===== char_string：字符与字符串（对应 Rust char_string.rs）=====
        System.out.println("============= char_string::std_char =============");
        CharStringDemo.stdChar();
        System.out.println("============= char_string::std_string =============");
        CharStringDemo.stdString();
        System.out.println("============= char_string::string_parse =============");
        CharStringDemo.stringParse();

        // ===== collection：集合类型（对应 Rust array_map.rs）=====
        System.out.println("============= collection::std_array =============");
        CollectionDemo.stdArray();
        System.out.println("============= collection::std_tuple =============");
        CollectionDemo.stdTuple();
        System.out.println("============= collection::std_array_list =============");
        CollectionDemo.stdArrayList();
        System.out.println("============= collection::std_array_deque =============");
        CollectionDemo.stdArrayDeque();
        System.out.println("============= collection::std_hash_map =============");
        CollectionDemo.stdHashMap();
        System.out.println("============= collection::std_tree_map =============");
        CollectionDemo.stdTreeMap();
        System.out.println("============= collection::std_hash_set =============");
        CollectionDemo.stdHashSet();
        System.out.println("============= collection::std_tree_set =============");
        CollectionDemo.stdTreeSet();
        System.out.println("============= collection::std_priority_queue =============");
        CollectionDemo.stdPriorityQueue();
        System.out.println("============= collection::std_linked_list =============");
        CollectionDemo.stdLinkedList();
        System.out.println("============= collection::ring_buffer =============");
        CollectionDemo.ringBuffer();
        System.out.println("============= collection::lru_cache =============");
        CollectionDemo.lruCache();
        System.out.println("============= collection::slice_handle =============");
        CollectionDemo.sliceHandle();
        System.out.println("============= collection::map_handle =============");
        CollectionDemo.mapHandle();
        System.out.println("============= collection::unique =============");
        CollectionDemo.unique();
        System.out.println("============= collection::grouping =============");
        CollectionDemo.grouping();

        // ===== stream：流/迭代器专题（对应 Rust iterators.rs）=====
        System.out.println("============= stream::basics =============");
        StreamDemo.basics();
        System.out.println("============= stream::adapters =============");
        StreamDemo.adapters();
        System.out.println("============= stream::consumers =============");
        StreamDemo.consumers();
        System.out.println("============= stream::practice (综合实战) =============");
        StreamDemo.practice();

        // ===== time：时间处理（对应 Go time_test.go）=====
        System.out.println("============= time::time_basics =============");
        TimeDemo.timeBasics();
        System.out.println("============= time::time_parse =============");
        TimeDemo.timeParse();
        System.out.println("============= time::time_before_after =============");
        TimeDemo.timeBeforeAfter();
        System.out.println("============= time::time_since_until =============");
        TimeDemo.timeSinceUntil();
        System.out.println("============= time::time_zone =============");
        TimeDemo.timeZone();

        // ===== sync：并发同步（对应 Go sync_test.go）=====
        System.out.println("============= sync::once =============");
        SyncDemo.once();
        System.out.println("============= sync::wait_group =============");
        SyncDemo.waitGroup();
        System.out.println("============= sync::mutex =============");
        SyncDemo.mutex();
        System.out.println("============= sync::rw_mutex =============");
        SyncDemo.rwMutex();
        System.out.println("============= sync::atomic =============");
        SyncDemo.atomic();

        // ===== json：JSON 序列化（对应 Go json_test.go）=====
        System.out.println("============= json::json_basic =============");
        JsonDemo.jsonBasic();
        System.out.println("============= json::json_query =============");
        JsonDemo.jsonQuery();
        System.out.println("============= json::json_pretty =============");
        JsonDemo.jsonPretty();

        // ===== http：HTTP 服务端与客户端（对应 Go http_serve/http_client）=====
        System.out.println("============= http::http_server + client =============");
        httpDemo();

        System.out.println("============= done =============");
    }

    // HTTP 演示：启动服务端，再访问 /get 与 /post，最后关闭
    private static void httpDemo() throws Exception {
        var server = HttpDemo.startServer();
        int port = server.getAddress().getPort();
        String base = "http://localhost:" + port;
        try {
            System.out.println("HTTP Server 已启动，监听 127.0.0.1:" + port);

            System.out.println("GET / → " + HttpDemo.clientGet(base + "/"));
            System.out.println("GET /get?name=laixhe → " + HttpDemo.clientGet(base + "/get?name=laixhe"));
            System.out.println("POST /post name=laixhe → "
                    + HttpDemo.clientPostForm(base + "/post", java.util.Map.of("name", "laixhe")));
            // 注意：JSON 请求体需要服务端按 JSON 解析，表单解析拿不到 name（此处仅演示）
            System.out.println("POST /post JSON（表单解析） → "
                    + HttpDemo.clientPostJson(base + "/post", "{\"name\":\"laixhe\"}"));
        } finally {
            server.stop(0);
        }
        System.out.println("HTTP Server 已关闭");
    }
}
