use crate::board::*;
use crate::players::*;
use crate::color::*;
use std::cmp::{min, max};
use std::thread;

pub fn get_bot_input(players: &Players, board: &Board) -> Input {
    let index = play_everything_and_compute(board.clone(), *players);
    get_input(index)
}

fn play_everything_and_compute(board: Board, players: Players) -> usize {
    let mut handle = Vec::new();
    for (i, child) in board.get_board().iter().enumerate() {
        if *child == Tile::Empty {
            let mut new_board = board.clone();
            let mut new_players = players.clone();
            handle.push(thread::spawn(move || {
                match new_board.add_value(get_input(i), &mut new_players) {
                            Err(_) => return (i32::MIN, i),
                            _ => {
                                let mut alpha = i32::MIN;
                                let mut beta = i32::MAX;
                                let score = minimax(new_board, 5, true, &mut alpha, &mut beta, new_players);
                                return (score, i)
                            }
                        }
                    }
                )
            );
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
            let mut founded = false;
            let input = get_input(i);
            for distance in 1..=2 {
                if get_distance(&board, players.get_current_player().get_player_color().get_inverse_color(), distance, input) {
                    founded = true;
                    break;
                }
            }
            if founded {
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

fn get_distance(board: &Board, color: Color, distance: usize, input: Input) -> bool {
    let mut loop_index = 4 * distance;
    let mut x: (i32, i32) = (distance as i32, -1);
    let mut y: (i32, i32) = (0, -1);
    while loop_index > 0 {
        loop_index -= 1;
        if (((input.0 as i32) + x.0) as usize) < BOARD_LENGTH && (((input.1 as i32) + y.0) as usize) < BOARD_LENGTH {
            if let Tile::Color(new_color) = board.get(((((input.0 as i32) + x.0) as usize), (((input.1 as i32) + y.0) as usize))) {
                if new_color == color {
                    return true
                }
            }
        }
        x.0 = x.0 + x.1;
        y.0 = y.0 + y.1;
        if x.0 == -(distance as i32) {
            x.1 = 1;
        } else if x.0 == distance as i32 {
            x.1 = -1;
        }
        if y.0 == -(distance as i32) {
            y.1 = 1;
        } else if y.0 == distance as i32 {
            y.1 = -1;
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
                        if get_distance(&board, color.get_inverse_color(), distance, input) {
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