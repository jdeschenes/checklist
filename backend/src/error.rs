use axum::{http::StatusCode, response::IntoResponse};
use eyre::ErrReport;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct InternalError(#[from] eyre::Report);

impl IntoResponse for InternalError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Internal server error: {}: {}", self.0, self.0.root_cause());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
            .into_response()
    }
}

#[derive(Debug, Error)]
pub enum APIError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("already exists error: {0}")]
    AlreadyExists(String),
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("internal server error: {0}")]
    Internal(#[from] InternalError),
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        match self {
            APIError::BadRequest(x) => (StatusCode::BAD_REQUEST, x).into_response(),
            APIError::AlreadyExists(x) => (StatusCode::BAD_REQUEST, x).into_response(),
            APIError::NotFound(x) => (StatusCode::NOT_FOUND, x).into_response(),
            APIError::Internal(x) => x.into_response(),
        }
    }
}

impl From<sqlx::Error> for InternalError {
    fn from(value: sqlx::Error) -> Self {
        InternalError(value.into())
    }
}

impl From<sqlx::Error> for APIError {
    fn from(value: sqlx::Error) -> Self {
        APIError::Internal(value.into())
    }
}

impl From<ErrReport> for APIError {
    fn from(value: ErrReport) -> Self {
        APIError::Internal(InternalError(value))
    }
}
