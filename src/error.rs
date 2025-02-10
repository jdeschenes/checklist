use axum::{http::StatusCode, response::IntoResponse};

pub struct InternalError(eyre::Report);

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

impl<E> From<E> for InternalError
where
    E: Into<eyre::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
