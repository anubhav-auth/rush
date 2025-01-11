mod parser;
use parser::parse_input;

use std::io::{self, Write};

fn main() {
    loop {
        print!("rush~$> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "exit" {
            break;
        }

        match parse_input(trimmed) {
            Ok()
        }
        println!("{}:invalid input", input)
    }
}
