use colored::*;

struct Matrix {
    boxes: Vec<char>
}

impl Matrix {
    fn print_vec(self) {
        for (i, item) in self.boxes.iter().enumerate() {
            if i % 3 != 0 {
                print!("{}", "|".blue());
            }
            print!(" {} ", item);
            if (i+1) % 3 == 0 && i != 8 {
                println!();
                println!("{}","---+---+---".blue()); 
            }
        }
    }  

    fn play(&mut self, position: usize, player: char) {
        self.boxes[position] = player
    }
}

fn main() {
    start_game();

    let mut game_board = Matrix {
        boxes: ('1'..='9').collect()
    };
    let player:char = 'X';
    let position:usize = 2;

//    Matrix::play(&mut gameboard, position, player);
    Matrix::print_vec(game_board);
    end_game();
}
fn start_game() {
    println!("{}","========= TIC TAC TOE =========".red());
}

fn end_game() {
    println!();
    println!();
    println!("{}", "==============================".red());
}
