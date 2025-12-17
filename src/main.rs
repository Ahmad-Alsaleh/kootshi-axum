use crate::{configs::config, models::ModelManager};
use axum::{
    Router,
    http::{Method, StatusCode, Uri},
    middleware,
    response::IntoResponse,
    routing::get,
};
use tokio::{net::TcpListener, signal};
use tower_cookies::CookieManagerLayer;

mod configs;
mod controllers;
mod errors;
mod extractors;
mod logging;
mod middlewares;
mod models;
mod routers;
mod secrets;

#[tokio::main]
async fn main() {
    let model_manager = ModelManager::new().await;

    sqlx::migrate!()
        .run(model_manager.db())
        .await
        .expect("failed to apply migrations");

    #[cfg(debug_assertions)]
    model_manager.seed_fake_data().await;

    let app = Router::new().nest("/api/v1", get_app_router(model_manager.clone()));
    let listener = TcpListener::bind(&config().server_address)
        .await
        .expect("failed to bind TCP listener");

    // TODO: use proper logging
    println!(
        "Litening at `{}`",
        listener
            .local_addr()
            .expect("failed to get address of TCP listener")
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(model_manager))
        .await
        .expect("axum::serve never retruns");
}

async fn shutdown_signal(
    #[cfg_attr(not(debug_assertions), allow(unused))] model_manager: ModelManager,
) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    #[cfg(debug_assertions)]
    {
        model_manager.unseed_fake_data().await;
        println!("unseeded the data base");
    }

    println!("Exiting...");
}

fn get_app_router(model_manager: ModelManager) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .nest("/auth", routers::auth::get_router())
        .nest("/users", routers::users::get_router())
        .with_state(model_manager)
        .layer(middleware::map_response(
            middlewares::insert_response_body_on_error,
        ))
        .layer(middleware::map_response(middlewares::log_response))
        .layer(middleware::map_request(middlewares::generate_request_id))
        .layer(CookieManagerLayer::new())
        .fallback(fallback)
}

/// used for health checks
async fn ping() -> impl IntoResponse {
    "pong!"
}

async fn fallback(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("The specified endpoint `{method} {uri}` is not found."),
    )
}
