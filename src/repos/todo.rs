use eyre::Context;
use sqlx::{pool::PoolConnection, Postgres};

use crate::{
    error::InternalError,
    types::{CreateListRequest, CreateListResponse, GetListRequest, GetListResponse},
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

pub async fn create_todo(
    mut conn: PoolConnection<Postgres>,
    req: CreateListRequest,
) -> Result<CreateListResponse, InternalError> {
    let result = sqlx::query_as!(
        CreateTodoQuery,
        r#"INSERT INTO todo (name) VALUES ($1) RETURNING name;"#,
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

pub async fn get_todo(
    mut conn: PoolConnection<Postgres>,
    req: GetListRequest,
) -> Result<GetListResponse, InternalError> {
    let result = sqlx::query_as!(
        GetTodoQuery,
        r#"SELECT name from todo WHERE name = $1;"#,
        req.name
    )
    .fetch_one(&mut *conn)
    .await
    .context("Running query")?;
    Ok(result.into())
}
