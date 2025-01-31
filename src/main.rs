mod parser;
mod commands;

use parser::{parse_input,handle_piping};
use commands::{handle_cd,handle_exit,handle_pwd,handle_grep,get_path};

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
            Ok((commands)) => {
                if commands.len() == 1 {
                    let (command, args) = &commands[0];
                    match command.as_str() {
                        "cd" => {
                            handle_cd(&args).unwrap();
                            let new_path = get_path().unwrap();
                            welcome.clear();
                            let new_wel = format!("rush {} $ ", new_path);
                            welcome.push_str(&new_wel)
                        },
                        "exit" => handle_exit().unwrap(),
                        "pwd" => handle_pwd().unwrap(),
                        "grep" => handle_grep(args).unwrap(),
                        _ => {
                            let output = Command::new(&command)
                                .args(args)
                                .output()
                                .expect("Failed to execute command");

                            if output.status.success() {
                                println!("{}", String::from_utf8_lossy(&output.stdout));
                            } else {
                                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                            }
                        }
                    }
                }else { 
                    handle_piping(commands.clone()).unwrap_or_else(|err| eprintln!("Error: {err}"))
                }
            }

            Err(err) => {
                eprintln!("Error: {}", err)
            }
        }
    }
}
