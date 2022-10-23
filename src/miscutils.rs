use crate::Result;
use std::str::FromStr;
use std::fmt::Display;
use std::io::{self, Write};

pub fn ask(prompt: &str) -> Result<String> {
    let stdin = io::stdin();
    // Print the prompt
    print!("{} ", prompt);
    io::stdout().flush()?;
    // Read & return
    let mut s = String::new();
    stdin.read_line(&mut s)?;
    return Ok(s.trim().to_string());
}

pub fn confirm(def: bool) -> Result<bool> {
    let answer = ask("Are you sure?")?;
    match answer.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        "" => Ok(def),
        _ => {
            println!("Accepted values:\ny, yes, n, no");
            confirm(def)
        }
    }
}

pub fn add_default<T: Display>(msg: &str, opt: &Option<T>) -> String {
    match opt {
        Some(s) => format!("{} [{}]", msg, s),
        None => msg.to_string(),
    }
}
