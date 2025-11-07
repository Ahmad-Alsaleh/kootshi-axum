// TODOs:
// 1. replace unwraps with proper error handeling
// 2. seed the dev db

use crate::{configs::config, models::ModelManager};
use axum::{
    Router,
    http::{Method, StatusCode, Uri},
    middleware,
    response::IntoResponse,
};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod configs;
mod controllers;
mod errors;
mod extractors;
mod middlewares;
mod models;
mod routers;

// #[cfg(debug_assertions)]
// mod dev_db;

#[tokio::main]
async fn main() {
    let model_manager = ModelManager::new().await;

    let app = get_app_router(model_manager);
    let listener = TcpListener::bind(&config().server_address)
        .await
        .expect("failed to bind TCP listener");

    println!("Litening at `{}`", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn get_app_router(model_manager: ModelManager) -> Router {
    Router::new()
        .merge(routers::companies::get_router())
        .with_state(model_manager)
        .merge(routers::basic::get_router())
        .nest("/auth", routers::auth::get_router())
        .layer(middleware::map_response(middlewares::map_response))
        .layer(middleware::map_response(middlewares::log_response))
        .layer(middleware::map_request(middlewares::generate_request_id))
        .layer(CookieManagerLayer::new())
        .fallback(fallback)
}

async fn fallback(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("The specified endpoint `{method} {uri}` is not found."),
    )
}
