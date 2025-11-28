use colored::*;

#[derive(Clone, Copy)]
pub enum Turn{
    X,
    O
}

impl Turn {
    pub fn netx_turn(&self) -> Self {
        match self {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    PlayerX,
    PlayerO,
}

pub struct GameBoard {
    board: [[Cell; 3]; 3]
}

impl GameBoard {
   pub fn new_board() -> Self {
        GameBoard {
            board: [[Cell::Empty; 3]; 3]
        }
    }

    pub fn show_board(&self) {
        for i in 0..3 {
            for j in 0..3{
                let current_pos = (i * 3 + j + 1).to_string();
                match self.board[i][j]{
                    Cell::Empty => print!(" {} ", current_pos),
                    Cell::PlayerX => print!(" {} ", "X".red()),
                    Cell::PlayerO => print!(" {} ", "O".blue())
                }

                if j < self.board[0].len() - 1 {
                    print!("{}", "|".green());
                }
            }
            println!();
            if i < self.board[0].len() - 1 {
                println!("{}", "---+---+---".green());
            }
        }
    }

}

