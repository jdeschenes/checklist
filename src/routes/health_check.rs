use crate::error::InternalError;
use crate::extractors::DatabaseConnection;
use crate::repos::health_check as health_check_repo;

#[tracing::instrument(name = "Health Check", skip(conn))]
pub async fn health_check(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<(), InternalError> {
    health_check_repo(conn).await
}
