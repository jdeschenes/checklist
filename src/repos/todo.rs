use sqlx::PgTransaction;
use uuid::Uuid;

use crate::{
    domain::{ListTodo, ListTodoItem, NewTodoRequest, Todo, TodoName, UpdateTodoRequest},
    error::APIError,
};

#[tracing::instrument(name = "Create todo in the database", skip(transaction, req))]
pub async fn create_todo(
    transaction: &mut PgTransaction<'_>,
    req: &NewTodoRequest,
) -> Result<(), APIError> {
    match sqlx::query!(
        r#"INSERT INTO todo (todo_id, name) VALUES ($1, $2)
           ON CONFLICT (name) DO NOTHING
           RETURNING name;"#,
        Uuid::new_v4(),
        req.name.as_ref(),
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(APIError::AlreadyExists(format!(
            "TODO: '{}' already exists",
            req.name.as_ref(),
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[derive(Debug)]
struct GetTodoQuery {
    name: String,
}

impl TryFrom<GetTodoQuery> for Todo {
    type Error = APIError;
    fn try_from(value: GetTodoQuery) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
        })
    }
}

#[tracing::instrument(name = "Create todo in the database", skip(transaction))]
pub async fn get_todo_by_name(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
) -> Result<Todo, APIError> {
    match sqlx::query_as!(
        GetTodoQuery,
        r#"SELECT name from todo WHERE name = $1;"#,
        todo_name.as_ref()
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result.try_into()?),
        Err(sqlx::Error::RowNotFound) => Err(APIError::NotFound(format!(
            "todo: {} is not found",
            todo_name.as_ref()
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(name = "Update todo in the database", skip(transaction))]
pub async fn update_todo(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    req: &UpdateTodoRequest,
) -> Result<(), APIError> {
    match sqlx::query!(
        r#"UPDATE todo SET
            name = $2
            WHERE name = $1
            RETURNING name;"#,
        todo_name.as_ref(),
        req.name.as_ref(),
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(APIError::NotFound(format!(
            "TODO: '{}' does not exist",
            todo_name.as_ref(),
        ))),
        Err(sqlx::Error::Database(e)) => {
            if e.is_unique_violation() {
                return Err(APIError::AlreadyExists(format!(
                    "TODO: '{}' already exists",
                    req.name.as_ref()
                )));
            }
            Err(APIError::Internal(sqlx::Error::Database(e).into()))
        }
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[derive(Debug)]
struct ListTodoQuery {
    name: String,
}

impl TryFrom<Vec<ListTodoQuery>> for ListTodo {
    type Error = APIError;
    fn try_from(value: Vec<ListTodoQuery>) -> Result<Self, Self::Error> {
        let items: Result<Vec<ListTodoItem>, Self::Error> =
            value.into_iter().map(|i| i.try_into()).collect();
        Ok(Self { items: items? })
    }
}

impl TryFrom<ListTodoQuery> for ListTodoItem {
    type Error = APIError;
    fn try_from(value: ListTodoQuery) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
        })
    }
}

#[tracing::instrument(name = "list todo in the database", skip(transaction))]
pub async fn list_todo(transaction: &mut PgTransaction<'_>) -> Result<ListTodo, APIError> {
    match sqlx::query_as!(ListTodoQuery, r#"SELECT name from todo;"#,)
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(result) => Ok(result.try_into()?),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}
