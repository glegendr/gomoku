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
    score: i32
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\nLeaves: {}", self.data.0, self.data.1, self.children.len())
    }
}

impl Tree {
    fn new(data: (Board, Players), input: usize, default_color: Color) -> Tree {
        Tree { children: vec![], input, score: heuristic(&data.0, &data.1, default_color), data}
    }

    fn push(&mut self, child: Tree) {
        self.children.push(child);
    }

    // fn get(&mut self, i: usize) -> &Tree {   
    //     self.children.get(i).unwrap()
    // }

    // fn get_mut(&mut self, i: usize) -> &mut Tree {
    //     self.children.get_mut(i).unwrap()
    // }

    // fn len(&self) -> usize {
    //     self.children.len()
    // }

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
    if board.get_board().iter().all(|x| x == &Tile::Empty) {
        return (board.get_total_tiles() / 2, None)
    }
    let mut handle:Vec<thread::JoinHandle<(i32, usize, Option<Tree>)>> = Vec::new();
    if calculated_tree.is_some() {
        if let Some(tree) = calculated_tree.unwrap().find((&board, &players)) {
            if tree.children.len() > 0 {
                tree.children.iter().for_each(|x| {
                    let mut new_tree = x.clone();
                    handle.push(thread::spawn(move || {
                        let score = minimax(MINMAX_DEPTH - 1, false, i32::MIN, i32::MAX, color, &mut new_tree);
                        // let score = pvs(&mut new_tree, MINMAX_DEPTH - 1, i32::MIN + 1, i32::MAX, color);
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
                                let mut tree = Tree::new((new_board, new_players), i, color);
                                // let score = pvs(&mut tree, MINMAX_DEPTH - 1, i32::MIN, i32::MAX, color);
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

fn play_everything(tree: &mut Tree, default_color: Color) -> &mut Vec<Tree> {
    for i in 0..tree.board().get_board().len() {
        let input = tree.board().get_input(i);
        if tree.board().get_index(i) == Tile::Empty
            && !tree.children.iter().any(|x| x.input == i)
            && pruning_heuristic(input, tree.board())
            && tree.board().check_add_value(input, tree.players()).is_ok() {
            let mut new_board = tree.board().clone();
            let mut new_players = tree.players().clone();
            new_board.add_value_checked(input, &mut new_players);
                new_players.next_player();
                let new_tree = Tree::new((new_board, new_players), i, default_color);
                tree.push(new_tree);
        }
    }
    tree.children.sort_by(|a, b| b.score.cmp(&a.score));
    &mut tree.children
}

fn minimax(depth: usize, maximizing_player: bool, alpha: i32, beta: i32, default_color: Color, tree: &mut Tree) -> i32 {
    if depth == 0 || tree.board().is_finished(tree.players().get_current_player()).0 || tree.players().is_finished().0 {
        return tree.score
    } else if maximizing_player {
        let mut value: i32 = i32::MIN;
        let mut new_alpha = alpha;
            for mut child in play_everything(tree, default_color) {
                let ret_minimax = minimax(depth - 1, false, new_alpha, beta, default_color, &mut child);
                value = max(value, ret_minimax);
                if value >= beta {
                    return value
                }
                new_alpha = max(new_alpha, value);
            }
        return value
    } else {
        let mut value: i32 = i32::MAX;
        let mut new_beta = beta;
            for mut child in play_everything(tree, default_color) {
                let ret_minimax = minimax(depth - 1, true, alpha, new_beta, default_color, &mut child);
                value = min(value, ret_minimax);
                if alpha >= value {
                    return value
                }
                new_beta = min(new_beta, value);
            }
        return value
    }
}

fn pvs(tree: &mut Tree, depth: usize, mut alpha: i32, beta: i32, color: Color) -> i32 {
    if depth == 0 || tree.board().is_finished(tree.players().get_current_player()).0 || tree.players().is_finished().0 {
        return tree.score
    }
    let mut is_first = true;
    for mut child in play_everything(tree, color) {
        let mut score;
        if is_first {
            is_first = false;
            score = -pvs(&mut child, depth - 1, -beta, -alpha, color);
        } else {
            score = -pvs(&mut child, depth - 1, -alpha - 1, -alpha, color);
            if alpha < score && score < beta {
                score = -pvs(&mut child, depth - 1, -beta, -score, color);
            }
        }
        alpha = max(alpha, score);
        if alpha >= beta {
            break
        }
    }
    return alpha
}