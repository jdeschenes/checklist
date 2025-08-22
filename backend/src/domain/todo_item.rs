use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NewTodoItemRequest {
    pub title: String,
    pub due_date: Date,
    pub recurring_template_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub todo_item_id: Uuid,
    pub title: String,
    pub due_date: Date,
    pub is_complete: bool,
    pub complete_time: Option<OffsetDateTime>,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct ListTodoItem {
    pub items: Vec<ListTodoItemSingle>,
}

#[derive(Debug, Clone)]
pub struct ListTodoItemSingle {
    pub todo_item_id: Uuid,
    pub title: String,
    pub due_date: Date,
    pub is_complete: bool,
    pub complete_time: Option<OffsetDateTime>,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct UpdateTodoItemRequest {
    pub title: String,
    pub due_date: Date,
}
