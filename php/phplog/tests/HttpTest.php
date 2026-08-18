<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\HttpDemo;
use PHPUnit\Framework\TestCase;

/**
 * HTTP 服务端与客户端测试（对应 Go http_serve_test.go / http_client_test.go）。
 * 通过 proc_open 启动 http_server.php 子进程，验证 GET / POST / JSON。
 */
final class HttpTest extends TestCase
{
    // GET 根路由（对应 Go TestHttpServe）
    public function testRoot(): void
    {
        HttpDemo::withServer(function (string $baseUrl): void {
            $this->assertSame('Hello PHP HTTP', HttpDemo::clientGet($baseUrl . '/'));
        });
    }

    // GET 查询参数（对应 /get 的 r.URL.Query().Get("name")）
    public function testGetQuery(): void
    {
        HttpDemo::withServer(function (string $baseUrl): void {
            $this->assertSame('http get name=laixhe', HttpDemo::clientGet($baseUrl . '/get?name=laixhe'));
        });
    }

    // POST 表单（对应 /post 的 ParseForm + FormValue）
    public function testPostForm(): void
    {
        HttpDemo::withServer(function (string $baseUrl): void {
            $this->assertSame('http post name=laixhe', HttpDemo::clientPostForm($baseUrl . '/post', ['name' => 'laixhe']));
        });
    }

    // POST JSON
    public function testPostJson(): void
    {
        HttpDemo::withServer(function (string $baseUrl): void {
            $this->assertSame('http post name=laixhe', HttpDemo::clientPostJson($baseUrl . '/post', '{"name":"laixhe"}'));
        });
    }

    // 指定端口启动（默认随机端口）
    public function testStartWithFixedPort(): void
    {
        $handle = HttpDemo::startServerProcess(18080);
        try {
            $baseUrl = $handle[2];
            $this->assertSame('Hello PHP HTTP', HttpDemo::clientGet($baseUrl . '/'));
        } finally {
            HttpDemo::stopServerProcess($handle);
        }
    }
}
