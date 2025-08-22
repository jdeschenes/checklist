use axum::{extract::FromRequestParts, http::request::Parts};
use eyre::WrapErr;

use crate::{configuration::RecurringSettings, error::InternalError, AppState};

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
