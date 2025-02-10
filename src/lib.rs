use axum::{routing::{get, post}, serve::Serve, Router};
use eyre::Result;
use sqlx::Pool;
use sqlx::postgres::Postgres;

use routes::{create_todo, health_check};

pub mod telemetry;
pub mod configuration;
mod error;
mod extractors;
mod repos;
mod routes;
mod types;


pub async fn run(
    listener: tokio::net::TcpListener,
    pg_pool: Pool<Postgres>,
) -> Result<Serve<tokio::net::TcpListener, Router, Router>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/todo", post(create_todo))
        .with_state(pg_pool);
    Ok(axum::serve(listener, app))
}

