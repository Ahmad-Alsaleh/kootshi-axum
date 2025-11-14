use crate::{
    controllers::CompanyController,
    errors::ServerError,
    middlewares::authenticate,
    models::{Company, CreateCompanyPayload, ModelManager},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get},
};
use serde_json::json;

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/companies", get(get_all_companies).post(create_company))
        .route("/companies/{company_name}", delete(delete_company_by_name))
        .route_layer(middleware::from_fn(authenticate))
}

// TODO: (later) add paganation
async fn get_all_companies(
    State(model_manager): State<ModelManager>,
) -> Result<Json<Vec<Company>>, ServerError> {
    let companies = CompanyController::get_all(&model_manager).await?;
    Ok(Json(companies))
}

async fn create_company(
    State(model_manager): State<ModelManager>,
    Json(create_company_payload): Json<CreateCompanyPayload>,
) -> Result<impl IntoResponse, ServerError> {
    let id = CompanyController::create(&model_manager, &create_company_payload.name).await?;

    let response = json!({
        "company_id": id
    });

    Ok((StatusCode::CREATED, Json(response)))
}

async fn delete_company_by_name(
    State(model_manager): State<ModelManager>,
    Path(company_name): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    CompanyController::delete_by_name(&model_manager, &company_name).await?;
    Ok(StatusCode::NO_CONTENT)
}
