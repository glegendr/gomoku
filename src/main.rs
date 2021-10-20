use std::io;
mod board;
use board::{TOTAL_TILES, Board};
mod error;
mod color;
use color::{Color};
mod players;
use players::*;

fn get_human_input(_player_color: Color) -> (usize, usize) {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let vec: Vec<i32> = guess.trim().split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (vec[0] as usize, vec[1] as usize)
}

fn main() {
    let mut board: Board = Board::new(TOTAL_TILES);
    let player1 = Player::new(Color::Black, PlayerType::Human);
    let player2 = Player::new(Color::White, PlayerType::Human);
    let mut players = Players::new(player1, player2);
    loop {
        match (board.is_finished(), players.is_finished()) {
            (_, (true, Some(color))) => {
                println!("BRAVO {:?} \"{}\"", color, color);
                break;
            },
            ((true, None), _) => {
                println!("DRAW !");
                break;
            },
            ((true, Some(color)), _) => {
                println!("BRAVO {:?} \"{}\"", color, color);
                break;
            },
            _ => ()

        };
        let input = match players.get_current_player().get_player_type() {
            PlayerType::Human => get_human_input(players.get_current_player().get_player_color()),
            PlayerType::Bot => (0,0)
        };
        match board.add_value(input, &mut players) {
            Ok(_) => players.next_player(),
            Err(e) => println!("{}", e)
        };
        println!("{}", board);
        println!("{:?}", players);
    }
}