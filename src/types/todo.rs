use std::fmt;
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
    pub todo_id: u32,
    pub name_todo: String,
    pub status: StatusTodo,
    pub date: String,
}

impl fmt::Display for StatusTodo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
        StatusTodo::ACTIVE => write!(f, "Active"),
        StatusTodo::DONE => write!(f, "Done"),
        StatusTodo::CLOSED => write!(f, "Closed"),
        StatusTodo::CANCELLED => write!(f, "Cancelled"),
    }
  }
}