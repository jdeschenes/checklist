use serde::{Deserialize, Serialize};
use validator::Validate;

use axum::extract;
use axum::Json;
use sqlx::Acquire;

use crate::domain::NewTodoRequest;
use crate::domain::{ListTodo, ListTodoItem, Todo};
use crate::error::APIError;
use crate::extractors::DatabaseConnection;
use crate::repos;
use crate::repos::{create_todo as create_todo_repos, get_todo_by_name};

#[derive(Debug, Serialize, Deserialize, Validate)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTodoResponse {
    pub name: String,
}

impl From<Todo> for GetTodoResponse {
    fn from(value: Todo) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
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
}

impl From<ListTodoItem> for ListTodoSingleItem {
    fn from(value: ListTodoItem) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
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
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(), APIError> {
    let todo = payload.try_into()?;
    let mut transaction = conn.begin().await?;
    create_todo_repos(&mut transaction, todo).await?;
    transaction.commit().await?;
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
    extract::Path(todo_str): extract::Path<String>,
) -> Result<Json<GetTodoResponse>, APIError> {
    let todo_name = todo_str.try_into()?;
    let mut transaction = conn.begin().await?;
    let todo_response = get_todo_by_name(&mut transaction, &todo_name).await?.into();
    Ok(Json(todo_response))
}

#[tracing::instrument(
    name = "List TODO"
    skip(conn),
)]
pub async fn list_todo(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<ListTodoResponse>, APIError> {
    let mut transaction = conn.begin().await?;
    let todo_response = repos::list_todo(&mut transaction).await?.into();
    Ok(Json(todo_response))
}
