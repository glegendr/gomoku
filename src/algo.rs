use crate::board::*;
use crate::players::*;
use crate::color::*;
use crate::heuristic::*;
use std::cmp::{min, max};
use std::thread;

const MINMAX_DEPTH: usize = 5;

pub fn get_bot_input(players: &Players, board: &Board) -> Input {
    let index = play_everything_and_compute(board.clone(), *players, players.get_current_player().get_player_color());
    //let index = minimax(board.clone(), MINMAX_DEPTH, true, i32::MIN, i32::MAX, *players, players.get_current_player().get_player_color());
    //println!("{}", index.0);
    board.get_input(index)
}

fn play_everything_and_compute(board: Board, players: Players, color: Color) -> usize {
    let mut handle = Vec::new();
    for (i, child) in board.get_board().iter().enumerate() {
        if *child == Tile::Empty {
            if pruning_heuristic(board.get_input(i), &board) {
                let mut new_board = board.clone();
                let mut new_players = players.clone();
                handle.push(thread::spawn(move || {
                    match new_board.add_value(new_board.get_input(i), &mut new_players) {
                            Err(_) => return (i32::MIN, i),
                            _ => {
                                new_players.next_player();
                                let score = minimax(new_board, MINMAX_DEPTH - 1 , false, i32::MIN, i32::MAX, new_players, color);
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
    //println!("{:?}", values.iter().map(|x| (x.0, board.get_input(x.1))).collect::<Vec<(i32, Input)>>());
    values.iter().fold((i32::MIN, 0), |acc, x| {
        if x.0 >= acc.0 {
            *x
        } else {
            acc
        }
    }).1
}

fn play_everything(board: Board, players: Players) -> Vec<(Board, Players)> {
    let mut ret = Vec::new();
    board.get_board().iter().enumerate().for_each(|(i, x)| {
        if *x == Tile::Empty {
            if pruning_heuristic(board.get_input(i), &board) {
                let mut new_board = board.clone();
                let mut new_players = players.clone();
                match new_board.add_value(board.get_input(i), &mut new_players) {
                    Err(_) => (),
                    _ => {
                        new_players.next_player();
                        ret.push((new_board, new_players))
                    },
                }
            }
        }
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
        let ret = match new_board.add_value(board.get_input(index), &mut new_players) {
            Err(_) => (new_board, new_players),
            _ => (new_board, new_players),
        };
        vec![ret]
    }
} // 4.7168902s

fn minimax(node: Board, depth: usize, maximizing_player: bool, alpha: i32, beta: i32, players: Players, default_color: Color) -> i32 {
    if depth == 0 || node.is_finished(players.get_current_player()).0 || players.is_finished().0 {
        let heu = heuristic(node, players, default_color);
        return heu// heu.to_string()
    } else if maximizing_player {
         let mut value: i32 = i32::MIN;
         let mut new_alpha = alpha;
         //let mut str_ret = "(".to_owned();
        for child in play_everything(node, players) {
            let ret_minimax = minimax(child.0, depth - 1, false, new_alpha, beta, child.1, default_color);
            //str_ret = format!("{}max{},", str_ret, ret_minimax.1);
            // println!("MAX: {} {} {}", value, ret_minimax, depth);
            value = max(value, ret_minimax);
            if value >= beta {
                //str_ret.pop();
                //str_ret = format!("{}){}", str_ret, value);
                return value//, str_ret)
            }
            new_alpha = max(new_alpha, value);
        }
        //str_ret.pop();
        //str_ret = format!("{}){}", str_ret, value);
        return value//, str_ret)
    } else {
        let mut value: i32 = i32::MAX;
        let mut new_beta = beta;
        //let mut str_ret = "(".to_owned();
        for child in play_everything(node, players) {
            let ret_minimax = minimax(child.0, depth - 1, true, alpha, new_beta, child.1, default_color);
            //str_ret = format!("{}min{},", str_ret, ret_minimax.1);
            // println!("MIN: {} {} {}", value, ret_minimax, depth);
            value = min(value, ret_minimax);
            if alpha >= value {
                //str_ret.pop();
                //str_ret = format!("{}){}", str_ret, value);
                return value//, str_ret)
            }
            new_beta = min(new_beta, value);
        }
        //str_ret.pop();
        //str_ret = format!("{}){}", str_ret, value);
        return value//, str_ret)
    }
}