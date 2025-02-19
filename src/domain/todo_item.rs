use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NewTodoItemRequest {
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub todo_item_id: Uuid,
    pub title: String,
    pub is_complete: bool,
}

#[derive(Debug, Clone)]
pub struct ListTodoItem {
    pub items: Vec<ListTodoItemSingle>,
}

#[derive(Debug, Clone)]
pub struct ListTodoItemSingle {
    pub todo_item_id: Uuid,
    pub title: String,
    pub is_complete: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateTodoItemRequest {
    pub title: String,
    pub is_complete: bool,
}
