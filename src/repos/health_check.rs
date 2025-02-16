use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use crate::error::APIError;

#[tracing::instrument(name = "health check query for database", skip(conn))]
pub async fn health_check(mut conn: PoolConnection<Postgres>) -> Result<(), APIError> {
    let _: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&mut *conn).await?;
    tracing::info!("Checking health check");
    Ok(())
}
