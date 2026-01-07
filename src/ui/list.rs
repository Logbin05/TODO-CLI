use crate::services::clear_terminal::clear_terminal;
use crate::types::todo::Todo;
use std::io::{self, Write};

pub fn list_todo(todos: &Vec<Todo>) {
    loop {
        clear_terminal();
        println!("• Your Todo List •\n");

        if todos.is_empty() {
            println!("No todos yet!\n");
        } else {
            println!(
                "{:<5} | {:<30} | {:<10} | {}",
                "ID", "Name", "Status", "Date"
            );
            println!("{}", "-".repeat(60));

            for todo in todos {
                println!(
                    "{:<5} | {:<30} | {:<10} | {}",
                    todo.todo_id, todo.name_todo, todo.status, todo.date
                );
            }
        }

        println!("\nInstruction: [q] to quit\n");
        print!("Enter action: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        }
    }
}
