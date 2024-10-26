use axum::http::Uri;


pub async fn register(uri: Uri) -> String {
    format!("register path={}", uri.path())
}

pub async fn login(uri: Uri) -> String {
    format!("login path={}", uri.path())
}

pub async fn refresh(uri: Uri) -> String {
    format!("refresh path={}", uri.path())
}
