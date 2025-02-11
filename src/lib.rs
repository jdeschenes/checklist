use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use eyre::Result;
use sqlx::postgres::Postgres;
use sqlx::Pool;

use routes::{create_todo, get_todo, health_check};

pub mod configuration;
mod error;
mod extractors;
mod repos;
mod routes;
pub mod telemetry;
mod types;

pub async fn run(
    listener: tokio::net::TcpListener,
    pg_pool: Pool<Postgres>,
) -> Result<Serve<tokio::net::TcpListener, Router, Router>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/todo", post(create_todo))
        .route("/todo/{todo_id}", get(get_todo))
        .with_state(pg_pool);
    Ok(axum::serve(listener, app))
}
