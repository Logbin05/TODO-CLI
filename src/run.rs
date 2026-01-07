use crate::services::{add_todo::add_todo, clear_terminal::clear_terminal};
use crate::types::todo::Todo;
use crate::ui::home::home_page;
use std::io::{self, Write};

pub async fn run_cli() {
  let mut todos: Vec<Todo> = Vec::new();
  let mut counter: u32 = 0;
    loop {
        home_page();
        print!("Enter option: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim();

        match choice {
          "2" => add_todo(&mut todos, &mut counter),
          "c" | "clear" | "cls" => clear_terminal(),
          "q" | "quit" | "exit" => break,
          _ => println!("\nUnknown option"),
        }
    }
}
