use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use axum::extract;
use axum::Json;

use crate::error::InternalError;
use crate::extractors::DatabaseConnection;
use crate::repos::{create_todo as create_todo_repos, get_todo_by_name};
use crate::types::{CreateListRequest, CreateListResponse, GetListResponse};

#[tracing::instrument(
    name = "Create TODO"
    skip(conn, payload),
)]
pub async fn create_todo(
    DatabaseConnection(conn): DatabaseConnection,
    ValidatedJson(payload): ValidatedJson<CreateListRequest>,
) -> Result<Json<CreateListResponse>, InternalError> {
    create_todo_repos(conn, payload).await.map(Json)
}

#[tracing::instrument(
    name = "Get TODO"
    skip(conn),
)]
pub async fn get_todo(
    DatabaseConnection(conn): DatabaseConnection,
    extract::Path(todo_name): extract::Path<String>,
) -> Result<Json<GetListResponse>, InternalError> {
    get_todo_by_name(conn, &todo_name).await.map(Json)
}

#[derive(Debug, Clone)]
pub struct ValidatedJson<T>(pub T);

#[derive(Debug, Error)]
pub enum JsonError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

impl IntoResponse for JsonError {
    fn into_response(self) -> axum::response::Response {
        match self {
            JsonError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            JsonError::JsonRejection(json_error) => json_error.into_response(),
        }
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = JsonError;

    async fn from_request(req: extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}
