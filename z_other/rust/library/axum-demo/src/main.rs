use std::collections::HashMap;
use std::net::SocketAddr;
// use std::time::Duration;

use axum::{
    extract::{Form, Path},
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

mod db_user;

// 路由构建
fn app_router(mysql_pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/get-html", get(get_html))
        .route("/get-path-info/:name", get(get_path_info))
        .route("/post-json", post(post_json))
        .route("/post-form", post(post_form))
        .with_state(mysql_pool)
}

#[tokio::main]
async fn main() {
    // 初始化跟踪日志
    tracing_subscriber::fmt::init();

    // 连接数据库
    //let mysql_pool = MySqlPool::connect("mysql://root:123456@127.0.0.1:3306/test").await.expect("数据库连接失败");
    let mysql_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:123456@127.0.0.1:3306/test")
        .await
        .expect("数据库连接失败");

    // 要绑定的 IP 和 端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("开始监听 http 端口: {}", addr);

    // 启动 http (hyper)
    axum::Server::bind(&addr)
        .serve(app_router(mysql_pool).into_make_service())
        .await
        .unwrap();
}

async fn index(uri: Uri) -> String {
    // http://127.0.0.1:3000
    // content-type: text/plain

    format!("index path={}", uri.path())
}

async fn get_html(uri: Uri) -> Html<String> {
    // http://127.0.0.1:3000
    // content-type: text/html

    Html(format!("<h1>get html path={}</h1>", uri.path()))
}

// 路径参数(path info) 返回：JSON
async fn get_path_info(uri: Uri, Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    // http://127.0.0.1:3000/get-path-info/laiki
    // content-type: application/json

    // 判断是否有这个 key
    if params.contains_key("name") {
        tracing::info!("路径存在 name 这个键")
    }

    let user = User {
        id: 1001,
        name: params["name"].clone(),
        age: 18,
        text: format!("路径={}", uri.path()),
    };

    (StatusCode::OK, Json(user))
}

async fn post_json(Json(params): Json<PostRequest>) -> impl IntoResponse {
    // http://127.0.0.1:3000/post-json
    // content-type: application/json
    // {"name":"laiki","age": 18}

    let user = User {
        id: 1,
        name: params.name,
        age: params.age,
        text: "".to_string(),
    };

    (StatusCode::OK, Json(user))
}

async fn post_form(Form(params): Form<PostRequest>) -> impl IntoResponse {
    // http://127.0.0.1:3000/post-form
    // content-type: application/x-www-form-urlencoded
    // name=laiki&age=18

    let user = User {
        id: 2,
        name: params.name,
        age: params.age,
        text: "".to_string(),
    };

    (StatusCode::OK, Json(user))
}

#[derive(Deserialize)]
struct PostRequest {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    age: i32,
    text: String,
}
