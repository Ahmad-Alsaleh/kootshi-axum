use axum::{Router, middleware};
use tokio::net::TcpListener;

mod errors;
mod middlewares;
mod models;
mod routers;

#[tokio::main]
async fn main() {
    // TODO: get the host address from env var with default of 127.0.0.1:1936
    let app = get_app_router();
    let listener = TcpListener::bind("127.0.0.1:1936").await.unwrap();

    println!("Litening at `{}`", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn get_app_router() -> Router {
    Router::new()
        .merge(routers::basic::get_router())
        .nest("/auth", routers::auth::get_router())
        .layer(middleware::map_response(middlewares::map_response))
}
