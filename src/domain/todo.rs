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

impl TryFrom<String> for TodoName {
    type Error = APIError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
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

impl TryFrom<&str> for TodoName {
    type Error = APIError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.to_string();
        s.try_into()
    }
}

impl AsRef<str> for TodoName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ListTodo {
    pub items: Vec<ListTodoItem>,
}

#[derive(Debug, Clone)]
pub struct ListTodoItem {
    pub name: TodoName,
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::error::APIError;

    use super::{TodoName, MAX_TODO_NAME_LENGTH};
    use claims::{assert_err, assert_ok};

    #[test]
    fn name_is_ok() {
        let test_cases = vec![
            ("banana", "banana"),
            ("  banana  ", "banana"),
            ("1234", "1234"),
        ];
        for test_case in test_cases {
            let result: Result<TodoName, _> = test_case.0.try_into();
            assert_ok!(&result);
            assert_eq!(result.unwrap().as_ref(), test_case.1);
        }
    }

    #[test]
    fn name_is_not_ok() {
        let test_cases = vec![
            ("", "Name cannot be empty".to_string()),
            ("    ", "Name cannot be empty".to_string()),
            (
                "123456789012345678901",
                format!("Name is too long cannot exceed: {}", MAX_TODO_NAME_LENGTH),
            ),
        ];
        for test_case in test_cases {
            let result: Result<TodoName, _> = test_case.0.try_into();
            assert_err!(&result);
            match result.unwrap_err() {
                APIError::BadRequest(x) => assert_eq!(x, test_case.1),
                x => panic!("Unexpected error: {}", x),
            }
        }
    }
}
