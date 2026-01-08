use chrono::Local;
use std::io::{self, Write};

use crate::services::clear_terminal::clear_terminal;
use crate::types::todo::{StatusTodo, Todo};

pub fn add_todo(todos: &mut Vec<Todo>, counter: &mut u32) {
    loop {
        clear_terminal();
        println!("Instruction for using CLI\n\n Steps:\n - enter name\n - [q] to quit\n");

        print!("Enter todo name: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => todo!(),
            "2" => todo!(),
            "q" => break,
            _ => println!("Ups")
        }

        *counter += 1;
        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        todos.push(Todo {
            todo_id: *counter,
            name_todo: input.to_string(),
            status: StatusTodo::ACTIVE,
            date: date.clone(),
        });
        println!("\n\nSuccess! The todo successfully created.");
        println!("{:<5} | {:<30} | {:<10} | {}", *counter, input, StatusTodo::ACTIVE, date);

    }
}
