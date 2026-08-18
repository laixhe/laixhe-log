import { test, expect } from "bun:test";

// HTTP 服务端与客户端：Bun.serve + fetch。
// 对应 Go golog/http_serve_test.go 与 http_client_test.go。
//
// 前置知识：
// - 服务端：Bun.serve（Bun 内置 HTTP 服务器，对应 Go http.ListenAndServe）
// - 客户端：fetch（Bun/Node 内置，对应 Go http.Client）
// - port: 0 表示使用系统随机分配的可用端口

// 启动一个随机端口的 HTTP 服务，返回 base URL
function startServer(): { base: string; stop: () => void } {
  const server = Bun.serve({
    port: 0,
    fetch(req) {
      const url = new URL(req.url);

      // 根路由（对应 http.HandleFunc("/", ...)）
      if (url.pathname === "/") {
        return new Response("Hello TypeScript HTTP");
      }

      // /get 路由：读取查询参数（对应 r.URL.Query().Get("name")）
      if (url.pathname === "/get") {
        const name = url.searchParams.get("name") ?? "";
        return new Response(`http get name=${name}`);
      }

      // /post 路由：解析表单或 JSON（对应 r.ParseForm() + FormValue）
      if (req.method === "POST" && url.pathname === "/post") {
        return req.text().then((body) => {
          let name = "";
          try {
            name = (JSON.parse(body) as { name?: string }).name ?? "";
          } catch {
            name = new URLSearchParams(body).get("name") ?? "";
          }
          return new Response(`http post name=${name}`);
        });
      }

      return new Response(`404 Not Found: ${url.pathname}`, { status: 404 });
    },
  });

  return {
    base: `http://127.0.0.1:${server.port}`,
    stop: () => server.stop(true),
  };
}

test("HTTP：GET / 与查询参数", async () => {
  const { base, stop } = startServer();
  try {
    // 对应 http.Get / http.NewRequest + Do
    expect(await (await fetch(base + "/")).text()).toBe("Hello TypeScript HTTP");
    expect(await (await fetch(base + "/get?name=laixhe")).text()).toBe("http get name=laixhe");
    expect(await (await fetch(base + "/get")).text()).toBe("http get name=");
  } finally {
    stop();
  }
});

test("HTTP：POST 表单", async () => {
  const { base, stop } = startServer();
  try {
    const resp = await fetch(base + "/post", {
      method: "POST",
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      body: "name=laixhe",
    });
    expect(await resp.text()).toBe("http post name=laixhe");
  } finally {
    stop();
  }
});

test("HTTP：POST JSON", async () => {
  const { base, stop } = startServer();
  try {
    const resp = await fetch(base + "/post", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: "laixhe" }),
    });
    expect(await resp.text()).toBe("http post name=laixhe");
  } finally {
    stop();
  }
});

test("HTTP：404 与状态码", async () => {
  const { base, stop } = startServer();
  try {
    const resp = await fetch(base + "/nope");
    expect(resp.status).toBe(404);
  } finally {
    stop();
  }
});
