use chrono::Local;
use std::io::{self, Write};

use crate::services::clear_terminal::clear_terminal;
use crate::types::todo::{StatusTodo, Todo};

pub fn add_todo(todos: &mut Vec<Todo>, counter: &mut u32) {
    loop {
        clear_terminal();
        println!("\n\nInstruction for using CLI\n\n Steps:\n - enter name\n - [q] to quit\n");

        print!("Enter todo name: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        }

        *counter += 1;
        todos.push(Todo {
            todo_id: *counter,
            name_todo: input.to_string(),
            status: StatusTodo::ACTIVE,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
        print!("\nSuccess! The todo successfully created.");
    }
}
