use sqlx::PgTransaction;
use uuid::Uuid;

use crate::{
    domain::{
        ListTodoItem, ListTodoItemSingle, NewTodoItemRequest, TodoItem, TodoName,
        UpdateTodoItemRequest,
    },
    error::APIError,
};

use super::get_todo_by_name;

#[tracing::instrument(
    name = "Create todo item in the database",
    skip(transaction, todo_name, req)
)]
pub async fn create_todo_item(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    req: &NewTodoItemRequest,
) -> Result<TodoItem, APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;
    let result = sqlx::query_as!(
        TodoItem,
        r#"INSERT INTO todo_item (todo_item_id, todo_id, title) VALUES ($1, $2, $3)
           RETURNING todo_item_id, title, is_complete;"#,
        Uuid::new_v4(),
        todo.todo_id,
        req.title,
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(result)
}

#[tracing::instrument(
    name = "Get todo item in the database",
    skip(transaction, todo_name, todo_item)
)]
pub async fn get_todo_item(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    todo_item: &Uuid,
) -> Result<TodoItem, APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;
    match sqlx::query_as!(
        TodoItem,
        r#"SELECT todo_item_id, title, is_complete
           FROM todo_item
           WHERE
              todo_id = $1
              AND todo_item_id = $2;"#,
        todo.todo_id,
        todo_item,
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(APIError::NotFound(format!(
            "todo item: {} is not found",
            todo_item
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(
    name = "Update todo item in the database",
    skip(transaction, todo_name, todo_item, req)
)]
pub async fn update_todo_item(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    todo_item: &Uuid,
    req: &UpdateTodoItemRequest,
) -> Result<TodoItem, APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;
    match sqlx::query_as!(
        TodoItem,
        r#"UPDATE todo_item SET
            title = $3
            , is_complete = $4
           WHERE
              todo_id = $1
              AND todo_item_id = $2
           RETURNING todo_item_id, title, is_complete
            ;"#,
        todo.todo_id,
        todo_item,
        req.title,
        req.is_complete,
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result),
        Err(sqlx::Error::RowNotFound) => Err(APIError::NotFound(format!(
            "todo item: {} is not found",
            todo_item
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

impl From<Vec<ListTodoItemSingle>> for ListTodoItem {
    fn from(value: Vec<ListTodoItemSingle>) -> Self {
        Self {
            items: value.into_iter().collect(),
        }
    }
}

#[tracing::instrument(name = "List todo items in the database", skip(transaction, todo_name))]
pub async fn list_todo_items(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
) -> Result<ListTodoItem, APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;
    match sqlx::query_as!(
        ListTodoItemSingle,
        r#"SELECT todo_item_id, title, is_complete
           FROM todo_item
           WHERE
              todo_id = $1;"#,
        todo.todo_id,
    )
    .fetch_all(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result.into()),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(
    name = "Delete todo items in the database",
    skip(transaction, todo_name)
)]
pub async fn delete_todo_item(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    todo_item: &Uuid,
) -> Result<(), APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;
    sqlx::query!(
        r#"DELETE from todo_item
           WHERE
              todo_id = $1
              AND todo_item_id = $2;"#,
        todo.todo_id,
        todo_item,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
