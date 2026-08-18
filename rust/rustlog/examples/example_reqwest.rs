//! HTTP 客户端示例（reqwest + tokio）。
//!
//! ## 前置知识
//! - **reqwest**：Rust 最流行的 HTTP 客户端库，支持异步、JSON、流式等
//! - 需要 tokio 运行时：`#[tokio::main]`
//! - reqwest 的 `.json()` 方法依赖 serde 反序列化
//!
//! ## ⚠️ 运行前提
//! 本示例需要网络连接，访问公共测试 API：https://jsonplaceholder.typicode.com
//! 无网络时函数会打印错误信息，不会 panic。
//!
//! ## 练习题
//! 1. 用 `reqwest::Client::builder()` 设置 5 秒超时。
//! 2. 发送一个带自定义 Header（如 `Authorization: Bearer xxx`）的请求。
//! 3. 用 `reqwest::get()` 下载一张图片并保存到文件。

use serde::{Deserialize, Serialize};

// 对应 JSONPlaceholder 的 Post 结构
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

// ============ GET 请求 + JSON 反序列化 ============

async fn get_json() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // reqwest::get：便捷方法，内部创建 Client 并发送 GET
    let resp = reqwest::get(url).await?;
    println!("✅ GET {url}");
    println!("   状态码: {}", resp.status());

    // .json()：自动反序列化响应体为指定类型（依赖 serde）
    let post: Post = resp.json().await?;
    println!("   title: {}", post.title);
    println!("   body:  {}...", &post.body[..50]);
    Ok(())
}

// ============ POST 请求 + JSON 序列化 ============

async fn post_json() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts";

    let new_post = Post {
        user_id: 1,
        id:      0,
        title:   "Rust reqwest".to_string(),
        body:    "Hello from reqwest!".to_string(),
    };

    // 用 Client 复用连接池（比每次 get() 更高效）
    let client = reqwest::Client::new();

    // .json()：自动序列化结构体为 JSON 请求体
    let resp = client
        .post(url)
        .json(&new_post)
        .send()
        .await?;

    println!("✅ POST {url}");
    println!("   状态码: {}", resp.status());

    // JSONPlaceholder 会返回创建的 post（带分配的 id）
    let created: Post = resp.json().await?;
    println!("   分配的 id: {}", created.id);
    println!("   title: {}", created.title);
    Ok(())
}

// ============ 错误处理（404 等状态码）============

async fn error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // 故意请求不存在的资源
    let url = "https://jsonplaceholder.typicode.com/posts/999999";
    let resp = reqwest::get(url).await?;

    println!("✅ GET {url}");
    println!("   状态码: {} ({})", resp.status(), resp.status().as_str());

    if resp.status().is_client_error() {
        println!("   → 客户端错误（4xx），资源不存在");
    } else if resp.status().is_server_error() {
        println!("   → 服务端错误（5xx），稍后重试");
    }
    Ok(())
}

// ============ 查询参数 ============

async fn with_query() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let client = reqwest::Client::new();

    // 手动拼接查询参数 ?userId=1
    // 提示：reqwest 0.13 的 query() 方法需要额外 feature，这里用字符串拼接演示
    let full_url = format!("{url}?userId=1");
    let resp = client
        .get(&full_url)
        .send()
        .await?;

    println!("✅ GET {url}?userId=1");
    println!("   状态码: {}", resp.status());

    let posts: Vec<Post> = resp.json().await?;
    println!("   返回 {} 篇帖子", posts.len());
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("============= get_json =============");
    if let Err(e) = get_json().await {
        println!("❌ 请求失败（需要网络）: {e}");
    }

    println!("\n============= post_json =============");
    if let Err(e) = post_json().await {
        println!("❌ 请求失败（需要网络）: {e}");
    }

    println!("\n============= error_handling =============");
    if let Err(e) = error_handling().await {
        println!("❌ 请求失败（需要网络）: {e}");
    }

    println!("\n============= with_query =============");
    if let Err(e) = with_query().await {
        println!("❌ 请求失败（需要网络）: {e}");
    }
}
