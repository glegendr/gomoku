use std::{io, time, env, process};
mod board;
use board::{Board, Input};
mod error;
use error::{FlagError};
mod color;
use color::{Color};
mod players;
use players::*;
mod algo;
use algo::{get_bot_input};
mod leakser;
use leakser::{leakser};
mod parser;

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
    let mut args: Vec<String> = env::args().collect();
    let mut board: Board;
    let player1 = Player::new(Color::Black, PlayerType::Human);
    let player2 = Player::new(Color::White, PlayerType::Bot);
    let mut players: Players;
    match leakser(&mut args[1..]) {
        Ok((m, c, r, a)) => {
            board = Board::new(m, a, r);
            players = Players::new(player1, player2, c, r)
        },
        Err(e) => {
            println!("{}", e);
            if e != FlagError::PrintHelper || e != FlagError::PrintRules {
                println!("for more information use \"cargo run -- --help\"");
            }
            process::exit(1);
        }
    };
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
