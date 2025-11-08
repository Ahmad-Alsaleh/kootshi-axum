use crate::{
    controllers::CompanyController,
    errors::ServerError,
    middlewares::authenticate,
    models::{Company, ModelManager},
};
use axum::{Json, Router, extract::State, middleware, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/companies", get(get_all_companies))
        .route_layer(middleware::from_fn(authenticate))
}

async fn get_all_companies(
    State(model_manager): State<ModelManager>,
) -> Result<Json<Vec<Company>>, ServerError> {
    let companies = CompanyController::get_all(&model_manager)
        .await
        .map_err(|err| ServerError::DataBase(err.to_string()))?;
    Ok(Json(companies))
}
