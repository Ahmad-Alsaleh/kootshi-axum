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
mod secrets;

#[tokio::main]
async fn main() {
    let model_manager = ModelManager::new().await;

    // TODO: modify build-script
    sqlx::migrate!()
        .run(model_manager.db())
        .await
        .expect("failed to apply migrations");

    #[cfg(debug_assertions)]
    model_manager.seed_fake_data().await;

    let app = Router::new().nest("/api/v1", get_app_router(model_manager));
    let listener = TcpListener::bind(&config().server_address)
        .await
        .expect("failed to bind TCP listener");

    println!(
        "Litening at `{}`",
        listener
            .local_addr()
            .expect("failed to get address of TCP listener")
    );
    axum::serve(listener, app)
        .await
        .expect("axum::serve never retruns");
}

fn get_app_router(model_manager: ModelManager) -> Router {
    Router::new()
        .merge(routers::companies::get_router())
        .nest("/auth", routers::auth::get_router())
        .nest("/users", routers::users::get_router())
        .with_state(model_manager)
        .merge(routers::basic::get_router())
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
