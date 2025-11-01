use axum::{
    Router,
    http::{Method, StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::get,
};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .fallback(fallback)
}

async fn index() -> impl IntoResponse {
    Html("<h1>Hello <i>World!</i></h1>")
}

// used for health checks
async fn ping() -> impl IntoResponse {
    "pong!"
}

async fn fallback(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("the specified endpoint `{method} {uri}` is not found."),
    )
}
