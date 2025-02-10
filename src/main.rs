use std::future::IntoFuture;
use eyre::{Context, Result};
use sqlx::postgres::PgPoolOptions;
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
    // Configure the tracing subscriber
    // Tracing color-eyre
    // Get the pool up and running
    let db_connection_str = configuration.database.to_connection_string(); 
    let pool = PgPoolOptions::new()
        .max_connections(configuration.database.max_connections)
        .acquire_timeout(configuration.database.pool_acquire_timeout)
        .connect(&db_connection_str)
        .await?;

    tracing::info!("Binding to: {}", address);
    let listener = TcpListener::bind(address).await.context("Binding listener")?;

    let server = run(listener, pool).await.context("Getting server")?;
    server.into_future().await.context("Serving traffic")?;
    Ok(())
}
