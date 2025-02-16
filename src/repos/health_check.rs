use sqlx::PgTransaction;

use crate::error::APIError;

#[tracing::instrument(name = "health check query for database", skip(transaction))]
pub async fn health_check(transaction: &mut PgTransaction<'_>) -> Result<(), APIError> {
    let _: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&mut **transaction)
        .await?;
    Ok(())
}
