use crate::models::Company;
use axum::{Json, Router, routing::get};

pub fn get_router() -> Router {
    Router::new().route("/companies", get(get_all_companies))
}

async fn get_all_companies() -> Json<Vec<Company>> {
    // TODO: ensure the user is authorized (ie logged in)
    // TODO: fetch all companies from the db using a state
    Json(vec![
        Company::new(String::from("Al Forsan")),
        Company::new(String::from("Al Joker")),
    ])
}
