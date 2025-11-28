use colored::*;
use std::io;
use std::error::Error;

pub fn start_game() {
    let decoration = format!("{}","===".repeat(6).green());
    let title = format!("{}","TIC TAC TOE".green());
    println!("{} {} {}", decoration, title, decoration);
    println!("- {}    - {}", "Player X".red(), "Player O".blue());
    println!("\n");
}
pub fn end_game() {
    let decoration = format!("{}","===".repeat(16).green());
    println!("\n");
    println!("{}", decoration);
}

pub fn option() -> Result<usize, Box<dyn Error>>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input:usize = input.trim().parse()?;
    Ok(input - 1)
}

