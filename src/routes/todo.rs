use serde::{Deserialize, Serialize};
use validator::Validate;

use axum::extract;
use axum::Json;

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
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(), APIError> {
    let todo = payload.try_into()?;
    create_todo_repos(conn, todo).await
}

#[tracing::instrument(
    name = "Get TODO"
    skip(conn, todo_str),
    fields(
        todo = todo_str
    )
)]
pub async fn get_todo(
    DatabaseConnection(conn): DatabaseConnection,
    extract::Path(todo_str): extract::Path<String>,
) -> Result<Json<GetTodoResponse>, APIError> {
    let todo_name = todo_str.try_into()?;
    let todo_response = get_todo_by_name(conn, &todo_name).await?.into();
    Ok(Json(todo_response))
}

#[tracing::instrument(
    name = "List TODO"
    skip(conn),
)]
pub async fn list_todo(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<Json<ListTodoResponse>, APIError> {
    let todo_response = repos::list_todo(conn).await?.into();
    Ok(Json(todo_response))
}
