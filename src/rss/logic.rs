use colored::*;

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
