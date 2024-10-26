##### 在 Axum 中提供静态文件
```
use tower_http::services::ServeDir;

fn init_router() -> Router {
  Router::new()
    .nest_service("/", ServeDir::new("dist"))
}
```