use crate::{
    controllers::{CompanyController, CompanyControllerError},
    errors::ServerError,
    middlewares::authenticate,
    models::{Company, CreateCompanyPayload, ModelManager},
};
use axum::{
    Json, Router, extract::State, http::StatusCode, middleware, response::IntoResponse,
    routing::get,
};
use serde_json::json;

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/companies", get(get_all_companies).post(create_company))
        .route_layer(middleware::from_fn(authenticate))
}

// TODO: (later) add paganation
async fn get_all_companies(
    State(model_manager): State<ModelManager>,
) -> Result<Json<Vec<Company>>, ServerError> {
    let companies = CompanyController::get_all(&model_manager)
        .await
        // TODO: handel all CompanyControllerError cases
        .map_err(|err| ServerError::DataBase(err.to_string()))?;
    Ok(Json(companies))
}

async fn create_company(
    State(model_manager): State<ModelManager>,
    Json(create_company_payload): Json<CreateCompanyPayload>,
) -> Result<impl IntoResponse, ServerError> {
    let result = CompanyController::create(&model_manager, &create_company_payload.name).await;

    let id = match result {
        Ok(id) => id,
        Err(CompanyControllerError::CompanyNameAlreadyExists) => {
            return Err(ServerError::CompanyNameAlreadyExists);
        }
        Err(CompanyControllerError::Sqlx(err)) => {
            return Err(ServerError::DataBase(err.to_string()));
        }
    };

    let response = json!({
        "company_id": id
    });

    Ok((StatusCode::CREATED, Json(response)))
}
