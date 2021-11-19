use std::fmt;

pub const BOARD_LENGTH: usize = 19;
pub const TOTAL_TILES: usize = BOARD_LENGTH * BOARD_LENGTH;
const TOTAL_TILES_MINUS_1: usize = TOTAL_TILES - 1;
const ALIGNEMENT_NB: usize = 5;
use crate::error::PlacementError;
use crate::color::Color;
use crate::players::{Players, CAPTURE_RANGE, Player};

pub type Input = (usize, usize);

pub fn get_input(i: usize) -> Input {
    (i % BOARD_LENGTH, i / BOARD_LENGTH)
}

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
    board: Vec<Tile>
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {board: vec![Tile::Empty; size]}
    }

    pub fn get_board(&self) -> &Vec<Tile> {
        &self.board
    }

    fn replace(&mut self, input: Input, tile: Tile) {
        self.board[input.0 + input.1 * BOARD_LENGTH] = tile;
    }

    pub fn get(&self, input: Input) -> Tile {
        self.board[input.0 + input.1 * BOARD_LENGTH]
    }

    pub fn is_finished(&self, player: Player) -> (bool, Option<Color>) {
        match self.board.iter().enumerate().map(|(i, x)| {
            if let Tile::Color(color) = x {
                return self.check_victory(i, *color, x, *color == player.get_player_color())
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

    pub fn _get_free_three(&self, input: Input, color: Color) -> u8 {
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
        lst.iter().sum::<u8>()
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
        count += case3(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case3(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize);   
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
        count += case5(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize);
        if count >= 2 {
            return true
        }
        count += case5(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize);
        if count >= 2 {
            return true
        }
        false
    }

    pub fn add_value(&mut self, input: Input, players: &mut Players) -> Result<(), PlacementError> {
        let color = players.get_current_player().get_player_color();
        if input.0 > BOARD_LENGTH - 1 || input.1 > BOARD_LENGTH - 1 {
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

    fn slice(&self, start: usize, length: usize, f_x: fn(usize, usize) -> usize, f_y: fn(usize, usize) -> usize) -> Vec<Tile> {
        let mut ret =  Vec::with_capacity(length);
        for i in 0..length {
            ret.push(self.board[f_x(start, i) + f_y(start, i) * BOARD_LENGTH])
        }
        ret
    }

    fn check_victory(&self, i: usize, color: Color, tile: &Tile, captured: bool) -> (bool, Option<Color>) {
        if  i % BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
            self.board[i..i + ALIGNEMENT_NB].iter().all(|x| *x == *tile) &&
            (!captured || cannot_be_captured(self, get_input(i), color, |x, y| (x as i32 + y) as usize, |x, _| x)) {
            return (true, Some(color))
        }
        if  i / BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
            self.slice(i, ALIGNEMENT_NB, |start, _| start, |_, x| x).iter().all(|x| *x == *tile) &&
            (!captured || cannot_be_captured(self, get_input(i), color, |x, _| x , |x, y| (x as i32 + y) as usize)) {
            return (true, Some(color))
        }
        if  i / BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
            i % BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
            self.slice(i, ALIGNEMENT_NB, |start, x| start + x, |_, x| x).iter().all(|x| *x == *tile) &&
            (!captured || cannot_be_captured(self, get_input(i), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize)) {
            return (true, Some(color))
        }
        if  i / BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
            i % BOARD_LENGTH >= ALIGNEMENT_NB - 1 &&
            self.slice(i, ALIGNEMENT_NB, |start, x| start - x, |_, x| x).iter().all(|x| *x == *tile) &&
            (!captured || cannot_be_captured(self, get_input(i), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize)) {
            return (true, Some(color))
        }
        (false, None)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let legend: String = (0..BOARD_LENGTH)
            .collect::<Vec<usize>>()
            .iter()
            .map(|x| *x % 10)
            .fold("".to_string(), |acc, x| format!("{}  {}", acc, x))
            .to_string()
            + "\n0 ";
        let my_str: String = self.board.iter().enumerate().fold(legend, |acc, (i, x)| {
            match ((i + 1) % BOARD_LENGTH, i) {
                (_, TOTAL_TILES_MINUS_1) => format!("{}{}", acc, x),
                (0, _) => format!("{}{}\n{} ", acc, x, ((i / BOARD_LENGTH) + 1) % 10),
                _ => format!("{}{}  ", acc, x)
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
    for i in 0..ALIGNEMENT_NB {
        if f_x(input.0, i as i32) < BOARD_LENGTH && f_y(input.1, i as i32) < BOARD_LENGTH {
            if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 + y) as usize, |x, _| x) {
                return false
            } else if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize) {
                return false
            } else if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, _| x, |x, y| (x as i32 + y) as usize) {
                return false
            } else if !cannot_be_captured_prime(board, (f_x(input.0, i as i32), f_y(input.1, i as i32)), color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize) {
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
    for i in 1..=CAPTURE_RANGE {
        if f_x(input.0, i as i32) > BOARD_LENGTH - 1 || f_y(input.1, i as i32) > BOARD_LENGTH - 1 {
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
    for i in 1..=CAPTURE_RANGE {
        if f_x(input.0, -(i as i32)) > BOARD_LENGTH - 1 || f_y(input.1, -(i as i32)) > BOARD_LENGTH - 1 {
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
    match (
        vec.len(),
        sorted == vec![Tile::Color(color.get_inverse_color()), Tile::Empty]
    ) {
        (CAPTURE_RANGE, true) => false,
        _ => true
    }
}

fn execute_capture(
    board: &mut Board,
    input: Input,
    players: &mut Players,
    f_x: fn(usize, i32) -> usize,
    f_y: fn(usize, i32) -> usize
) {
    if f_x(input.0, (CAPTURE_RANGE + 1) as i32) > BOARD_LENGTH - 1 || f_y(input.1, (CAPTURE_RANGE + 1) as i32) > BOARD_LENGTH - 1 {
        return
    }
    let mut selected_vec: Vec<Tile> = Vec::with_capacity(CAPTURE_RANGE + 1);
    let color = players.get_current_player().get_player_color();
    for i in 1..=CAPTURE_RANGE + 1 {
        selected_vec.push(board.get((f_x(input.0, i as i32), f_y(input.1, i as i32))));
    }
    match (
        Tile::Color(color) == selected_vec.pop().unwrap(),
        selected_vec.iter().all(|x| *x == Tile::Color(color.get_inverse_color()))
    ) {
        (true, true) => {
            for z in 1..=CAPTURE_RANGE {
                board.replace((f_x(input.0, z as i32), f_y(input.1, z as i32)), Tile::Empty);
            }
            players.add_capture(color)
        },
        _ => ()
    }
}


fn case1(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 3) > BOARD_LENGTH - 1 || f_x(input.0, -1) > BOARD_LENGTH - 1 || f_y(input.1, 3) > BOARD_LENGTH - 1 || f_y(input.1, -1) > BOARD_LENGTH - 1 {
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

fn case2(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 2) > BOARD_LENGTH - 1 || f_x(input.0, -2) > BOARD_LENGTH - 1 || f_y(input.1, 2) > BOARD_LENGTH - 1 || f_y(input.1, -2) > BOARD_LENGTH - 1 {
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

fn case3(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 4) > BOARD_LENGTH - 1 || f_x(input.0, -1) > BOARD_LENGTH - 1 || f_y(input.1, 4) > BOARD_LENGTH - 1 || f_y(input.1, -1) > BOARD_LENGTH - 1 {
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

fn case4(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 3) > BOARD_LENGTH - 1 || f_x(input.0, -2) > BOARD_LENGTH - 1 || f_y(input.1, 3) > BOARD_LENGTH - 1 || f_y(input.1, -2) > BOARD_LENGTH - 1 {
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

fn case5(board: &Board, input: Input, color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> u8 {
    if f_x(input.0, 4) > BOARD_LENGTH - 1 || f_x(input.0, -1) > BOARD_LENGTH - 1 || f_y(input.1, 4) > BOARD_LENGTH - 1 || f_y(input.1, -1) > BOARD_LENGTH - 1 {
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