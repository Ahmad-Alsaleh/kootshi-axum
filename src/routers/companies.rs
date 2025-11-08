use crate::{
    controllers::CompanyController,
    middlewares::authenticate,
    models::{Company, ModelManager},
};
use axum::{Json, Router, extract::State, middleware, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/companies", get(get_all_companies))
        .route_layer(middleware::from_fn(authenticate))
}

async fn get_all_companies(State(model_manager): State<ModelManager>) -> Json<Vec<Company>> {
    // // TODO: replace unwrap with .map_err(|err| ServerError::DataBase(err))
    let companies = CompanyController::get_all(&model_manager).await.unwrap();
    Json(companies)
}
