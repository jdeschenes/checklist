use axum::http;

#[derive(Debug, thiserror::Error)]
pub enum Error {

    #[error("extractor used multiple time in the same handler/middleware")]
    OverlappingExtractors,

    #[error("extractor used without extension")]
    MissingExtension,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl axum_core::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()  
    }
}
