use uuid::Uuid;

use axum::extract;
use axum::Json;

use crate::error::InternalError;
use crate::extractors::DatabaseConnection;
use crate::repos::{create_todo as create_todo_repos, get_todo as get_todo_repos};
use crate::types::{CreateListRequest, CreateListResponse, GetListRequest, GetListResponse};

#[tracing::instrument(
    name = "Create TODO"
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
pub async fn create_todo(
    DatabaseConnection(conn): DatabaseConnection,
    extract::Json(payload): extract::Json<CreateListRequest>,
) -> Result<Json<CreateListResponse>, InternalError> {
    create_todo_repos(conn, payload).await.map(Json)
}

#[tracing::instrument(
    name = "Get TODO"
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
pub async fn get_todo(
    DatabaseConnection(conn): DatabaseConnection,
    extract::Json(payload): extract::Json<GetListRequest>,
) -> Result<Json<GetListResponse>, InternalError> {
    get_todo_repos(conn, payload).await.map(Json)
}
