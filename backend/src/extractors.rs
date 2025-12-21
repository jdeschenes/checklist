use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use eyre::WrapErr;

use crate::{
    configuration::RecurringSettings, error::InternalError, repos::UserRepository, AppState,
};

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

impl FromRequestParts<AppState> for DatabaseConnection {
    type Rejection = InternalError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let conn = state
            .pool
            .acquire()
            .await
            .context("acquiring database connection from pool")?;

        Ok(Self(conn))
    }
}

pub struct AppRecurringSettings(pub RecurringSettings);

impl FromRequestParts<AppState> for AppRecurringSettings {
    type Rejection = InternalError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(state.recurring_settings.clone()))
    }
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims = state
            .jwt_service
            .validate_token(auth_header)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_repo = UserRepository::new(state.pool.clone());
        let user_exists = user_repo
            .find_by_email(&claims.email)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        match user_exists {
            Some(user) => Ok(AuthenticatedUser {
                user_id: user.user_id,
            }),
            None => Err(StatusCode::FORBIDDEN),
        }
    }
}
