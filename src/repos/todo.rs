use eyre::Context;
use sqlx::{pool::PoolConnection, Postgres};
use uuid::Uuid;

use crate::{
    error::InternalError,
    types::{CreateListRequest, CreateListResponse, GetListResponse},
};

#[derive(Debug)]
struct CreateTodoQuery {
    name: String,
}

impl From<CreateTodoQuery> for CreateListResponse {
    fn from(value: CreateTodoQuery) -> Self {
        Self { name: value.name }
    }
}

#[tracing::instrument(name = "Create todo in the database", skip(conn, req))]
pub async fn create_todo(
    mut conn: PoolConnection<Postgres>,
    req: CreateListRequest,
) -> Result<CreateListResponse, InternalError> {
    let result = sqlx::query_as!(
        CreateTodoQuery,
        r#"INSERT INTO todo (todo_id, name) VALUES ($1, $2) RETURNING name;"#,
        Uuid::new_v4(),
        req.name
    )
    .fetch_one(&mut *conn)
    .await?;
    Ok(result.into())
}

#[derive(Debug)]
struct GetTodoQuery {
    name: String,
}

impl From<GetTodoQuery> for GetListResponse {
    fn from(value: GetTodoQuery) -> Self {
        Self { name: value.name }
    }
}

#[tracing::instrument(name = "Create todo in the database", skip(conn))]
pub async fn get_todo_by_name(
    mut conn: PoolConnection<Postgres>,
    todo_name: &str,
) -> Result<GetListResponse, InternalError> {
    let result = sqlx::query_as!(
        GetTodoQuery,
        r#"SELECT name from todo WHERE name = $1;"#,
        todo_name
    )
    .fetch_one(&mut *conn)
    .await
    .context("Running query")?;
    Ok(result.into())
}
