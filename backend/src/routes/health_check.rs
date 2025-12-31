use eyre::Context;

use crate::error::APIError;
use crate::repos::health_check as health_check_repo;
use crate::tx::tx::Tx;

#[tracing::instrument(name = "Health Check", skip(tx))]
pub async fn health_check(
    mut tx: Tx,
) -> Result<(), APIError> {
    health_check_repo(&mut tx)
        .await
        .context("Failed to perform health check on the repo")?;
    Ok(())
}
