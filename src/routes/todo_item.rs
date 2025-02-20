use axum::extract;
use axum::Json;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use uuid::Uuid;

use crate::domain;
use crate::domain::NewTodoItemRequest;
use crate::error::APIError;
use crate::extractors::DatabaseConnection;
use crate::repos;

#[derive(Debug, Deserialize)]
pub struct CreateTodoItemRequest {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct ListTodoItemResponse {
    pub items: Vec<TodoItemSingleResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoItemSingleResponse {
    pub todo_item_id: Uuid,
    pub title: String,
    pub is_complete: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoItemRequest {
    pub title: String,
    pub is_complete: bool,
}

// These are alias as they are incidentally the same thing.
// Create new types as it evolves
pub type CreateTodoItemResponse = TodoItemSingleResponse;
pub type GetTodoItemResponse = TodoItemSingleResponse;
pub type UpdateTodoItemResponse = TodoItemSingleResponse;

impl TryFrom<CreateTodoItemRequest> for NewTodoItemRequest {
    type Error = APIError;
    fn try_from(value: CreateTodoItemRequest) -> Result<Self, Self::Error> {
        Ok(Self { title: value.title })
    }
}

impl TryFrom<UpdateTodoItemRequest> for domain::UpdateTodoItemRequest {
    type Error = APIError;
    fn try_from(value: UpdateTodoItemRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            is_complete: value.is_complete,
        })
    }
}

impl From<domain::ListTodoItem> for ListTodoItemResponse {
    fn from(value: domain::ListTodoItem) -> Self {
        Self {
            items: value.items.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<domain::TodoItem> for TodoItemSingleResponse {
    fn from(value: domain::TodoItem) -> Self {
        Self {
            todo_item_id: value.todo_item_id,
            title: value.title,
            is_complete: value.is_complete,
        }
    }
}

impl From<domain::ListTodoItemSingle> for TodoItemSingleResponse {
    fn from(value: domain::ListTodoItemSingle) -> Self {
        Self {
            todo_item_id: value.todo_item_id,
            title: value.title,
            is_complete: value.is_complete,
        }
    }
}

impl TryFrom<TodoItemSingleResponse> for domain::UpdateTodoItemRequest {
    type Error = APIError;
    fn try_from(value: TodoItemSingleResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            is_complete: value.is_complete,
        })
    }
}

#[tracing::instrument(
    name = "List TODO Item"
    skip(conn, todo_str),
    fields(
        todo_name = %todo_str
    )
)]
pub async fn list_todo_items(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path(todo_str): extract::Path<String>,
) -> Result<Json<ListTodoItemResponse>, APIError> {
    let todo_name = todo_str.try_into()?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    let result = repos::list_todo_items(&mut transaction, &todo_name)
        .await?
        .into();
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(Json(result))
}

#[tracing::instrument(
    name = "Create TODO Item"
    skip(conn, todo_str, payload),
    fields(
        todo_name = %todo_str
    )
)]
pub async fn create_todo_item(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path(todo_str): extract::Path<String>,
    Json(payload): Json<CreateTodoItemRequest>,
) -> Result<Json<CreateTodoItemResponse>, APIError> {
    let todo_name = todo_str.try_into()?;
    let todo = payload.try_into()?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    let todo_item = repos::create_todo_item(&mut transaction, &todo_name, &todo)
        .await?
        .into();
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(Json(todo_item))
}

#[tracing::instrument(
    name = "Get TODO Item"
    skip(conn, todo_str, todo_item),
    fields(
        todo_name = %todo_str,
        todo_item = %todo_item,
    )
)]
pub async fn get_todo_item(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path((todo_str, todo_item)): extract::Path<(String, Uuid)>,
) -> Result<Json<GetTodoItemResponse>, APIError> {
    let todo_name = todo_str.try_into()?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    let todo_item = repos::get_todo_item(&mut transaction, &todo_name, &todo_item)
        .await?
        .into();
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(Json(todo_item))
}

#[tracing::instrument(
    name = "Update TODO Item"
    skip(conn, todo_str, todo_item, payload),
    fields(
        todo_name = %todo_str,
        todo_item = %todo_item,
    )
)]
pub async fn update_todo_item(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path((todo_str, todo_item)): extract::Path<(String, Uuid)>,
    Json(payload): Json<UpdateTodoItemRequest>,
) -> Result<Json<UpdateTodoItemResponse>, APIError> {
    let todo_name = todo_str.try_into()?;
    let item = payload.try_into()?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    let todo_item = repos::update_todo_item(&mut transaction, &todo_name, &todo_item, &item)
        .await?
        .into();
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(Json(todo_item))
}

#[tracing::instrument(
    name = "Delete TODO Item"
    skip(conn, todo_str, todo_item),
    fields(
        todo_name = %todo_str,
        todo_item = %todo_item,
    )
)]
pub async fn delete_todo_item(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path((todo_str, todo_item)): extract::Path<(String, Uuid)>,
) -> Result<(), APIError> {
    let todo_name = todo_str.try_into()?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire transaction")?;
    repos::delete_todo_item(&mut transaction, &todo_name, &todo_item).await?;
    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;
    Ok(())
}
