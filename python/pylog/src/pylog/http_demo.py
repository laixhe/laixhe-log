"""主题：HTTP 服务端与客户端（http.server + urllib.request 标准库）。

对应 Go golog/http_serve_test.go 与 http_client_test.go。

前置知识：
- 服务端：http.server.ThreadingHTTPServer + BaseHTTPRequestHandler（轻量 HTTP 服务器）
- 客户端：urllib.request.urlopen / Request（对应 Go http.Client）
- 与 PHP 不同，Python 有真实的多线程：服务端在后台线程运行，测试自包含
"""

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlparse
from urllib.request import Request, urlopen


class Handler(BaseHTTPRequestHandler):
    """路由处理（对应 Go TestHttpServeRequestResponse 的 HandleFunc）。"""

    def log_message(self, format, *args):  # 屏蔽默认访问日志
        pass

    def _send(self, body: str) -> None:
        payload = body.encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "text/plain; charset=utf-8")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)

    def do_GET(self):
        # 对应 r.URL.Query().Get("name")
        parsed = urlparse(self.path)
        if parsed.path == "/":
            self._send("Hello Python HTTP")
        elif parsed.path == "/get":
            name = parse_qs(parsed.query).get("name", [""])[0]
            self._send(f"http get name={name}")
        else:
            self._send(f"404 Not Found: {parsed.path}")

    def do_POST(self):
        # 对应 r.ParseForm() + r.FormValue("name")
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8")
        # 兼容表单（name=laixhe）与 JSON（{"name":"laixhe"}）
        try:
            params = json.loads(body)
        except json.JSONDecodeError:
            params = parse_qs(body)
        name = params.get("name") if isinstance(params, dict) else ""
        if isinstance(name, list):
            name = name[0]
        self._send(f"http post name={name}")


def start_server() -> ThreadingHTTPServer:
    """启动一个随机端口的 HTTP 服务（对应 Go http.ListenAndServe）。"""
    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server


# ============ 客户端（对应 Go http_client_test.go）============
def client_get(url: str) -> str:
    req = Request(url, headers={
        "User-Agent": ("Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
                       "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"),
        "Accept": "application/json",
    })
    with urlopen(req, timeout=10) as resp:
        return resp.read().decode("utf-8")


def client_post_form(url: str, form: dict) -> str:
    # 对应 url.Values{}.Encode()：表单编码
    body = "&".join(f"{k}={v}" for k, v in form.items())
    req = Request(url, data=body.encode("utf-8"), headers={
        "Content-Type": "application/x-www-form-urlencoded",
    })
    with urlopen(req, timeout=10) as resp:
        return resp.read().decode("utf-8")


def client_post_json(url: str, payload: str) -> str:
    req = Request(url, data=payload.encode("utf-8"), headers={
        "Content-Type": "application/json",
        "Accept": "application/json",
    })
    with urlopen(req, timeout=10) as resp:
        return resp.read().decode("utf-8")


def run() -> None:
    print("========== HTTP 服务与客户端 ==========")

    # 启动服务端（对应 Go TestHttpServe，后台线程运行）
    server = start_server()
    base = f"http://127.0.0.1:{server.server_address[1]}"
    try:
        print(f"HTTP Server 已启动，监听 {base}")

        print("GET / →", client_get(base + "/"))
        print("GET /get?name=laixhe →", client_get(base + "/get?name=laixhe"))
        print("POST /post name=laixhe →", client_post_form(base + "/post", {"name": "laixhe"}))
        print("POST /post JSON →", client_post_json(base + "/post", '{"name":"laixhe"}'))
    finally:
        server.shutdown()
        server.server_close()
    print("HTTP Server 已关闭")
