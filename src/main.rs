use std::io;
use colored::*;
use std::error::Error;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    PlayerX,
    PlayerO,
}

#[derive(Clone, Copy)]
enum Turn{
    X,
    O
}

impl Turn {
    fn next_turn(&self) -> Self{
        match self {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
}

struct GameBoard {
    board: [[ Cell; 3]; 3]
}

impl GameBoard {
    fn prime_board() -> Self {
        GameBoard {
            board: [[Cell::Empty; 3]; 3],
        }
    }

    fn win_cheker(&self) -> Cell {
        let b = self.board;
        for i in 0..3 {
            if b[i][0] != Cell::Empty && b[i][0] == b[i][1] && b[i][2] == b[i][0] {
                return b[i][0];
            }

            if b[0][i] != Cell::Empty && b[0][i] == b[1][i] && b[2][i] == b[0][i] {
                return b[0][i];
            }
        }

        if b[0][0] != Cell::Empty && b[0][0] == b[1][1] && b[1][1] == b[2][2] {
            return b[0][0];
        }

        if b[0][2] != Cell::Empty && b[0][2] == b[1][1] && b[1][1] == b[2][0] {
            return b[0][2];
        }

        Cell::Empty
    }

    fn show_board(&self) {
        for i in 0..3 {
            for j in 0..3 {
                let current_pos = (i * 3 + j + 1 ).to_string();
                let item_show = match self.board[i][j] {
                    Cell::Empty => &current_pos,
                    Cell::PlayerX => "X",
                    Cell::PlayerO => "Y",
                };

                match self.board[i][j] {
                    Cell::Empty => print!(" {} ", item_show),
                    Cell::PlayerX => print!(" {} ", item_show.red()),
                    Cell::PlayerO => print!(" {} ", item_show.blue()),
                }

                if j < self.board[0].len() -1 {
                    print!("{}", "|".green());
                }
            }
            println!();
            if i < self.board.len()-1 {
                println!("{}", "---+---+---".green());
            }
        }
    } 

    fn play(&mut self, pos: usize, player: Cell) -> bool {
        let i = pos / 3;
        let j = pos % 3;

        if i >=3 || j >=3 { return false; }

        if self.board[i][j] == Cell::Empty {
            self.board[i][j] = player;
            return true;
        }
        else { return false;}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_board = GameBoard::prime_board();
    start_game();
    game_board.show_board();
    println!("\n");

    let mut current_turn = Turn::X;
    for _i in 0..9 {
        let (turn, player, next) = match current_turn{
            Turn::X => ("Payer X", Cell::PlayerX, current_turn),
            Turn::O => ("Payer O", Cell::PlayerO, current_turn)
        };

        match current_turn {
            Turn::X => println!("{}: ", turn.red()), 
            Turn::O => println!("{}", turn.blue())
        }

        let play = option()?;
        if game_board.play(play, player){
            println!("\n");
            game_board.show_board();
            println!("\n");

            if game_board.win_cheker() != Cell::Empty {
                println!(" ¡Felicidades! ¡El Jugador {} ha ganado! ",turn );
                break; // Salir del bucle principal, el juego ha terminado
            }

            current_turn = next.next_turn();
        } else {
            println!("\n{}", "!!!Error!!! try enter a valid number".yellow());
        }



    }
    end_game();
    Ok(())
}

fn option() -> Result<usize, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input:usize = input.trim().parse()?;
    Ok(input - 1)
}



















fn start_game() {
    let decoration = format!("{}","===".repeat(6).green());
    let title = format!("{}","TIC TAC TOE".green());
    println!("{} {} {}", decoration, title, decoration);
    println!("- {}    - {}", "Player X".red(), "Player O".blue());
    println!("\n");
}
fn end_game() {
    let decoration = format!("{}","===".repeat(16).green());
    println!("\n");
    println!("{}", decoration);
}
