<?php

namespace Laixhe\Phplog;

use RuntimeException;

/**
 * HTTP 客户端示例（cURL 扩展）与测试辅助（proc_open 启动子进程服务器）。
 * 对应 Go golog/http_client_test.go。
 *
 * 服务端为同目录 http_server.php（对应 Go http_serve_test.go），
 * 支持两种运行方式：
 * 1. 手动：终端 1 运行 `php http_server.php`，终端 2 运行客户端
 * 2. 自动：HttpDemo::withServer() 用 proc_open 启动子进程，测试/演示自包含
 */
final class HttpDemo
{
    // ============ 客户端：GET（对应 Go TestHttpClientGet）============
    public static function clientGet(string $url, array $headers = []): string
    {
        $ch = curl_init($url);
        curl_setopt_array($ch, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_TIMEOUT => 10,
            CURLOPT_USERAGENT => 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3',
            CURLOPT_HTTPHEADER => $headers,
        ]);
        $body = curl_exec($ch);
        if ($body === false) {
            throw new RuntimeException('curl GET 失败: ' . curl_error($ch));
        }
        // 注意：PHP 8.0+ 的 cURL 句柄由 GC 自动释放，无需（也不再建议）调用 curl_close()
        return $body;
    }

    // ============ 客户端：POST 表单（对应 Go TestHttpClientPost）============
    public static function clientPostForm(string $url, array $form): string
    {
        $ch = curl_init($url);
        curl_setopt_array($ch, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_TIMEOUT => 10,
            CURLOPT_POST => true,
            CURLOPT_POSTFIELDS => http_build_query($form), // 对应 url.Values{}.Encode()
            CURLOPT_HTTPHEADER => ['Content-Type: application/x-www-form-urlencoded'],
        ]);
        $body = curl_exec($ch);
        if ($body === false) {
            throw new RuntimeException('curl POST 失败: ' . curl_error($ch));
        }
        return $body;
    }

    // ============ 客户端：POST JSON ============
    public static function clientPostJson(string $url, string $json): string
    {
        $ch = curl_init($url);
        curl_setopt_array($ch, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_TIMEOUT => 10,
            CURLOPT_POST => true,
            CURLOPT_POSTFIELDS => $json,
            CURLOPT_HTTPHEADER => ['Content-Type: application/json', 'Accept: application/json'],
        ]);
        $body = curl_exec($ch);
        if ($body === false) {
            throw new RuntimeException('curl POST JSON 失败: ' . curl_error($ch));
        }
        return $body;
    }

    // ============ 测试辅助：启动服务端子进程 ============
    /**
     * 用 proc_open 启动 http_server.php 子进程，返回 [proc, pipes, baseUrl]。
     *
     * @return array{0: resource, 1: array<int, resource>, 2: string}
     */
    public static function startServerProcess(int $port = 0): array
    {
        $serverScript = dirname(__DIR__) . '/http_server.php';
        $cmd = [PHP_BINARY, $serverScript, (string) $port]; // 数组形式命令（PHP 7.4+）

        $proc = proc_open($cmd, [
            0 => ['pipe', 'r'], // stdin
            1 => ['pipe', 'w'], // stdout（首行输出实际端口）
            2 => ['pipe', 'w'], // stderr
        ], $pipes);

        if (!is_resource($proc)) {
            throw new RuntimeException('无法启动 http_server.php 子进程');
        }

        // 读取首行端口地址（带超时，避免服务器启动失败时无限阻塞）
        stream_set_timeout($pipes[1], 5);
        $addrLine = fgets($pipes[1]);
        if ($addrLine === false || !str_contains($addrLine, ':')) {
            $err = stream_get_contents($pipes[2]);
            throw new RuntimeException('读取服务器端口失败: ' . ($err ?: '无输出'));
        }
        $baseUrl = 'http://' . trim($addrLine);

        return [$proc, $pipes, $baseUrl];
    }

    /**
     * 停止服务端子进程。
     *
     * @param array{0: resource, 1: array<int, resource>} $handle
     */
    public static function stopServerProcess(array $handle): void
    {
        [$proc, $pipes] = $handle;
        if (is_resource($proc)) {
            proc_terminate($proc);
            foreach ($pipes as $pipe) {
                if (is_resource($pipe)) {
                    fclose($pipe);
                }
            }
            proc_close($proc);
        }
    }

    /**
     * 自包含运行：启动服务器 → 执行回调 → 停止服务器。
     *
     * @param callable(string $baseUrl): void $fn
     */
    public static function withServer(callable $fn): void
    {
        $handle = self::startServerProcess();
        try {
            $baseUrl = $handle[2];
            $fn($baseUrl);
        } finally {
            self::stopServerProcess($handle);
        }
    }

    // ============ 完整演示（对应 Main 中 http 小节）============
    public static function httpDemo(): void
    {
        self::withServer(function (string $baseUrl): void {
            echo "HTTP Server 已启动，监听 {$baseUrl}", PHP_EOL;

            echo 'GET / → ', self::clientGet($baseUrl . '/'), PHP_EOL;
            echo 'GET /get?name=laixhe → ', self::clientGet($baseUrl . '/get?name=laixhe'), PHP_EOL;
            echo 'POST /post name=laixhe → ',
                self::clientPostForm($baseUrl . '/post', ['name' => 'laixhe']), PHP_EOL;
            echo 'POST /post JSON → ',
                self::clientPostJson($baseUrl . '/post', '{"name":"laixhe"}'), PHP_EOL;
        });
        echo 'HTTP Server 已关闭', PHP_EOL;
    }
}
