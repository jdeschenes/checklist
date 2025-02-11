use eyre::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use std::future::IntoFuture;
use tokio::net::TcpListener;

use checklist::telemetry::{get_subscriber, init_subscriber};
use checklist::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = get_subscriber("checklist".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber)?;
    let configuration = get_configuration().context("Getting configuration")?;
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let pool = PgPoolOptions::new()
        .max_connections(configuration.database.max_connections)
        .acquire_timeout(configuration.database.pool_acquire_timeout)
        .connect_lazy_with(configuration.database.connection_options());

    tracing::info!("Binding to: {}", address);
    let listener = TcpListener::bind(address)
        .await
        .context("Binding listener")?;

    let server = run(listener, pool).await.context("Getting server")?;
    server.into_future().await.context("Serving traffic")?;
    Ok(())
}
