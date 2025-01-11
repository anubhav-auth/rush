pub fn parse_input(input: &str) -> Result<(String , Vec<String>), &'static str>{
    let parts:Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty(){
        return Err("Empty input")
    }

    let command = parts[0].to_string();
    let args = parts[1..].iter().map(|s| s.to_string()).collect();

    Ok((command, args))
}