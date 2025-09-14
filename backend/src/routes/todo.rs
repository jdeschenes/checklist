use eyre::Context;
use serde::{Deserialize, Serialize};

use axum::extract;
use axum::Json;
use sqlx::Acquire;
use time::OffsetDateTime;

use crate::domain;
use crate::domain::NewTodoRequest;
use crate::domain::{ListTodo, ListTodoSingle, Todo};
use crate::error::APIError;
use crate::extractors::{AuthenticatedUser, DatabaseConnection};
use crate::repos;

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub name: String,
}

impl TryFrom<CreateTodoRequest> for NewTodoRequest {
    type Error = APIError;
    fn try_from(value: CreateTodoRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub name: String,
}

impl TryFrom<UpdateTodoRequest> for domain::UpdateTodoRequest {
    type Error = APIError;
    fn try_from(value: UpdateTodoRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GetTodoResponse {
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub update_time: OffsetDateTime,
}

impl From<Todo> for GetTodoResponse {
    fn from(value: Todo) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
            create_time: value.create_time,
            update_time: value.update_time,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListTodoResponse {
    items: Vec<ListTodoSingleItem>,
}

#[derive(Debug, Serialize)]
pub struct ListTodoSingleItem {
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub update_time: OffsetDateTime,
}

impl From<ListTodoSingle> for ListTodoSingleItem {
    fn from(value: ListTodoSingle) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
            create_time: value.create_time,
            update_time: value.update_time,
        }
    }
}

impl From<ListTodo> for ListTodoResponse {
    fn from(value: ListTodo) -> Self {
        Self {
            items: value.items.into_iter().map(|i| i.into()).collect(),
        }
    }
}

#[tracing::instrument(
    name = "Create TODO"
    skip(conn, payload),
    fields(
        todo_name = %payload.name
    )
)]
pub async fn create_todo(
    DatabaseConnection(mut conn): DatabaseConnection,
    user: AuthenticatedUser,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(), APIError> {
    let todo = payload.try_into()?;
    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    repos::create_todo(&mut transaction, &todo, user.user_id).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(())
}

#[tracing::instrument(
    name = "Get TODO"
    skip(conn, todo_str),
    fields(
        todo = todo_str
    )
)]
pub async fn get_todo(
    DatabaseConnection(mut conn): DatabaseConnection,
    user: AuthenticatedUser,
    extract::Path(todo_str): extract::Path<String>,
) -> Result<Json<GetTodoResponse>, APIError> {
    let todo_name = todo_str.try_into()?;
    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    let todo_response = repos::get_todo_by_name(&mut transaction, &todo_name, user.user_id)
        .await?
        .into();
    Ok(Json(todo_response))
}

#[tracing::instrument(
    name = "Delete TODO"
    skip(conn, todo_str),
    fields(
        todo = todo_str
    )
)]
pub async fn delete_todo(
    DatabaseConnection(mut conn): DatabaseConnection,
    user: AuthenticatedUser,
    extract::Path(todo_str): extract::Path<String>,
) -> Result<(), APIError> {
    let todo_name = todo_str.try_into()?;
    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    repos::delete_todo_by_name(&mut transaction, &todo_name, user.user_id).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(())
}

#[tracing::instrument(
    name = "Update TODO"
    skip(conn, todo_str, payload),
    fields(
        old_todo = todo_str,
        todo_name = %payload.name,
    )
)]
pub async fn update_todo(
    DatabaseConnection(mut conn): DatabaseConnection,
    user: AuthenticatedUser,
    extract::Path(todo_str): extract::Path<String>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<(), APIError> {
    let todo_name = todo_str.try_into()?;
    let todo = payload.try_into()?;
    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    repos::update_todo(&mut transaction, &todo_name, &todo, user.user_id).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(())
}

#[tracing::instrument(
    name = "List TODO"
    skip(conn),
)]
pub async fn list_todo(
    DatabaseConnection(mut conn): DatabaseConnection,
    user: AuthenticatedUser,
) -> Result<Json<ListTodoResponse>, APIError> {
    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    let todo_response = repos::list_todo(&mut transaction, user.user_id)
        .await?
        .into();
    Ok(Json(todo_response))
}
