mod service;
mod models;

use service::serv;
use models::model;

fn main() {
    serv::start_game();
    let game = model::GameBoard::new_board();
    game.show_board();






    let option = serv::option();
    match option {
        Ok(opt) => println!("your option is: {}", opt),
        Err(e) => println!("Error...\nfor: {}", e)
    }
    serv::end_game();
}


















