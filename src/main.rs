use std::{io, time};
mod board;
use board::{TOTAL_TILES, Board, Input};
mod error;
mod color;
use color::{Color};
mod players;
use players::*;
mod algo;
use algo::{get_bot_input};

fn get_human_input(_player_color: Color) -> Input {
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
    let player2 = Player::new(Color::White, PlayerType::Bot);
    let mut players = Players::new(player1, player2);
    loop {
        match (board.is_finished(players.get_current_player()), players.is_finished()) {
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
        let now = time::Instant::now();
        let input = match players.get_current_player().get_player_type() {
            PlayerType::Human => get_human_input(players.get_current_player().get_player_color()),
            PlayerType::Bot => get_bot_input(&players, &board),
        };
        let elapsed_time = now.elapsed();
        println!("Input took {:?}.", elapsed_time);
        match board.add_value(input, &mut players) {
            Ok(_) => players.next_player(),
            Err(e) => println!("{}", e)
        };
        println!("{}", board);
        println!("{:?}", players);
    }
}