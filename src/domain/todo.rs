use std::str::FromStr;

use crate::error::APIError;

#[derive(Debug, Clone)]
pub struct NewTodoRequest {
    pub name: TodoName,
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub name: TodoName,
}

#[derive(Debug, Clone)]
pub struct TodoName(String);

const MAX_TODO_NAME_LENGTH: usize = 20;

impl FromStr for TodoName {
    type Err = APIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(APIError::BadRequest("Name cannot be empty".to_string()));
        }
        if s.len() > MAX_TODO_NAME_LENGTH {
            return Err(APIError::BadRequest(format!(
                "Name is too long cannot exceed: {}",
                MAX_TODO_NAME_LENGTH
            )));
        }
        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for TodoName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
