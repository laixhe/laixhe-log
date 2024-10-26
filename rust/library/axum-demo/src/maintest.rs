use std::collections::HashMap;
// use std::time::Duration;

use axum::{
    extract::{Form, Path, Request},
    http::{header, StatusCode, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format::FmtSpan;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

// 中间件
async fn middleware_is_json(req: Request, next: Next) -> Result<Response, StatusCode> {
    // 需要 http crate 来获取 header 名称
    if req.headers().get(header::CONTENT_TYPE).unwrap() != "application/json" {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(next.run(req).await)
}

// 路由构建
fn app_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/get-html", get(get_html))
        .route("/get-path-info/:name", get(get_path_info))
        .route("/post-json", post(post_json)).route_layer(middleware::from_fn(middleware_is_json))
        .route("/post-form", post(post_form))
}

// 路由构建
fn app_router_mysql(mysql_pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/get-html", get(get_html))
        .route("/get-path-info/:name", get(get_path_info))
        .route("/post-json", post(post_json)).route_layer(middleware::from_fn(middleware_is_json))
        .route("/post-form", post(post_form))
        // .route("/post-sql", post(insert_user))
        .with_state(mysql_pool)
}

async fn main11() {
    // 初始化跟踪日志
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // let mysql_pool = MySqlPoolOptions::new()
    //     .max_connections(5)
    //     .connect("mysql://root:123456@127.0.0.1:3306/webapi")
    //     .await
    //     .expect("数据库连接失败");

    tracing::info!("开始监听 http 端口: {}", "0.0.0.0:3000");
    // 要绑定的 IP 和 端口
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // 启动 http
    axum::serve(listener, app_router()).await.unwrap();
    // axum::serve(listener, app_router_mysql(mysql_pool)).await.unwrap();
}

async fn index(uri: Uri) -> String {
    // http://127.0.0.1:3000
    // content-type: text/plain

    format!("index path={}", uri.path())
}

async fn get_html(uri: Uri) -> Html<String> {
    // http://127.0.0.1:3000/get-html
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
    // {"name":"laixhe","age": 18}

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
    // name=laixhe&age=18

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

pub async fn insert_user(mysql_pool: &MySqlPool) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::error::Error> {

    let sql = String::from("INSERT INTO `user` (`password`,`email`,`uname`,`age`,`score`,`login_at`) VALUES (?,?,?,?,?,?)");

    let result = sqlx::query(sql.as_str())
        .bind("$2y$10$nAwKmE9MJx9y.ouzY0R/Puqr6u.6nHa6Rt2MS28w0Gs5agIaZeAP6")
        .bind("laixhe@laixhe.com")
        .bind("laixhe")
        .bind(18)
        .bind(0)
        .bind("2024-06-21 21:21:21")
        .execute(mysql_pool).await?;

        Ok(result)
}