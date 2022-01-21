use std::fmt;
use crate::{
    error::PlacementError,
    color::Color,
    players::{Player, Players},
    config::CONFIG,
};

pub type Input = (usize, usize);

#[derive(PartialEq, Clone, Debug, Copy, Eq, Ord, PartialOrd, Hash)]
pub enum Tile {
    Color(Color),
    Empty,
    OutOfBounds
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty =>  write!(f, "."),
            Tile::Color(color) => write!(f, "{}", color),
            Tile::OutOfBounds => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub struct Board {
    board: Vec<Tile>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            board: vec![Tile::Empty; size * size],
        }
    }

    pub fn reset(&mut self) {
        self.board = vec![Tile::Empty; CONFIG.total_tiles];
    }

    pub fn get_board(&self) -> &Vec<Tile> {
        &self.board
    }

    // pub fn get_size(&self) -> usize {
    //     self.board_length
    // }

    // pub fn get_alignement_nb(&self) -> usize {
    //     self.alignement_nb
    // }

    // pub fn get_capture_range(&self) -> usize {
    //     self.capture_range
    // }

    pub fn from_input(&self, input: Input) -> usize {
        input.0 + input.1 * CONFIG.board_length
    }

    pub fn get_input(&self, i: usize) -> Input {
        (i % CONFIG.board_length, i / CONFIG.board_length)
    }

    fn replace(&mut self, input: Input, tile: Tile) {
        let size = CONFIG.board_length;
        self.board[input.0 + input.1 * size] = tile;
    }

    pub fn get(&self, input: Input) -> Tile {
        self.board[input.0 + input.1 * CONFIG.board_length]
    }

    pub fn get_protected(&self, input: Input) -> Option<&Tile> {
        self.board.get(input.0 + input.1 * CONFIG.board_length)
    }

    pub fn get_ref(&self, input: Input) -> &Tile {
        self.board.get(input.0 + input.1 * CONFIG.board_length).unwrap()
    }

    pub fn get_index(&self, i: usize) -> Tile {
        self.board[i]
    }

    pub fn is_finished(&self, player: Player) -> (bool, Option<Color>) {
        let mut winner = (false, None);
        if !self.board.iter().any(|x| *x == Tile::Empty) {
            winner = (true, None)
        }
        for i in 0..self.board.len() {
            if let Tile::Color(color) = self.board[i] {
                if let (true, Some(actual_winner)) = self.check_victory(i, color, &self.board[i], color == player.get_player_color()) {
                    winner = (true, Some(actual_winner));
                    if actual_winner == player.get_player_color() {
                        return winner
                    }
                }
            }
        }
        winner
    }

    pub fn check_double_free_three(&self, input: Input, color: Color) -> bool {
        let mut count: u8 = 0;
        count += case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x);
        count += case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case1(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case1(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case2(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize); 
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case_size_5(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize);   
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case4(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        false
    }

    pub fn check_add_value(&self, input: Input, players: &Players) -> Result<(), PlacementError> {
        if input.0 > CONFIG.board_length - 1 || input.1 > CONFIG.board_length - 1 {
            return Err(PlacementError::OutOfBounds)
        } else if self.get(input) != Tile::Empty {
            return Err(PlacementError::NotEmpty)
        } else if self.check_double_free_three(input, players.get_current_player().get_player_color()) {
            return Err(PlacementError::DoubleFreeThree)
        }
        Ok(())
    }

    pub fn check_add_value_algo(&self, input: Input, players: &Players) -> Result<(), PlacementError> {
        if self.check_double_free_three(input, players.get_current_player().get_player_color()) {
            return Err(PlacementError::DoubleFreeThree)
        }
        Ok(())
    }

    pub fn add_value_checked(&mut self, input: Input, players: &mut Players) {
        let color = players.get_current_player().get_player_color();
        if CONFIG.capture_range != 0 {
            self.capture(input, players);
        }
        self.replace(input, Tile::Color(color));
    }

    pub fn add_value(&mut self, input: Input, players: &mut Players) -> Result<(), PlacementError> {
        match self.check_add_value(input, players) {
            Ok(()) => {
                self.add_value_checked(input, players);
                Ok(())
            },
            err => err
        }
    }

    fn capture(&mut self, input: Input, players: &mut Players) {
        execute_capture(self, input, players, |x, y| (x as i32 - y) as usize, |x, _| x);
        execute_capture(self, input, players, |x, y| (x as i32 + y) as usize, |x, _| x);
        execute_capture(self, input, players, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize);
        execute_capture(self, input, players, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize);
        execute_capture(self, input, players, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        execute_capture(self, input, players, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        execute_capture(self, input, players, |x, _|  x, |x, y| (x as i32 - y) as usize);
        execute_capture(self, input, players, |x, _| x, |x, y| (x as i32 + y) as usize);
    }

    fn slice(&self, start: usize, length: usize, f_x: fn(usize, usize) -> usize, f_y: fn(usize, usize) -> usize) -> Vec<Tile> {
        let mut ret =  Vec::with_capacity(length);
        for i in 0..length {
            ret.push(self.board[f_x(start, i) + f_y(start, i) * CONFIG.board_length])
        }
        ret
    }

    fn check_victory(&self, i: usize, color: Color, tile: &Tile, captured: bool) -> (bool, Option<Color>) {
        let is_capture_disabled: bool = CONFIG.capture_range == 0;
        if  i % CONFIG.board_length <= CONFIG.board_length - CONFIG.alignement_nb &&
            self.board[i..i + CONFIG.alignement_nb].iter().all(|x| *x == *tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 + y) as usize, |x, _| x))) {
            return (true, Some(color))
        }
        if  i / CONFIG.board_length <= CONFIG.board_length - CONFIG.alignement_nb &&
            self.slice(i, CONFIG.alignement_nb, |start, _| start, |_, x| x).iter().all(|x| *x == *tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, _| x , |x, y| (x as i32 + y) as usize))) {
            return (true, Some(color))
        }
        if  i / CONFIG.board_length <= CONFIG.board_length - CONFIG.alignement_nb &&
            i % CONFIG.board_length <= CONFIG.board_length - CONFIG.alignement_nb &&
            self.slice(i, CONFIG.alignement_nb, |start, x| start + x, |_, x| x).iter().all(|x| *x == *tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize))) {
            return (true, Some(color))
        }
        if  i / CONFIG.board_length <= CONFIG.board_length - CONFIG.alignement_nb &&
            i % CONFIG.board_length >= CONFIG.alignement_nb - 1 &&
            self.slice(i, CONFIG.alignement_nb, |start, x| start - x, |_, x| x).iter().all(|x| *x == *tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize))) {
            return (true, Some(color))
        }
        (false, None)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let legend: String = (0..CONFIG.board_length)
            .collect::<Vec<usize>>()
            .iter()
            .map(|x| *x % 10)
            .fold("".to_string(), |acc, x| format!("{}  {}", acc, x))
            .to_string()
            + "\n0 ";
        let total_tiles_minus_1: usize = CONFIG.total_tiles- 1;
        let my_str: String = self.board.iter().enumerate().fold(legend, |acc, (i, x)| {
            if i == total_tiles_minus_1 {
                return format!("{}{}", acc, x);
            } else if (i + 1) % CONFIG.board_length == 0 {
                return format!("{}{}\n{} ", acc, x, ((i / CONFIG.board_length) + 1) % 10);
            } else {
                return format!("{}{}  ", acc, x);
            }
        });
        write!(f, "{}", my_str)
    }
}

/* PRIVATE */

fn cannot_be_captured(
    board: &Board,
    input: Input,
    color: Color,
    f_x: fn(usize, i32) -> usize,
    f_y: fn(usize, i32) -> usize
) -> bool {
    for i in 0..CONFIG.alignement_nb {
        if f_x(input.0, i as i32) < CONFIG.board_length && f_y(input.1, i as i32) < CONFIG.board_length {
            if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 + y) as usize, |x, _| x) {
                return false
            } else if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize) {
                return false
            } else if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, _| x, |x, y| (x as i32 + y) as usize) {
                return false
            } else if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize) {
                return false
            }
        }
    }
    true
}

fn cannot_be_captured_prime(
    board: &Board,
    input: Input,
    color: Color,
    f_x: fn(usize, i32) -> usize,
    f_y: fn(usize, i32) -> usize
) -> bool {
    let mut vec: Vec<Tile> = vec![board.get(input)];
    for i in 1..=CONFIG.capture_range {
        if f_x(input.0, i as i32) >= CONFIG.board_length || f_y(input.1, i as i32) >= CONFIG.board_length {
            break;
        }
        let tile = board.get((f_x(input.0, i as i32), f_y(input.1, i as i32)));
        if tile == Tile::Color(color.get_inverse_color()) || tile == Tile::Empty {
            vec.push(tile);
            break;
        }
        vec.push(tile);
    }
    vec.reverse();
    for i in 1..=CONFIG.capture_range {
        if f_x(input.0, -(i as i32)) > CONFIG.board_length - 1 || f_y(input.1, -(i as i32)) > CONFIG.board_length - 1 {
            break;
        }
        let tile = board.get((f_x(input.0, -(i as i32)), f_y(input.1, -(i as i32))));
        if tile == Tile::Color(color.get_inverse_color()) || tile == Tile::Empty {
            vec.push(tile);
            break;
        }
        vec.push(tile);
    }
    if vec.len() < 2 {
        return true
    }
    let mut sorted = vec![vec.pop().unwrap(), *vec.get(0).unwrap()];
    sorted.sort();
    vec = (&vec[1..]).to_vec();
    if (
        vec.len(),
        sorted == vec![Tile::Color(color.get_inverse_color()), Tile::Empty]
    ) == (CONFIG.capture_range, true) {
        return false;
    } else {
        return true
    }
}

fn execute_capture(
    board: &mut Board,
    input: Input,
    players: &mut Players,
    f_x: fn(usize, i32) -> usize,
    f_y: fn(usize, i32) -> usize
) {
    if f_x(input.0, (CONFIG.capture_range + 1) as i32) > CONFIG.board_length - 1 || f_y(input.1, (CONFIG.capture_range + 1) as i32) > CONFIG.board_length - 1 {
        return
    }
    let mut selected_vec: Vec<Tile> = Vec::with_capacity(CONFIG.capture_range + 1);
    let color = players.get_current_player().get_player_color();
    for i in 1..=CONFIG.capture_range + 1 {
        selected_vec.push(board.get((f_x(input.0, i as i32), f_y(input.1, i as i32))));
    }
    match (
        Tile::Color(color) == selected_vec.pop().unwrap(),
        selected_vec.iter().all(|x| *x == Tile::Color(color.get_inverse_color()))
    ) {
        (true, true) => {
            for z in 1..=CONFIG.capture_range {
                board.replace((f_x(input.0, z as i32), f_y(input.1, z as i32)), Tile::Empty);
            }
            players.add_capture(color)
        },
        _ => ()
    }
}

/* .-X-XX.*/
fn case1(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 3) > CONFIG.board_length - 1 || f_x(input.0, -1) > CONFIG.board_length - 1 || f_y(input.1, 3) > CONFIG.board_length - 1 || f_y(input.1, -1) > CONFIG.board_length - 1 {
        return 0
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, -1), f_y(input.1, -1)))
    ) {
        (Color::Black, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Empty) => 1,
        (Color::White, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Empty) => 1,
        _ => 0
    }
}

/* .X-X-X. */
fn case2(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 2) > CONFIG.board_length - 1 || f_x(input.0, -2) > CONFIG.board_length - 1 || f_y(input.1, 2) > CONFIG.board_length - 1 || f_y(input.1, -2) > CONFIG.board_length - 1 {
        return 0
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, -1), f_y(input.1, -1))),
        board.get((f_x(input.0, -2), f_y(input.1, -2)))
    ) {
        (Color::Black, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => 1,
        (Color::White, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => 1,
        _ => 0
    }
}

/* .-X-X.X.*/
fn _case3(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 4) > CONFIG.board_length - 1 || f_x(input.0, -1) > CONFIG.board_length - 1 || f_y(input.1, 4) > CONFIG.board_length - 1 || f_y(input.1, -1) > CONFIG.board_length - 1 {
        return 0
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, 4), f_y(input.1, 4))),
        board.get((f_x(input.0, -1), f_y(input.1, -1)))
    ) {
        (Color::Black, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty) => 1,
        (Color::White, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty) => 1,
        _ => 0
    }
}

/* .X-X-.X. */
fn case4(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 3) > CONFIG.board_length - 1 || f_x(input.0, -2) > CONFIG.board_length - 1 || f_y(input.1, 3) > CONFIG.board_length - 1 || f_y(input.1, -2) > CONFIG.board_length - 1 {
        return 0
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, -1), f_y(input.1, -1))),
        board.get((f_x(input.0, -2), f_y(input.1, -2)))
    ) {
        (Color::Black, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => 1,
        (Color::White, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => 1,
        _ => 0
    }
}

/* .-X-.XX. */
fn _case5(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 4) > CONFIG.board_length - 1 || f_x(input.0, -1) > CONFIG.board_length - 1 || f_y(input.1, 4) > CONFIG.board_length - 1 || f_y(input.1, -1) > CONFIG.board_length - 1 {
        return 0
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, 4), f_y(input.1, 4))),
        board.get((f_x(input.0, -1), f_y(input.1, -1)))
    ) {
        (Color::Black, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Empty) => 1,
        (Color::White, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Empty) => 1,
        _ => 0
    }
}

fn case_size_5(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 4) > CONFIG.board_length - 1 || f_x(input.0, -1) > CONFIG.board_length - 1 || f_y(input.1, 4) > CONFIG.board_length - 1 || f_y(input.1, -1) > CONFIG.board_length - 1 {
        return 0
    }
    match (
        board.get((f_x(input.0, -1), f_y(input.1, -1))),
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, 4), f_y(input.1, 4))),
    ) {
        /* .-X-.XX. */
        (Tile::Empty, Color::Black, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => 1,
        (Tile::Empty, Color::White, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => 1,
        /* .-X-X.X. */
        (Tile::Empty, Color::Black, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => 1,
        (Tile::Empty, Color::White, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => 1,
        _ => 0
    }
}