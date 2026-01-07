use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusTodo {
    ACTIVE,
    DONE,
    CLOSED,
    CANCELLED,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub todo_id: i32,
    pub name_todo: String,
    pub status: StatusTodo,
    pub date: String,
}
