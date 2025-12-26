use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    http::{HeaderName, Request, StatusCode},
    routing::{delete, get, post, put},
    serve::Serve,
    BoxError, Router,
};
use eyre::Result;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::time::Duration;

use crate::configuration::RecurringSettings;

use axum::http::Method;
use routes::{
    complete_todo_item, create_recurring_template_handler, create_todo, create_todo_item,
    delete_recurring_template_handler, delete_todo, delete_todo_item,
    get_recurring_template_handler, get_todo, get_todo_item, google_callback, google_login,
    health_check, list_recurring_templates_handler, list_todo, list_todo_items,
    update_recurring_template_handler, update_todo, update_todo_item,
};
use tower::timeout::TimeoutLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info_span};

pub mod auth;
pub mod configuration;
mod domain;
mod error;
mod extractors;
mod repos;
mod routes;
pub mod services;
pub mod startup;
pub mod telemetry;

const REQUEST_ID_HEADER: &str = "x-request-id";
const MAX_BODY_BYTES: usize = 1024 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub recurring_settings: RecurringSettings,
    pub auth: configuration::AuthSettings,
    pub jwt_service: auth::JwtService,
}

pub type Server = Serve<tokio::net::TcpListener, Router, Router>;

pub async fn run(
    listener: tokio::net::TcpListener,
    pg_pool: Pool<Postgres>,
    recurring_settings: RecurringSettings,
    auth: configuration::AuthSettings,
    jwt_service: auth::JwtService,
) -> Result<Server> {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);
    let request_middleware = ServiceBuilder::new()
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
        .layer(PropagateRequestIdLayer::new(x_request_id))
        .layer(HandleErrorLayer::new(|error: BoxError| async move {
            if error.is::<tower::timeout::error::Elapsed>() {
                StatusCode::REQUEST_TIMEOUT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }))
        .layer(TimeoutLayer::new(REQUEST_TIMEOUT));
    let cors = CorsLayer::new()
        // allow `GET`, `POST`, `PUT`, and `DELETE` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);
    let mut app = Router::new().route("/health_check", get(health_check));

    // Conditionally add authentication routes based on auth type
    match &auth {
        configuration::AuthSettings::GoogleOAuth { .. } => {
            app = app
                .route("/auth/google", get(google_login))
                .route("/auth/google/callback", get(google_callback));
        }
        configuration::AuthSettings::Jwt { .. } => {
            // JWT auth doesn't need special login routes - tokens are handled via the extractor
        }
    }

    let app = app
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
        .route(
            "/todo/{todo_id}/recurring",
            post(create_recurring_template_handler),
        )
        .route(
            "/todo/{todo_id}/recurring",
            get(list_recurring_templates_handler),
        )
        .route(
            "/todo/{todo_id}/recurring/{template_id}",
            get(get_recurring_template_handler),
        )
        .route(
            "/todo/{todo_id}/recurring/{template_id}",
            put(update_recurring_template_handler),
        )
        .route(
            "/todo/{todo_id}/recurring/{template_id}",
            delete(delete_recurring_template_handler),
        )
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(request_middleware)
        .layer(cors)
        .with_state(AppState {
            pool: pg_pool,
            recurring_settings,
            auth,
            jwt_service,
        });
    Ok(axum::serve(listener, app))
}
