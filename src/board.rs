use std::fmt;

use crate::error::PlacementError;
use crate::color::Color;
use crate::players::{Players, Player};

pub type Input = (usize, usize);

#[derive(PartialEq, Clone, Debug, Copy, Eq, Ord, PartialOrd)]
pub enum Tile {
    Color(Color),
    Empty
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty =>  write!(f, "."),
            Tile::Color(color) => write!(f, "{}", color)
        }
    }
}

#[derive(PartialEq, Clone)]
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

    pub fn is_finished(&self, player: Player) -> (bool, Option<Color>) {
        match self.board.iter().enumerate().map(|(i, x)| {
            if let Tile::Color(color) = x {
                if *color == player.get_player_color() {
                    return self.check_victory(i, *color, x, false)
                } else {
                    return self.check_victory(i, *color, x, true)
                }
            }
            (false, None)
        }).find(|x| x.0 == true) {
            Some((true, Some(color))) => (true, Some(color)),
            _ => {
                if !self.board.iter().any(|x| *x == Tile::Empty) {
                    return (true, None)
                }
                (false, None)
            }
        }
    }

    pub fn get_free_three(&self, input: Input, color: Color) -> i32 {
        let lst = [
            case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
            case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
            case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
            case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case1(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
            case1(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),
            case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case2(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),  
            case3(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
            case3(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case3(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
            case3(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
            case3(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case3(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case3(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
            case3(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),    
            case4(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
            case4(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case4(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
            case4(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
            case4(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case4(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case4(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
            case4(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),
            case5(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
            case5(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case5(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
            case5(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
            case5(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case5(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case5(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
            case5(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),
        ];
        lst.iter().filter(|x| **x == true).count() as i32
    }

    pub fn check_double_free_three(&self, input: Input, color: Color) -> bool {
        self.get_free_three(input, color) >= 2
    }
    

    pub fn add_value(&mut self, input: Input, players: &mut Players) -> Result<(), PlacementError> {
        let color = players.get_current_player().get_player_color();
        if input.0 > self.get_size() - 1 || input.1 > self.get_size() - 1 {
            return Err(PlacementError::OutOfBounds)
        } else if self.get(input) != Tile::Empty {
            return Err(PlacementError::NotEmpty)
        } else if self.check_double_free_three(input, color) {
            return Err(PlacementError::DoubleFreeThree)
        }
        self.capture(input, players);
        self.replace(input, Tile::Color(color));
        Ok(())
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

    fn check_victory(&self, i: usize, color: Color, x: &Tile, captured: bool) -> (bool, Option<Color>) {
        if i % self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            self.board[i..i + self.get_alignement_nb()].iter().filter(|z| **z == *x).count() == self.get_alignement_nb() &&
            (!captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 + y) as usize, |x, _| x)) {
            return (true, Some(color))
        }
        if i / self.get_size() <= self.get_size() - self.get_alignement_nb() {
            if self.board[i..=i + self.get_size() * (self.get_alignement_nb() - 1)]
                .iter()
                .enumerate()
                .filter(|(i2, z)| i2 % self.get_size() == 0 && **z == *x)
                .count() == self.get_alignement_nb() &&
                (!captured || cannot_be_captured(self, self.get_input(i), color, |x, _| x , |x, y| (x as i32 + y) as usize)) {
                return (true, Some(color))
            }
        }
        if  i / self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            i % self.get_size() <= self.get_size() - self.get_alignement_nb() {
                let mut index = 0;
            if self.board[i..i + self.get_size() * (self.get_alignement_nb() - 1) + self.get_alignement_nb()]
                .iter()
                .enumerate()
                .filter(|(i2, z)| {
                    if *i2 == self.get_size() * index + index && **z == *x {
                        index += 1;
                        return true
                    } 
                    false
                })
                .count() == self.get_alignement_nb() &&
                (!captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize)) {
                return (true, Some(color))
            }
        }
        if  i / self.get_size() <= self.get_size() - self.get_alignement_nb() &&
            i % self.get_size() >= self.get_alignement_nb() - 1 {
                let mut index = 0;
            if self.board[i..i + self.get_size() * (self.get_alignement_nb() - 1)]
                .iter()
                .enumerate()
                .filter(|(i2, z)| {
                    if *i2 == self.get_size() * index - index && **z == *x {
                            index += 1;
                            return true
                        } 
                        false
                    })
                    .count() == self.get_alignement_nb() &&
                    (!captured || cannot_be_captured(self, self.get_input(i), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize)) {
                return (true, Some(color))
            }
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
    let mut vec: Vec<bool> = Vec::new();
    for i in 0..board.get_alignement_nb(){
        if f_x(input.0, i as i32) < board.get_size() && f_y(input.1, i as i32) < board.get_size() {
            vec.push(cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 + y) as usize, |x, _| x));
            vec.push(cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize));
            vec.push(cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, _| x, |x, y| (x as i32 + y) as usize));
            vec.push(cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize));
        }
    }
    vec.iter().all(|x| *x == true)
}

fn cannot_be_captured_prime(
    board: &Board,
    input: Input,
    color: Color,
    f_x: fn(usize, i32) -> usize,
    f_y: fn(usize, i32) -> usize
) -> bool {
    let mut vec: Vec<Tile> = vec![board.get(input)];
    for i in 1..=board.get_capture_range() {
        if f_x(input.0, i as i32) > board.get_size() - 1 || f_y(input.1, i as i32) > board.get_size() - 1 {
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
    for i in 1..=board.get_capture_range() {
        if f_x(input.0, -(i as i32)) > board.get_size() - 1 || f_y(input.1, -(i as i32)) > board.get_size() - 1 {
            break;
        }
        let tile = board.get((f_x(input.0, -(i as i32)), f_y(input.1, -(i as i32))));
        if tile == Tile::Color(color.get_inverse_color()) || tile == Tile::Empty {
            vec.push(tile);
            break;
        }
        vec.push(tile);
    }
    let mut sorted = vec![vec.pop().unwrap(), *vec.get(0).unwrap()];
    sorted.sort();
    vec = (&vec[1..]).to_vec();
    if (
        vec.len(),
        sorted == vec![Tile::Color(color.get_inverse_color()), Tile::Empty]
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
    let mut selected_vec: Vec<Tile> = Vec::with_capacity(board.get_capture_range() + 1);
    let color = players.get_current_player().get_player_color();
    for i in 1..=board.get_capture_range() + 1 {
        selected_vec.push(board.get((f_x(input.0, i as i32), f_y(input.1, i as i32))));
    }
    match (
        Tile::Color(color) == selected_vec.pop().unwrap(),
        selected_vec.iter().all(|x| *x == Tile::Color(color.get_inverse_color()))
    ) {
        (true, true) => {
            for z in 1..=board.get_capture_range() {
                board.replace((f_x(input.0, z as i32), f_y(input.1, z as i32)), Tile::Empty);
            }
            players.add_capture(color)
        },
        _ => ()
    }
}


fn case1(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 3) > board.get_size() - 1 || f_x(input.0, -1) > board.get_size() - 1 || f_y(input.1, 3) > board.get_size() - 1 || f_y(input.1, -1) > board.get_size() - 1 {
        return false
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, -1), f_y(input.1, -1)))
    ) {
        (Color::Black, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Empty) => true,
        (Color::White, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Empty) => true,
        _ => false
    }
}

fn case2(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 2) > board.get_size() - 1 || f_x(input.0, -2) > board.get_size() - 1 || f_y(input.1, 2) > board.get_size() - 1 || f_y(input.1, -2) > board.get_size() - 1 {
        return false
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, -1), f_y(input.1, -1))),
        board.get((f_x(input.0, -2), f_y(input.1, -2)))
    ) {
        (Color::Black, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => true,
        (Color::White, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => true,
        _ => false
    }
}

fn case3(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 4) > board.get_size() - 1 || f_x(input.0, -1) > board.get_size() - 1 || f_y(input.1, 4) > board.get_size() - 1 || f_y(input.1, -1) > board.get_size() - 1 {
        return false
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, 4), f_y(input.1, 4))),
        board.get((f_x(input.0, -1), f_y(input.1, -1)))
    ) {
        (Color::Black, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty) => true,
        (Color::White, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty) => true,
        _ => false
    }
}

fn case4(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 3) > board.get_size() - 1 || f_x(input.0, -2) > board.get_size() - 1 || f_y(input.1, 3) > board.get_size() - 1 || f_y(input.1, -2) > board.get_size() - 1 {
        return false
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, -1), f_y(input.1, -1))),
        board.get((f_x(input.0, -2), f_y(input.1, -2)))
    ) {
        (Color::Black, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => true,
        (Color::White, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => true,
        _ => false
    }
}

fn case5(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 4) > board.get_size() - 1 || f_x(input.0, -1) > board.get_size() - 1 || f_y(input.1, 4) > board.get_size() - 1 || f_y(input.1, -1) > board.get_size() - 1 {
        return false
    }
    match (
        color,
        board.get((f_x(input.0, 1), f_y(input.1, 1))),
        board.get((f_x(input.0, 2), f_y(input.1, 2))),
        board.get((f_x(input.0, 3), f_y(input.1, 3))),
        board.get((f_x(input.0, 4), f_y(input.1, 4))),
        board.get((f_x(input.0, -1), f_y(input.1, -1)))
    ) {
        (Color::Black, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Empty) => true,
        (Color::White, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Empty) => true,
        _ => false
    }
}
