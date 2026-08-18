package com.laixhe.javalog;

import com.laixhe.javalog.demo.HttpDemo;
import com.sun.net.httpserver.HttpServer;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * HTTP 服务端与客户端测试（对应 Go http_serve_test.go / http_client_test.go）。
 * 先启动服务端，再通过客户端请求验证。
 */
class HttpTest {

    private static HttpServer server;
    private static String base;

    @BeforeAll
    static void setUp() throws Exception {
        server = HttpDemo.startServer();
        base = "http://localhost:" + server.getAddress().getPort();
    }

    @AfterAll
    static void tearDown() {
        if (server != null) {
            server.stop(0);
        }
    }

    // 根路由（对应 Go TestHttpServe）
    @Test
    void testRoot() throws Exception {
        String body = HttpDemo.clientGet(base + "/");
        assertEquals("Hello Java HTTP", body);
    }

    // GET 查询参数（对应 Go TestHttpServeRequestResponse 的 /get）
    @Test
    void testGetQuery() throws Exception {
        String body = HttpDemo.clientGet(base + "/get?name=laixhe");
        assertEquals("http get name=laixhe", body);
    }

    // POST 表单（对应 /post 的 ParseForm + FormValue）
    @Test
    void testPostForm() throws Exception {
        String body = HttpDemo.clientPostForm(base + "/post", Map.of("name", "laixhe"));
        assertEquals("http post name=laixhe", body);
    }

    // POST JSON
    @Test
    void testPostJson() throws Exception {
        String body = HttpDemo.clientPostJson(base + "/post", "{\"name\":\"laixhe\"}");
        assertTrue(body.startsWith("http post name="));
    }

    // 自定义 Handler（对应 Go TestHttpServeHandler）
    @Test
    void testCustomHandler() throws Exception {
        HttpServer custom = HttpDemo.startCustomServer();
        try {
            String body = HttpDemo.clientGet("http://localhost:" + custom.getAddress().getPort() + "/");
            assertEquals("custom handler", body);
        } finally {
            custom.stop(0);
        }
    }
}
