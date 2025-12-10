use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
}

async fn index() -> impl IntoResponse {
    Html("<h1>Hello <i>World!</i></h1>")
}

// used for health checks
async fn ping() -> impl IntoResponse {
    "pong!"
}
