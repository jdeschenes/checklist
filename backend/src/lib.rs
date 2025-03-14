use axum::{
    http::{HeaderName, Request},
    routing::{delete, get, post, put},
    serve::Serve,
    Router,
};
use eyre::Result;
use sqlx::postgres::Postgres;
use sqlx::Pool;

use routes::{
    complete_todo_item, create_todo, create_todo_item, delete_todo, delete_todo_item, get_todo,
    get_todo_item, health_check, list_todo, list_todo_items, update_todo, update_todo_item,
};
use axum::http::Method;
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};
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
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/todo", post(create_todo))
        .route("/todo", get(list_todo))
        .route("/todo/{todo_id}", delete(delete_todo))
        .route("/todo/{todo_id}", get(get_todo))
        .route("/todo/{todo_id}", put(update_todo))
        .route("/todo/{todo_id}/item", post(create_todo_item))
        .route("/todo/{todo_id}/item", get(list_todo_items))
        .route("/todo/{todo_id}/item/{item_id}", get(get_todo_item))
        .route("/todo/{todo_id}/item/{item_id}", put(update_todo_item))
        .route("/todo/{todo_id}/item/{item_id}", delete(delete_todo_item))
        .route(
            "/todo/{todo_id}/item/{item_id}/complete",
            post(complete_todo_item),
        )
        .layer(request_id_middleware)
        .layer(cors)
        .with_state(pg_pool);
    Ok(axum::serve(listener, app))
}
