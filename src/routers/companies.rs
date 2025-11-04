use crate::{extractors::JwtToken, models::Company};
use axum::{Json, Router, routing::get};

pub fn get_router() -> Router {
    Router::new().route("/companies", get(get_all_companies))
}

async fn get_all_companies(jwt_token: JwtToken) -> Json<Vec<Company>> {
    // TODO: ensure the user is authorized (ie logged in)
    dbg!(jwt_token.user_id);

    // TODO: fetch all companies from the db using a state
    Json(vec![
        Company::new(String::from("Al Forsan")),
        Company::new(String::from("Al Joker")),
    ])
}
