use crate::{middlewares::authenticate, models::Company};
use axum::{Json, Router, middleware, routing::get};

pub fn get_router() -> Router {
    Router::new()
        .route("/companies", get(get_all_companies))
        .route_layer(middleware::from_fn(authenticate))
}

async fn get_all_companies() -> Json<Vec<Company>> {
    // TODO: fetch all companies from the db using a state
    Json(vec![
        Company::new(String::from("Al Forsan")),
        Company::new(String::from("Al Joker")),
    ])
}
