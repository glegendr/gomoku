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
extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
#[path = "../src/heuristic.rs"]
mod heuristic;
#[path = "../src/matching_cases.rs"]
mod matching_cases;

const BENCHMARK_SIZE: usize = 19;
const BENCHMARK_TOTAL_TILES: usize = BENCHMARK_SIZE * BENCHMARK_SIZE;
const ALIGNEMENT_NB: usize = 5;
const CAPTURE_RANGE: usize = 2;
const CAPTURE_NB: usize = 10;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Algo piece start", |b| {
        let mut board: Board = Board::new(BENCHMARK_SIZE, ALIGNEMENT_NB, CAPTURE_RANGE);
        let player1 = Player::new(Color::Black, PlayerType::Bot(Algorithm::basic_algorithm()));
        let player2 = Player::new(Color::White, PlayerType::Bot(Algorithm::basic_algorithm()));
        let mut players = Players::new(player1, player2, CAPTURE_NB, CAPTURE_RANGE);
        let _ = board.add_value(board.get_input(0), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board, &None);
        });
    });

    c.bench_function("Algo piece center", |b| {
        let mut board: Board = Board::new(BENCHMARK_SIZE, ALIGNEMENT_NB, CAPTURE_RANGE);
        let player1 = Player::new(Color::Black, PlayerType::Bot(Algorithm::basic_algorithm()));
        let player2 = Player::new(Color::White, PlayerType::Bot(Algorithm::basic_algorithm()));
        let mut players = Players::new(player1, player2, CAPTURE_NB, CAPTURE_RANGE);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board, &None);
        });
    });

    c.bench_function("Algo piece end", |b| {
        let mut board: Board = Board::new(BENCHMARK_SIZE, ALIGNEMENT_NB, CAPTURE_RANGE);
        let player1 = Player::new(Color::Black, PlayerType::Bot(Algorithm::basic_algorithm()));
        let player2 = Player::new(Color::White, PlayerType::Bot(Algorithm::basic_algorithm()));
        let mut players = Players::new(player1, player2, CAPTURE_NB, CAPTURE_RANGE);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES - 1), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board, &None);
        });
    });

    c.bench_function("Algo 3 pieces stacked", |b| {
        let mut board: Board = Board::new(BENCHMARK_SIZE, ALIGNEMENT_NB, CAPTURE_RANGE);
        let player1 = Player::new(Color::Black, PlayerType::Bot(Algorithm::basic_algorithm()));
        let player2 = Player::new(Color::White, PlayerType::Bot(Algorithm::basic_algorithm()));
        let mut players = Players::new(player1, player2, CAPTURE_NB, CAPTURE_RANGE);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 + 1), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 - 1), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board, &None);
        });
    });

    c.bench_function("Algo 6 pieces stacked", |b| {
        let mut board: Board = Board::new(BENCHMARK_SIZE, ALIGNEMENT_NB, CAPTURE_RANGE);
        let player1 = Player::new(Color::Black, PlayerType::Bot(Algorithm::basic_algorithm()));
        let player2 = Player::new(Color::White, PlayerType::Bot(Algorithm::basic_algorithm()));
        let mut players = Players::new(player1, player2, CAPTURE_NB, CAPTURE_RANGE);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 + 1), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 - 1), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 + BENCHMARK_SIZE), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 + 1 + BENCHMARK_SIZE), &mut players);
        let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 - 1 + BENCHMARK_SIZE), &mut players);
        b.iter(|| {
            let new_players = players.clone();
            get_bot_input(&new_players, &board, &None);
        });
    });

    // c.bench_function("Algo 6 pieces non stacked", |b| {
    //     let mut board: Board = Board::new(BENCHMARK_SIZE, ALIGNEMENT_NB, CAPTURE_RANGE);
    //     let player1 = Player::new(Color::Black, PlayerType::Bot);
    //     let player2 = Player::new(Color::White, PlayerType::Bot);
    //     let mut players = Players::new(player1, player2, CAPTURE_NB, CAPTURE_RANGE);
    //     let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2), &mut players);
    //     let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 + 8), &mut players);
    //     let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 - 8), &mut players);
    //     let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 - 2 * BENCHMARK_SIZE), &mut players);
    //     let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 + 8 + BENCHMARK_SIZE), &mut players);
    //     let _ = board.add_value(board.get_input(BENCHMARK_TOTAL_TILES / 2 - 4 * BENCHMARK_SIZE), &mut players);
    //     b.iter(|| {
    //         let new_players = players.clone();
    //         get_bot_input(&new_players, &board);
    //     });
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);