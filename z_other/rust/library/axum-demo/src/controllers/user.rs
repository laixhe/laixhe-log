use axum::http::Uri;


pub async fn info(uri: Uri) -> String {
    format!("info path={}", uri.path())
}

pub async fn list(uri: Uri) -> String {
    format!("list path={}", uri.path())
}

pub async fn update(uri: Uri) -> String {
    format!("update path={}", uri.path())
}
