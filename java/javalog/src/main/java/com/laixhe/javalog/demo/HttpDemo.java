package com.laixhe.javalog.demo;

import com.sun.net.httpserver.HttpServer;

import java.io.IOException;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.URLDecoder;
import java.net.URLEncoder;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import java.util.LinkedHashMap;
import java.util.Map;

/**
 * HTTP 服务端与客户端示例（JDK 内置，无需第三方库）。
 * 对应 Go golog/http_serve_test.go 与 http_client_test.go。
 *
 * 前置知识：
 * - 服务端：com.sun.net.httpserver.HttpServer（JDK 内置轻量 HTTP 服务器）
 * - 客户端：java.net.http.HttpClient（JDK 11+ 内置 HTTP/1.1、HTTP/2 客户端）
 *   （HTTP/3 需要第三方库，如 quic-go 对应 Java 的 quiche/jetty-http3，本示例略）
 */
public final class HttpDemo {

    private HttpDemo() {
    }

    // ============ HTTP 服务端 ============
    // 对应 Go TestHttpServeRequestResponse：注册 /get /post 路由
    // 端口 0 表示使用系统随机分配的可用端口，避免冲突
    public static HttpServer startServer() throws IOException {
        HttpServer server = HttpServer.create(new InetSocketAddress(0), 0);

        // 根路由（对应 http.HandleFunc("/", ...)）
        server.createContext("/", exchange -> {
            String body = "Hello Java HTTP";
            writeResponse(exchange, 200, body);
        });

        // /get 路由：读取查询参数（对应 r.URL.Query().Get("name")）
        server.createContext("/get", exchange -> {
            String name = parseQuery(exchange.getRequestURI()).getOrDefault("name", "");
            writeResponse(exchange, 200, "http get name=" + name);
        });

        // /post 路由：读取表单参数（对应 r.ParseForm() + r.FormValue("name")）
        server.createContext("/post", exchange -> {
            String requestBody = new String(exchange.getRequestBody().readAllBytes(), StandardCharsets.UTF_8);
            String name = parseForm(requestBody).getOrDefault("name", "");
            writeResponse(exchange, 200, "http post name=" + name);
        });

        server.start();
        return server;
    }

    // 自定义 Handler（对应 Go 的 Handler 机制，可实现中间件/路由系统）
    public static HttpServer startCustomServer() throws IOException {
        HttpServer server = HttpServer.create(new InetSocketAddress(0), 0);
        server.createContext("/", exchange -> writeResponse(exchange, 200, "custom handler"));
        server.start();
        return server;
    }

    // ============ HTTP 客户端 ============
    // 对应 Go TestHttpClientGet：设置 User-Agent / Accept 头
    public static String clientGet(String url) throws IOException, InterruptedException {
        HttpClient client = HttpClient.newBuilder()
                .connectTimeout(Duration.ofSeconds(10))
                .build();

        HttpRequest request = HttpRequest.newBuilder(URI.create(url))
                .header("User-Agent",
                        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
                .header("Accept", "application/json")
                .GET()
                .build();

        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());
        return response.body();
    }

    // 对应 Go TestHttpClientPost：POST 表单 / JSON
    public static String clientPostForm(String url, Map<String, String> form) throws IOException, InterruptedException {
        // 编码表单（对应 url.Values{}.Encode()）
        StringBuilder body = new StringBuilder();
        form.forEach((k, v) -> {
            if (!body.isEmpty()) {
                body.append("&");
            }
            body.append(URLEncoder.encode(k, StandardCharsets.UTF_8))
                    .append("=")
                    .append(URLEncoder.encode(v, StandardCharsets.UTF_8));
        });

        HttpRequest request = HttpRequest.newBuilder(URI.create(url))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .POST(HttpRequest.BodyPublishers.ofString(body.toString()))
                .build();

        HttpResponse<String> response = HttpClient.newHttpClient().send(request, HttpResponse.BodyHandlers.ofString());
        return response.body();
    }

    public static String clientPostJson(String url, String json) throws IOException, InterruptedException {
        HttpRequest request = HttpRequest.newBuilder(URI.create(url))
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .POST(HttpRequest.BodyPublishers.ofString(json))
                .build();

        HttpResponse<String> response = HttpClient.newHttpClient().send(request, HttpResponse.BodyHandlers.ofString());
        return response.body();
    }

    // ============ 工具方法 ============
    private static void writeResponse(com.sun.net.httpserver.HttpExchange exchange, int code, String body)
            throws IOException {
        byte[] bytes = body.getBytes(StandardCharsets.UTF_8);
        exchange.getResponseHeaders().set("Content-Type", "text/plain; charset=utf-8");
        exchange.sendResponseHeaders(code, bytes.length);
        try (OutputStream os = exchange.getResponseBody()) {
            os.write(bytes);
        }
    }

    // 解析 URL 查询参数：?name=laixhe&age=18 -> {name=laixhe, age=18}
    private static Map<String, String> parseQuery(URI uri) {
        String rawQuery = uri.getRawQuery();
        if (rawQuery == null || rawQuery.isEmpty()) {
            return Map.of();
        }
        return parseForm(rawQuery);
    }

    // 解析表单/查询串：name=laixhe&age=18
    private static Map<String, String> parseForm(String raw) {
        Map<String, String> params = new LinkedHashMap<>();
        for (String pair : raw.split("&")) {
            int idx = pair.indexOf('=');
            if (idx < 0) {
                continue;
            }
            String key = URLDecoder.decode(pair.substring(0, idx), StandardCharsets.UTF_8);
            String value = URLDecoder.decode(pair.substring(idx + 1), StandardCharsets.UTF_8);
            params.put(key, value);
        }
        return params;
    }
}
