use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub todo_id: i32,
    pub name_todo: String,
    pub date: String,
}
