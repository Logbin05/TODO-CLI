use crate::ui::home::home_page;
use std::io::{self, Write};

pub async fn run_cli() {
    loop {
        home_page();
        print!("Enter option: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim();

        
    }
}
