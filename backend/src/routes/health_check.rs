use eyre::Context;
use sqlx::Acquire;

use crate::error::APIError;
use crate::extractors::DatabaseConnection;
use crate::repos::health_check as health_check_repo;

#[tracing::instrument(name = "Health Check", skip(conn))]
pub async fn health_check(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<(), APIError> {
    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    health_check_repo(&mut transaction)
        .await
        .context("Failed to perform health check on the repo")?;
    transaction
        .rollback()
        .await
        .context("Unable to rollback transaction")?;
    Ok(())
}
