use std::fmt;

use crate::error::PlacementError;
use crate::color::Color;
use crate::players::{Players, Player};

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
    board_length: usize,
    alignement_nb: usize,
    capture_range: usize
}

impl Board {
    pub fn new(size: usize, alignement_nb: usize, capture_range: usize) -> Board {
        Board {
            board: vec![Tile::Empty; size * size],
            board_length: size,
            alignement_nb,
            capture_range
        }
    }

    pub fn reset(&mut self) {
        self.board = vec![Tile::Empty; self.get_total_tiles()];
    }

    pub fn get_board(&self) -> &Vec<Tile> {
        &self.board
    }

    pub fn get_size(&self) -> usize {
        self.board_length
    }

    pub fn get_alignement_nb(&self) -> usize {
        self.alignement_nb
    }

    pub fn get_capture_range(&self) -> usize {
        self.capture_range
    }

    pub fn from_input(&self, input: Input) -> usize {
        input.0 + input.1 * self.get_size()
    }

    pub fn get_input(&self, i: usize) -> Input {
        (i % self.get_size(), i / self.get_size())
    }

    pub fn get_total_tiles(&self) -> usize {
        self.board_length * self.board_length
    }

    fn replace(&mut self, input: Input, tile: Tile) {
        let size = self.get_size();
        self.board[input.0 + input.1 * size] = tile;
    }

    pub fn get(&self, input: Input) -> Tile {
        self.board[input.0 + input.1 * self.get_size()]
    }

    pub fn get_protected(&self, input: Input) -> Option<&Tile> {
        self.board.get(input.0 + input.1 * self.get_size())
    }

    pub fn get_ref(&self, input: Input) -> &Tile {
        self.board.get(input.0 + input.1 * self.get_size()).unwrap()
    }

    pub fn get_index(&self, i: usize) -> Tile {
        self.board[i]
    }

    pub fn is_finished(&self, player: &Player) -> (bool, Option<Color>) {
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
        count += double_free_three_cases(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        count += double_free_three_cases(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += double_free_three_cases(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += double_free_three_cases(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize); 
        if count >= 2 {
            return true
        }
        false
    }

    pub fn check_add_value(&self, input: Input, players: &Players) -> Result<(), PlacementError> {
        let color = players.get_current_player().get_player_color();
        if input.0 > self.get_size() - 1 || input.1 > self.get_size() - 1 {
            return Err(PlacementError::OutOfBounds)
        } else if self.get_ref(input) != &Tile::Empty {
            return Err(PlacementError::NotEmpty)
        } else if self.check_double_free_three(input, color) {
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
        if self.get_capture_range() != 0 {
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
            ret.push(self.board[f_x(start, i) + f_y(start, i) * self.get_size()])
        }
        ret
    }

    fn check_victory(&self, i: usize, color: Color, tile: &Tile, captured: bool) -> (bool, Option<Color>) {
        let is_capture_disabled: bool = self.get_capture_range() == 0;
        if  i % self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            self.board[i..i + self.get_alignement_nb()].iter().all(|x| x == tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 + y) as usize, |x, _| x))) {
            return (true, Some(color))
        }
        if  i / self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            self.slice(i, self.get_alignement_nb(), |start, _| start, |_, x| x).iter().all(|x| x == tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, _| x , |x, y| (x as i32 + y) as usize))) {
            return (true, Some(color))
        }
        if  i / self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            i % self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            self.slice(i, self.get_alignement_nb(), |start, x| start + x, |_, x| x).iter().all(|x| x == tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize))) {
            return (true, Some(color))
        }
        if  i / self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            i % self.get_size() >= self.get_alignement_nb() - 1 &&
            self.slice(i, self.get_alignement_nb(), |start, x| start - x, |_, x| x).iter().all(|x| x == tile) &&
            (is_capture_disabled || (captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize))) {
            return (true, Some(color))
        }
        (false, None)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let legend: String = (0..self.get_size())
            .collect::<Vec<usize>>()
            .iter()
            .map(|x| *x % 10)
            .fold("".to_string(), |acc, x| format!("{}  {}", acc, x))
            .to_string()
            + "\n0 ";
        let total_tiles_minus_1: usize = self.get_total_tiles() - 1;
        let my_str: String = self.board.iter().enumerate().fold(legend, |acc, (i, x)| {
            if i == total_tiles_minus_1 {
                return format!("{}{}", acc, x);
            } else if (i + 1) % self.get_size() == 0 {
                return format!("{}{}\n{} ", acc, x, ((i / self.get_size()) + 1) % 10);
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
    for i in 0..board.get_alignement_nb() {
        if f_x(input.0, i as i32) < board.get_size() && f_y(input.1, i as i32) < board.get_size() {
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
    let mut vec: Vec<&Tile> = vec![board.get_ref(input)];
    for i in 1..=board.get_capture_range() {
        if f_x(input.0, i as i32) >= board.get_size() || f_y(input.1, i as i32) >= board.get_size() {
            break;
        }
        let tile = board.get_ref((f_x(input.0, i as i32), f_y(input.1, i as i32)));
        if tile == &Tile::Color(color.get_inverse_color()) || tile == &Tile::Empty {
            vec.push(tile);
            break;
        }
        vec.push(tile);
    }
    vec.reverse();
    for i in 1..=board.get_capture_range() {
        if f_x(input.0, -(i as i32)) > board.get_size() - 1 || f_y(input.1, -(i as i32)) > board.get_size() - 1 {
            break;
        }
        let tile = board.get_ref((f_x(input.0, -(i as i32)), f_y(input.1, -(i as i32))));
        if tile == &Tile::Color(color.get_inverse_color()) || tile == &Tile::Empty {
            vec.push(tile);
            break;
        }
        vec.push(tile);
    }
    if vec.len() < 2 {
        return true
    }
    let mut sorted = vec![vec.pop().unwrap(), vec[0]];
    sorted.sort();
    vec = (&vec[1..]).to_vec();
    if (
        vec.len(),
        sorted == vec![&Tile::Color(color.get_inverse_color()), &Tile::Empty]
    ) == (board.get_capture_range(), true) {
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
    if f_x(input.0, (board.get_capture_range() + 1) as i32) > board.get_size() - 1 || f_y(input.1, (board.get_capture_range() + 1) as i32) > board.get_size() - 1 {
        return
    }
    let mut selected_vec: Vec<(Tile, Input)> = Vec::with_capacity(board.get_capture_range() + 1);
    let color = players.get_current_player().get_player_color();
    for i in 1..=board.get_capture_range() + 1 {
        let inp = (f_x(input.0, i as i32), f_y(input.1, i as i32));
        selected_vec.push((board.get(inp), inp));
    }
    match (
        Tile::Color(color) == selected_vec.pop().unwrap().0,
        selected_vec.iter().all(|x| x.0 == Tile::Color(color.get_inverse_color()))
    ) {
        (true, true) => {
            for (_, inp) in selected_vec.iter() {
                board.replace(*inp, Tile::Empty);
            }
            players.add_capture(color)
        },
        _ => ()
    }
}

fn double_free_three_cases(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    fn drift(board: &Board, input: Input, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, drift: i32) -> Option<&Tile> {
        if f_x(input.0, drift) > board.get_size() - 1 || f_y(input.1, drift) > board.get_size() - 1 {
            None
        } else {
            board.get_protected((f_x(input.0, drift), f_y(input.1, drift)))
        }
    }
    match (
        drift(board, input, f_x, f_y, -4),
        drift(board, input, f_x, f_y, -3),
        drift(board, input, f_x, f_y, -2),
        drift(board, input, f_x, f_y, -1),
        color,
        drift(board, input, f_x, f_y, 1),
        drift(board, input, f_x, f_y, 2),
        drift(board, input, f_x, f_y, 3),
        drift(board, input, f_x, f_y, 4),
    ) {
        /* .0.23. */
        (_, _, _, Some(Tile::Empty), Color::Black, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => 1,
        (_, _, _, Some(Tile::Empty), Color::White, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => 1,
        /* .32.0. */
        (Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Color::Black, Some(Tile::Empty), _, _, _) => 1,
        (Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty), Color::White, Some(Tile::Empty), _, _, _) => 1,
        /* .01.3. */
        (_, _, _, Some(Tile::Empty), Color::Black, Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => 1,
        (_, _, _, Some(Tile::Empty), Color::White, Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => 1,
        /* .3.10. */
        (Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Color::Black, Some(Tile::Empty), _, _, _) => 1,
        (Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Color::White, Some(Tile::Empty), _, _, _) => 1,
        /* .012. */
        (_, _, _, Some(Tile::Empty), Color::Black, Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty), _) => 1,
        (_, _, _, Some(Tile::Empty), Color::White, Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty), _) => 1,
        /* .210. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Color::Black, Some(Tile::Empty), _, _, _) => 1,
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Color::White, Some(Tile::Empty), _, _, _) => 1,
        /* .10.2. */
        (_, _, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Color::Black, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), _) => 1,
        (_, _, Some(Tile::Empty), Some(Tile::Color(Color::White)), Color::White, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), _) => 1,
        /* .2.01. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Color::Black, Some(Tile::Color(Color::Black)), Some(Tile::Empty), _, _) => 1,
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Color::White, Some(Tile::Color(Color::White)), Some(Tile::Empty), _, _) => 1,
        /* .101. */
        (_, _, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Color::Black, Some(Tile::Color(Color::Black)), Some(Tile::Empty), _, _) => 1,
        (_, _, Some(Tile::Empty), Some(Tile::Color(Color::White)), Color::White, Some(Tile::Color(Color::White)), Some(Tile::Empty), _, _) => 1,
        _ => 0
    }
}