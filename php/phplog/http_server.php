<?php

/**
 * 简易 HTTP 服务器（对应 Go http_serve_test.go 的 http.ListenAndServe）。
 *
 * 运行方式（两个终端，对应 Rust TCP 示例的两终端模式）：
 *   终端 1：php http_server.php [port]     # 不传 port 则使用随机空闲端口
 *   终端 2：php run.php                     # 或直接访问 http://127.0.0.1:PORT
 *
 * 也可以被 HttpDemo::withServer() 作为子进程启动（proc_open），
 * 输出首行 "127.0.0.1:PORT" 便于外部获取实际端口。
 */

declare(strict_types=1);

$port = isset($argv[1]) ? (int) $argv[1] : 0;
$address = "tcp://127.0.0.1:{$port}";

$server = stream_socket_server($address, $errno, $errstr);
if ($server === false) {
    fwrite(STDERR, "监听失败: {$errstr} ({$errno})" . PHP_EOL);
    exit(1);
}

// 获取实际绑定地址（port=0 时由系统分配）
$actual = stream_socket_get_name($server, false);
fwrite(STDOUT, $actual . PHP_EOL);
flush();

echo "HTTP Server 已启动，监听 {$actual}" . PHP_EOL;

while (true) {
    $conn = @stream_socket_accept($server, -1);
    if ($conn === false) {
        continue;
    }

    // ---- 读取请求头（直到空行）----
    $request = '';
    while (!str_contains($request, "\r\n\r\n")) {
        $chunk = fread($conn, 8192);
        if ($chunk === false || $chunk === '') {
            break;
        }
        $request .= $chunk;
    }
    [$head, $body] = array_pad(explode("\r\n\r\n", $request, 2), 2, '');

    // ---- 解析请求行：GET /get?name=laixhe HTTP/1.1 ----
    $lines = explode("\r\n", $head);
    $requestLine = $lines[0] ?? '';
    preg_match('#^(\S+)\s+(\S+)(?:\s+HTTP/(\d\.\d))?#', $requestLine, $m);
    $method = $m[1] ?? 'GET';
    $target = $m[2] ?? '/';

    // ---- 按 Content-Length 读取请求体 ----
    foreach ($lines as $line) {
        if (stripos($line, 'Content-Length:') === 0) {
            $contentLength = (int) trim(substr($line, 15));
            while (strlen($body) < $contentLength) {
                $chunk = fread($conn, 8192);
                if ($chunk === false || $chunk === '') {
                    break;
                }
                $body .= $chunk;
            }
            break;
        }
    }

    // ---- 路由处理（对应 Go TestHttpServeRequestResponse）----
    $path = parse_url($target, PHP_URL_PATH) ?: '/';
    $query = parse_url($target, PHP_URL_QUERY) ?? '';
    $params = [];
    parse_str($query, $params);

    switch ($path) {
        case '/':
            $respBody = 'Hello PHP HTTP';
            break;
        case '/get':
            $name = $params['name'] ?? '';
            $respBody = "http get name={$name}";
            break;
        case '/post':
            // 表单（name=laixhe）或 JSON（{"name":"laixhe"}）都尝试解析
            $post = json_decode($body, true);
            if (!is_array($post)) {
                parse_str($body, $post);
            }
            $name = $post['name'] ?? '';
            $respBody = "http post name={$name}";
            break;
        default:
            $respBody = "404 Not Found: {$path}";
    }

    // ---- 响应（对应 w.Header().Set + w.Write）----
    $payload = $respBody;
    $response = "HTTP/1.1 200 OK\r\n"
        . "Content-Type: text/plain; charset=utf-8\r\n"
        . 'Content-Length: ' . strlen($payload) . "\r\n"
        . "Connection: close\r\n"
        . "\r\n"
        . $payload;

    fwrite($conn, $response);
    fclose($conn);
}
