use crate::board::*;
use crate::players::*;
use crate::color::*;
use crate::heuristic::*;
use std::{
    cmp::{min, max},
    thread,
    fmt,
    sync::{RwLock, Arc}
};

const AVERAGE_PRUNNING: i32 = i32::MAX / 27;

#[derive(Debug, Clone)]
pub struct Tree {
    data: (Board, Players),
    input: usize,
    children: Vec<Tree>,
    score: i32
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-----------\nInput: {}\nLeaves: {}", self.input, self.children.len())
    }
}

impl Tree {
    fn new(data: (Board, Players), input: usize, default_color: Color) -> Tree {
        Tree { children: vec![], input, score: heuristic(&data.0, &data.1, default_color), data}
    }

    fn push(&mut self, child: Tree) {
        self.children.push(child);
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
            if &self.children[i].data.0 == data.0 && &self.children[i].data.1 == data.1 {
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
pub fn get_bot_input(players: Players, board: &Board, calculated_tree: &Option<Tree>, depth: usize) -> (Input, Option<Tree>) {
    let color = players.get_current_player().get_player_color();
    let lock: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));

    if board.get_board().iter().all(|x| x == &Tile::Empty) {
        return ((board.get_size() / 2, board.get_size() / 2), None)
    }
    if calculated_tree.is_some() {
        if let Some(tree) = calculated_tree.as_ref().unwrap().find((&board, &players)) {
            if tree.children.len() > 0 {
                if let Some(finished_tree) = tree.children.iter().find(|x| x.score == i32::MAX) {
                    return (board.get_input(finished_tree.input), Some(finished_tree.clone()))
                }
                let mut handle:Vec<thread::JoinHandle<(i32, usize, Option<Tree>)>> = Vec::new();
                let end = if tree.children.len() > depth + 2 {
                    depth + 3
                    } else {
                        tree.children.len()
                    };
                
                for i in 0..end {
                    let mut new_tree = tree.children[i].clone();
                    let c_lock = Arc::clone(&lock);
                    handle.push(thread::spawn(move || {
                        let score = match players.get_current_player().get_player_type() {
                            PlayerType::Bot(Algorithm::Minimax) => minimax((depth - 1, depth), false, i32::MIN, i32::MAX, color, &mut new_tree, &c_lock),
                            PlayerType::Bot(Algorithm::Pvs) => pvs(&mut new_tree, depth - 1, i32::MIN + 1, i32::MAX, color),
                            _ => unreachable!()
                        };
                        if score >= AVERAGE_PRUNNING && depth >= 5 {
                            let mut mut_lock = c_lock.write().unwrap();
                            *mut_lock = true;
                        }
                        return (score, new_tree.input, Some(new_tree))
                    }));
                }
                let mut values = Vec::new();
                for child in handle {
                    values.push(child.join().unwrap());
                }
                if values.len() > 0 {
                    let ret = values.iter().fold((i32::MIN, 0, None), |acc, x| {
                        if x.0 >= acc.0 {
                            (x.0, x.1, x.2.clone())
                        } else {
                            acc
                        }
                    });
                    return (board.get_input(ret.1), ret.2)
                }
            }
        }
    }
    let mut handle:Vec<thread::JoinHandle<(i32, usize, Option<Tree>)>> = Vec::new();
    for (i, child) in board.get_board().iter().enumerate() {
        if *child == Tile::Empty {
            let input = board.get_input(i);
            if pruning_heuristic(input, &board) && board.check_add_value_algo(input, &players).is_ok() {
                let mut new_board = board.clone();
                let mut new_players = players.clone();
                let c_lock = Arc::clone(&lock);
                handle.push(thread::spawn(move || {
                    new_board.add_value_checked(input, &mut new_players);
                    new_players.next_player();
                    let mut tree = Tree::new((new_board, new_players), i, color);
                    let score = match players.get_current_player().get_player_type() {
                        PlayerType::Bot(Algorithm::Minimax) => minimax((depth - 1, depth), false, i32::MIN, i32::MAX, color, &mut tree, &c_lock),
                        PlayerType::Bot(Algorithm::Pvs) => pvs(&mut tree, depth - 1, i32::MIN + 1, i32::MAX, color),
                        _ => unreachable!()
                    };
                    if score >= AVERAGE_PRUNNING && depth >= 5 {
                        let mut mut_lock = c_lock.write().unwrap();
                        *mut_lock = true;
                    }
                    return (score, i, Some(tree))
                }));
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
    (board.get_input(ret.1), ret.2)
}

fn play_everything(tree: &mut Tree, default_color: Color, is_minimax: bool) -> &mut Vec<Tree> {
    for i in 0..tree.board().get_board().len() {
        let input = tree.board().get_input(i);
        if tree.board().get_index(i) == Tile::Empty
            && !tree.children.iter().any(|x| x.input == i)
            && pruning_heuristic(input, tree.board())
            && tree.board().check_add_value_algo(input, tree.players()).is_ok() {
            let mut new_board = tree.board().clone();
            let mut new_players = tree.players().clone();
            new_board.add_value_checked(input, &mut new_players);
                new_players.next_player();
                tree.push(Tree::new((new_board, new_players), i, default_color));
        }
    }
    if tree.data.1.get_current_player().get_player_color() == default_color {
        tree.children.sort_by(|a, b| b.score.cmp(&a.score));
    } else if is_minimax {
        tree.children.sort_by(|a, b| a.score.cmp(&b.score));
    } else {
        tree.children.sort_by(|a, b| b.score.cmp(&a.score));
    }
    &mut tree.children
}

fn minimax(depth: (usize, usize), maximizing_player: bool, mut alpha: i32, mut beta: i32, default_color: Color, tree: &mut Tree, lock: &RwLock<bool>) -> i32 {
    if depth.0 == 0 || tree.score == i32::MAX || tree.score == i32::MIN || *lock.read().unwrap() {
        return tree.score
    }
    let childs = play_everything(tree, default_color, true);
    if maximizing_player {
        let mut value: i32 = i32::MIN;
        let end = if childs.len() > depth.0 + 2 {
                    depth.0 + 3
                } else {
                    childs.len()
                };
        for i in 0..end {
            value = max(value, minimax((depth.0 - 1, depth.1), false, alpha, beta, default_color, &mut childs[i], lock));
            if value >= beta || (value >= AVERAGE_PRUNNING && depth.1 >= 5) {
                return value
            }
            alpha = max(alpha, value);
        }
        return value
    } else {
        let mut value: i32 = i32::MAX;
        for i in 0..childs.len() {
            value = min(value, minimax((depth.0 - 1, depth.1), true, alpha, beta, default_color, &mut childs[i], lock));
            if alpha >= value || (value <= -AVERAGE_PRUNNING && depth.1 >= 5) {
                return value
            }
            beta = min(beta, value);
        }
        return value
    }
}

fn pvs(tree: &mut Tree, depth: usize, mut alpha: i32, beta: i32, color: Color) -> i32 {
    if depth == 0 || tree.score == i32::MAX || tree.score == i32::MIN {
        if tree.players().get_current_player().get_player_color() == color {
            return tree.score
        } else {
            return -tree.score
        }
    }
    let childs = play_everything(tree, tree.players().get_current_player().get_player_color().get_inverse_color(), false);
    for i in 0..childs.len() {
        let mut score;
        if i == 0 {
            score = -pvs(&mut childs[i], depth - 1, -beta, -alpha, color);
        } else {
            score = -pvs(&mut childs[i], depth - 1, -alpha - 1, -alpha, color);
            if alpha < score && score < beta {
                score = -pvs(&mut childs[i], depth - 1, -beta, -score, color);
            }
        }
        alpha = max(alpha, score);
        if alpha >= beta {
            break
        }
    }
    return alpha
}