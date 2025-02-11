use eyre::Context;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use crate::error::InternalError;

pub async fn health_check(mut conn: PoolConnection<Postgres>) -> Result<(), InternalError> {
    let _: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&mut *conn)
        .await
        .context("Running health check query")?;
    tracing::info!("Checking health check");
    Ok(())
}
