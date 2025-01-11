mod parser;
mod commands;

use parser::parse_input;
use commands::{handle_cd,handle_exit,handle_pwd,get_path};

use std::io::{self, Write};
use std::process::Command;

fn main() {
    let path = get_path().unwrap();
    let mut welcome:String = format!("rush {} $ ", &path);
    loop {
        print!("{welcome}");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match parse_input(input.trim()) {
            Ok((command, args)) => {
                match command.as_str() {
                    "cd" => {
                        handle_cd(&args).unwrap();
                        let new_path = get_path().unwrap();
                        welcome.clear();
                        let new_wel = format!("rush {} &",new_path);
                        welcome.push_str(&new_wel)
                    },
                    "exit" => handle_exit().unwrap(),
                    "pwd" => handle_pwd().unwrap(),
                    _ => {
                        let output = Command::new(&command)
                            .args(&args)
                            .output()
                            .expect("Failed to execute command");

                        if output.status.success() {
                            println!("{}", String::from_utf8_lossy(&output.stdout));
                        } else {
                            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                }
            }

            Err(err) => {
                eprintln!("Error: {}", err)
            }
        }
    }
}
