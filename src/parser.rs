pub fn parse_input(input: &str) -> Result<(String , Vec<&str>), &'static str>{
    let parts:Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty(){
        return Err("Empty input")
    }

    let command = parts[0].to_string();
    let args = parts[1..].to_vec();

    Ok((command, args))
}