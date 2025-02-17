use axum::{
    http::{HeaderName, Request},
    routing::{get, post, put},
    serve::Serve,
    Router,
};
use eyre::Result;
use sqlx::postgres::Postgres;
use sqlx::Pool;

use routes::{create_todo, get_todo, health_check, list_todo, update_todo};
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info_span};

pub mod configuration;
mod domain;
mod error;
mod extractors;
mod repos;
mod routes;
pub mod startup;
pub mod telemetry;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub type Server = Serve<tokio::net::TcpListener, Router, Router>;

pub async fn run(listener: tokio::net::TcpListener, pg_pool: Pool<Postgres>) -> Result<Server> {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);
    let request_id_middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = request.headers().get(REQUEST_ID_HEADER);
                match request_id {
                    Some(request_id) => {
                        info_span!(
                            "http_request",
                            request_id = ?request_id,
                        )
                    }
                    None => {
                        error!("could not extract request_id");
                        info_span!("http_request")
                    }
                }
            }),
        )
        .layer(PropagateRequestIdLayer::new(x_request_id));
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/todo", post(create_todo))
        .route("/todo", get(list_todo))
        .route("/todo/{todo_id}", get(get_todo))
        .route("/todo/{todo_id}", put(update_todo))
        .layer(request_id_middleware)
        .with_state(pg_pool);
    Ok(axum::serve(listener, app))
}
