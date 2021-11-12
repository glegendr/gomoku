// #![feature(test)]
#[path = "../src/board.rs"]
mod board;
use board::*;
#[path = "../src/error.rs"]
mod error;
#[path = "../src/color.rs"]
mod color;
use color::{Color};
#[path = "../src/players.rs"]
mod players;
use players::*;
#[path = "../src/algo.rs"]
mod algo;
use algo::{get_bot_input};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Algo piece start", |b| {
        let mut board: Board = Board::new(TOTAL_TILES);
        let player1 = Player::new(Color::Black, PlayerType::Bot);
        let player2 = Player::new(Color::White, PlayerType::Bot);
        let mut players = Players::new(player1, player2);
        let _ = board.add_value(get_input(0), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board);
        });
    });

    c.bench_function("Algo piece center", |b| {
        let mut board: Board = Board::new(TOTAL_TILES);
        let player1 = Player::new(Color::Black, PlayerType::Bot);
        let player2 = Player::new(Color::White, PlayerType::Bot);
        let mut players = Players::new(player1, player2);
        let _ = board.add_value(get_input(TOTAL_TILES / 2), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board);
        });
    });

    c.bench_function("Algo piece end", |b| {
        let mut board: Board = Board::new(TOTAL_TILES);
        let player1 = Player::new(Color::Black, PlayerType::Bot);
        let player2 = Player::new(Color::White, PlayerType::Bot);
        let mut players = Players::new(player1, player2);
        let _ = board.add_value(get_input(TOTAL_TILES - 1), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board);
        });
    });

    c.bench_function("Algo 3 pieces stacked", |b| {
        let mut board: Board = Board::new(TOTAL_TILES);
        let player1 = Player::new(Color::Black, PlayerType::Bot);
        let player2 = Player::new(Color::White, PlayerType::Bot);
        let mut players = Players::new(player1, player2);
        let _ = board.add_value(get_input(TOTAL_TILES / 2), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 + 1), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 - 1), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board);
        });
    });

    c.bench_function("Algo 6 pieces stacked", |b| {
        let mut board: Board = Board::new(TOTAL_TILES);
        let player1 = Player::new(Color::Black, PlayerType::Bot);
        let player2 = Player::new(Color::White, PlayerType::Bot);
        let mut players = Players::new(player1, player2);
        let _ = board.add_value(get_input(TOTAL_TILES / 2), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 + 1), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 - 1), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 + BOARD_LENGTH), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 + 1 + BOARD_LENGTH), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 - 1 + BOARD_LENGTH), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board);
        });
    });

    c.bench_function("Algo 6 pieces non stacked", |b| {
        let mut board: Board = Board::new(TOTAL_TILES);
        let player1 = Player::new(Color::Black, PlayerType::Bot);
        let player2 = Player::new(Color::White, PlayerType::Bot);
        let mut players = Players::new(player1, player2);
        let _ = board.add_value(get_input(TOTAL_TILES / 2), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 + 8), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 - 8), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 - 2 * BOARD_LENGTH), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 + 8 + BOARD_LENGTH), &mut players);
        let _ = board.add_value(get_input(TOTAL_TILES / 2 - 4 * BOARD_LENGTH), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);