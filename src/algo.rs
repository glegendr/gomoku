use crate::board::*;
use crate::players::*;
use crate::color::*;
use std::cmp::{min, max};
use std::thread;

const MINMAX_DEPTH: usize = 8;

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
                    }
                )
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

fn pruning_heuristic(input: Input, board: &Board, players: Players) -> bool {
    for distance in 1..=2 {
        if get_distance(board, players.get_current_player().get_player_color().get_inverse_color(), distance, input) {
            return true
        }
    }
    false
} 

/* TESTS */

#[derive(PartialEq, Debug)]
struct ChaindedList<'a> {
    nb: usize,
    parent: Option<&'a ChaindedList<'a>>,
}

impl<'a> ChaindedList<'a> {
    fn push_son(&self, nb: usize) -> ChaindedList {
        ChaindedList{nb: nb, parent: Some(self)}
    }
}

fn create_childs<'a>(chain: &'a ChaindedList<'a>) -> Vec<ChaindedList<'a>> {
    let mut ret: Vec<ChaindedList<'a>> = Vec::new();
    for i in 0..10 {
        ret.push(chain.push_son(i));
    }
    ret
}

fn test<'a>(depth: usize, chain: &'a ChaindedList<'a>) -> i32 {
    if depth == 0 {
        println!("{:?}", chain);
        return 1;
    } else {
        for mut child in create_childs(chain) {
            //*chain = ChaindedList{nb: depth + i, parent: Some(chain)};
            // chain = &mut value;
            //*chain = value;
            test(depth - 1, &mut child);
        }
        return 2
    }
}

// fn test_play_everything<'a>(board: Board, players: Players, chain: &mut ChaindedList<'a>) -> Vec<ChaindedList<'a>>{
//     players.next_player();
//     let ret = board.get_board().iter().enumerate().fold(Vec::new(), |mut acc, (i, x)| {
//         if *x == Tile::Empty {
//             // if pruning_heuristic(get_input(i), &board, players) {
//             //     let mut new_board = board.clone();
//             //     let mut new_players = players.clone();
//             //     match new_board.add_value(get_input(i), &mut new_players) {
//             //         Err(_) => (),
//             //         _ => acc.push((new_board, new_players)),
//             //     }
//             // }
//             acc.push(chain.push_son(i))
//         }
//         acc
//     });
//     if ret.len() > 0 {
//         ret
//     } else  {
//         let index = board.get_board().iter().enumerate().fold(0, |acc, (i, x)| {
//             if *x == Tile::Empty {
//                 return i
//             }
//             acc
//         });
//         let z = chain.push_son(index);
//         let w = Vec::new();
//         w.push(z);
//         w
//     }
// } // 4.7168902s

// fn test_minimax(node: Board, depth: usize, maximizing_player: bool, alpha: &mut i32, beta: &mut i32, players: Players, chain: &mut ChaindedList) -> i32 {
//     if depth == 0 { //|| node is a terminal node {
//         return test_close_heuristic(node, players, chain) // Heuristique
//     } else if maximizing_player {
//          let mut value: i32 = i32::MIN;
//         for child in test_play_everything(node, players, chain) {
//             value = max(value, test_minimax(child.0, depth - 1, false, alpha, beta, child.1));
//             if *beta < value {
//                 return value
//             }
//             *alpha = max(*alpha, value);
//         }
//         return value
//     } else { // (* minimizing player *)
//         let mut value: i32 = i32::MAX;
//         for child in play_everything(node, players) {
//             value = min(value, test_minimax(child.0, depth - 1, true, alpha, beta, child.1));
//             if *alpha >= value {
//                 return value
//             }
//             *beta = min(*beta, value);
//         }
//         return value
//     }
// }
