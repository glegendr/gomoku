use crate::board::*;
use crate::players::*;
use crate::color::*;
use std::cmp::{min, max};
use std::thread;

const MINMAX_DEPTH: usize = 5;

pub fn get_bot_input(players: &Players, board: &Board) -> Input {
    let index = play_everything_and_compute(board.clone(), *players);
    get_input(index)
}

fn play_everything_and_compute(board: Board, players: Players) -> usize {
    let mut handle = Vec::new();
    for (i, child) in board.get_board().iter().enumerate() {
        if *child == Tile::Empty {
            if pruning_heuristic(get_input(i), &board, players) {
                let mut new_board = board.clone();
                let mut new_players = players.clone();
                handle.push(thread::spawn(move || {
                    match new_board.add_value(get_input(i), &mut new_players) {
                            Err(_) => return (i32::MIN, i),
                            _ => {
                                let mut alpha = i32::MIN;
                                let mut beta = i32::MAX;
                                let score = minimax(new_board, MINMAX_DEPTH - 1 , false, &mut alpha, &mut beta, new_players);
                                return (score, i)
                            }
                        }
                    })
                );
            }
        }
    };
    let mut values = Vec::new();
    for child in handle {
        values.push(child.join().unwrap());
    }
    values.iter().fold((i32::MIN, 0), |acc, x| {
        if x.0 > acc.0 {
            *x
        } else {
            acc
        }
    }).1
}

fn play_everything(board: Board, players: Players) -> Vec<(Board, Players)> {
    let ret = board.get_board().iter().enumerate().fold(Vec::new(), |mut acc, (i, x)| {
        if *x == Tile::Empty {
            if pruning_heuristic(get_input(i), &board, players) {
                let mut new_board = board.clone();
                let mut new_players = players.clone();
                match new_board.add_value(get_input(i), &mut new_players) {
                    Err(_) => (),
                    _ => acc.push((new_board, new_players)),
                }
            }
        }
        acc
    });
    if ret.len() > 0 {
        ret
    } else  {
        let index = board.get_board().iter().enumerate().fold(0, |acc, (i, x)| {
            if *x == Tile::Empty {
                return i
            }
            acc
        });
        let mut new_board = board.clone();
        let mut new_players = players.clone();
        let ret = match new_board.add_value(get_input(index), &mut new_players) {
            Err(_) => (new_board, new_players),
            _ => (new_board, new_players),
        };
        vec![ret]
    }
} // 4.7168902s

fn minimax(node: Board, depth: usize, maximizing_player: bool, alpha: &mut i32, beta: &mut i32, players: Players) -> i32 {
    if depth == 0 { //|| node is a terminal node {
        return close_heuristic(node, players.get_current_player().get_player_color()) // Heuristique
    } else if maximizing_player {
         let mut value: i32 = i32::MIN;
        for child in play_everything(node, players) {
            value = max(value, minimax(child.0, depth - 1, false, alpha, beta, child.1));
            if *beta < value {
                return value
            }
            *alpha = max(*alpha, value);
        }
        return value
    } else { // (* minimizing player *)
        let mut value: i32 = i32::MAX;
        for child in play_everything(node, players) {
            value = min(value, minimax(child.0, depth - 1, true, alpha, beta, child.1));
            if *alpha >= value {
                return value
            }
            *beta = min(*beta, value);
        }
        return value
    }
}

/* HEURISTICS */

fn get_distance(board: &Board, distance: i32, input: Input) -> bool {
    for y in -distance..=distance {
        if (input.1 as i32) + y < 0 {
            continue;
        } else if (input.1 as i32) + y >= BOARD_LENGTH as i32 {
            break;
        }
        for x in -distance..=distance {
            if (input.0 as i32) + x < 0 || (y != -distance && y != distance && x != -distance && x != distance){
                continue;
            } else if (input.0 as i32) + x >= BOARD_LENGTH as i32 {
                break;
            }
            if let Tile::Color(_) = board.get((((input.0 as i32) + x) as usize, ((input.1 as i32) + y) as usize)) {
                return true
            }
            
        }

    }
    false
}

fn close_heuristic(board: Board, color: Color) -> i32 {
    if board.get_board().iter().any(|x| *x == Tile::Color(color.get_inverse_color())) {
        board.get_board().iter().enumerate().map(|(i, x)| {
            if let Tile::Color(new_color) = x {
                if *new_color == color {
                    let input = get_input(i);
                    for distance in 1.. {
                        if get_distance(&board, distance, input) {
                            return (((BOARD_LENGTH as i32 - 1) * 2) - (distance as i32)) as usize
                        }
                    }
                }
            }
            ((BOARD_LENGTH as i32 - 1) * 2) as usize
        }).sum::<usize>() as i32
    } else {
        (BOARD_LENGTH as i32 - 1) * 2
    }
}

fn pruning_heuristic(input: Input, board: &Board, players: Players) -> bool {
    for distance in 1..=1 {
        if get_distance(board, distance, input) {
            return true
        }
    }
    false
}