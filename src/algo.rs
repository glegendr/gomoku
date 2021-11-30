use crate::board::*;
use crate::players::*;
use crate::color::*;
use crate::heuristic::*;
use std::cmp::{min, max};
use std::thread;
use std::fmt;

const MINMAX_DEPTH: usize = 5;

#[derive(Debug, Clone)]
pub struct Tree {
    data: (Board, Players),
    input: usize,
    children: Vec<Tree>,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\nLeaves: {}", self.data.0, self.data.1, self.children.len())
    }
}

impl Tree {
    fn new(data: (Board, Players), input: usize) -> Tree {
        Tree { data: data, children: vec![], input}
    }

    fn push(&mut self, child: Tree) {
        self.children.push(child);
    }

    fn get_mut(&mut self, i: usize) -> &mut Tree {
        self.children.get_mut(i).unwrap()
    }

    fn len(&self) -> usize {
        self.children.len()
    }

    fn _leaves(&self) -> usize {
        let mut ret = 1;
        self.children.iter().for_each(|x| {
            ret += x._leaves();
        });
        ret
    }

    fn find(&self, data: (&Board, &Players)) -> Option<&Tree> {
        for i in 0..self.children.len() {
            if self.children[i].data.0 == *data.0 && self.children[i].data.1 == *data.1 {
                return Some(&self.children[i])
            }
        }
        None
    }

    fn board(&self) -> &Board {
        &self.data.0
    }

    fn players(&self) -> &Players {
        &self.data.1
    }

}
pub fn get_bot_input(players: &Players, board: &Board, tree: Option<Tree>) -> (Input, Option<Tree>) {
    let (index, calculated_tree) = play_everything_and_compute(board.clone(), *players, players.get_current_player().get_player_color(), tree);
    (board.get_input(index), calculated_tree)
}

fn play_everything_and_compute(board: Board, players: Players, color: Color, calculated_tree: Option<Tree>) -> (usize, Option<Tree>) {
    let mut handle:Vec<thread::JoinHandle<(i32, usize, Option<Tree>)>> = Vec::new();
    if calculated_tree.is_some() {
        if let Some(tree) = calculated_tree.unwrap().find((&board, &players)) {
            if tree.children.len() > 0 {
                tree.children.iter().for_each(|x| {
                    let mut new_tree = x.clone();
                    handle.push(thread::spawn(move || {
                        let score = minimax(1, false, i32::MIN, i32::MAX, color, &mut new_tree);
                    return (score, new_tree.input, Some(new_tree))
                    }));
                });
                let mut values = Vec::new();
                for child in handle {
                    values.push(child.join().unwrap());
                }
                let ret = values.iter().fold((i32::MIN, 0, None), |acc, x| {
                    if x.0 >= acc.0 {
                        (*x).clone()
                    } else {
                        acc
                    }
                });
                return (ret.1, ret.2)
            }
        }
    }
    for (i, child) in board.get_board().iter().enumerate() {
        if *child == Tile::Empty {
            if pruning_heuristic(board.get_input(i), &board) {
                let mut new_board = board.clone();
                let mut new_players = players.clone();
                handle.push(thread::spawn(move || {
                    match new_board.add_value(new_board.get_input(i), &mut new_players) {
                            Err(_) => return (i32::MIN, i, None),
                            _ => {
                                new_players.next_player();
                                let mut tree = Tree::new((new_board, new_players), i);
                                let score = minimax(MINMAX_DEPTH - 1 , false, i32::MIN, i32::MAX, color, &mut tree);
                                return (score, i, Some(tree))
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
    let ret = values.iter().fold((i32::MIN, 0, None), |acc, x| {
        if x.0 >= acc.0 {
            (*x).clone()
        } else {
            acc
        }
    });
    (ret.1, ret.2)
}

fn play_everything(tree: &mut Tree) -> Vec<usize> {
    let mut ret = Vec::new();
    for i in 0..tree.board().get_board().len() {
        let input = tree.board().get_input(i);
        if tree.board().get_index(i) == Tile::Empty
            && pruning_heuristic(input, tree.board())
            && tree.board().check_add_value(input, tree.players()).is_ok() {
            let mut new_board = tree.board().clone();
            let mut new_players = tree.players().clone();
            new_board.add_value_checked(input, &mut new_players);
            new_players.next_player();
            let new_tree = Tree::new((new_board, new_players), i);
            tree.push(new_tree);
            ret.push(tree.len())
        }
    }
    ret
}

fn minimax(depth: usize, maximizing_player: bool, alpha: i32, beta: i32, default_color: Color, tree: &mut Tree) -> i32 {
    if depth == 0 || tree.board().is_finished(tree.players().get_current_player()).0 || tree.players().is_finished().0 {
        return heuristic(tree.board(), tree.players(), default_color)
    } else if maximizing_player {
        let mut value: i32 = i32::MIN;
        let mut new_alpha = alpha;
        if tree.children.len() > 0 {
            for child in &mut tree.children {
                let ret_minimax = minimax(depth, false, new_alpha, beta, default_color, child);
                value = max(value, ret_minimax);
                if value >= beta {
                    return value
                }
                new_alpha = max(new_alpha, value);
            }
        } else {
            for child in play_everything(tree) {
                let ret_minimax = minimax(depth - 1, false, new_alpha, beta, default_color, tree.get_mut(child - 1));
                value = max(value, ret_minimax);
                if value >= beta {
                    return value
                }
                new_alpha = max(new_alpha, value);
            }
        }
        return value
    } else {
        let mut value: i32 = i32::MAX;
        let mut new_beta = beta;
        if tree.children.len() > 0 {
            for child in &mut tree.children {
                let ret_minimax = minimax(depth, true, alpha, new_beta, default_color, child);
                value = min(value, ret_minimax);
                if alpha >= value {
                    return value
                }
                new_beta = min(new_beta, value);
            }
        } else {
            for child in play_everything(tree) {
                let ret_minimax = minimax(depth - 1, true, alpha, new_beta, default_color, tree.get_mut(child - 1));
                value = min(value, ret_minimax);
                if alpha >= value {
                    return value
                }
                new_beta = min(new_beta, value);
            }
        }
        return value
    }
}