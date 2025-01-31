use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};

pub fn parse_input(input: &str) -> Result<Vec<(String, Vec<&str>)>, &'static str>{
    let parts = input.split('|')
        .map(str::trim)
        .filter(|cmd| !cmd.is_empty())
        .map(|cmd| {
            let mut parts = cmd.split_whitespace();
            let command = parts.next().unwrap().to_string();
            let args = parts.collect();
            (command,args)
        })
        .collect();

    Ok(parts)
}

pub fn handle_piping(commands: Vec<(String, Vec<&str>)>) -> Result<(), &'static str> {
    let mut previous_output = None;
    let mut children = Vec::new();

    for (i, (command, args)) in commands.iter().enumerate() {
        let stdin = match previous_output {
            Some(output) => Stdio::from(output), // Pipe output from previous command
            None => Stdio::inherit(), // First command takes input from terminal
        };

        let stdout = if i == commands.len() - 1 {
            Stdio::inherit() // Last command prints to terminal
        } else {
            Stdio::piped() // Intermediate commands pipe their output
        };

        let mut child = Command::new(command)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
            .map_err(|_| "Failed to execute command")?;

        previous_output = child.stdout.take();
        children.push(child);
    }

    // Wait for all child processes to finish
    for mut child in children {
        child.wait().map_err(|_| "Failed to wait for child process")?;
    }

    Ok(())
}

