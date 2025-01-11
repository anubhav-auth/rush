use std::env;

pub fn handle_cd(args: &[&str]) -> Result<(), &'static str>{
    if args.len() != 1 {
        return Err("Usage: cd <directory>");
    }

    let new_dir = &args[0];
    env::set_current_dir(new_dir).map_err(|_| "Failed to change directory")?;

    Ok(())
}

pub fn handle_exit() -> Result<(), &'static str> {
    std::process::exit(0);
}

pub fn handle_pwd() -> Result<(), &'static str> {
    let current_dir = env::current_dir().map_err(|_| "Failed to get current directory")?;
    println!("{}", current_dir.display());
    Ok(())
}
pub fn get_path()->Result<String, &'static str>{
    let current_dir = env::current_dir().map_err(|_| "Failed to get current directory")?;
    let s = current_dir
        .to_str()
        .ok_or("failed to convert directory path to string")?
        .to_string();
    Ok(s)
}