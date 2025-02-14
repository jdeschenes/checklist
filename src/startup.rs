use std::future::IntoFuture;

use eyre::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;

use crate::configuration::{DatabaseSettings, Settings};
use crate::{run, Server};

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Application> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        tracing::info!("Binding to: {}", address);
        let listener = TcpListener::bind(address)
            .await
            .context("Binding listener")?;
        let port = listener.local_addr().unwrap().port();
        let pool = get_connection_pool(&configuration.database);
        let server = run(listener, pool).await?;
        Ok(Application { server, port })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<()> {
        self.server
            .into_future()
            .await
            .context("Error when serving traffic")
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(configuration.max_connections)
        .acquire_timeout(configuration.pool_acquire_timeout)
        .connect_lazy_with(configuration.connection_options())
}
