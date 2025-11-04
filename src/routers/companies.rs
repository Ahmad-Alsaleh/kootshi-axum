use crate::{extractors::JwtToken, models::Company};
use axum::{Json, Router, routing::get};

pub fn get_router() -> Router {
    Router::new().route("/companies", get(get_all_companies))
}

async fn get_all_companies(_: JwtToken) -> Json<Vec<Company>> {
    // TODO: fetch all companies from the db using a state
    Json(vec![
        Company::new(String::from("Al Forsan")),
        Company::new(String::from("Al Joker")),
    ])
}
