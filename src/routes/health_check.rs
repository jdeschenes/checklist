use eyre::Context;

use crate::error::APIError;
use crate::extractors::DatabaseConnection;
use crate::repos::health_check as health_check_repo;

#[tracing::instrument(name = "Health Check", skip(conn))]
pub async fn health_check(DatabaseConnection(conn): DatabaseConnection) -> Result<(), APIError> {
    health_check_repo(conn)
        .await
        .context("Failed to perform health check on the repo")?;
    Ok(())
}
