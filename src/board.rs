use std::fmt;

const BOARD_LENGTH: usize = 3;
pub const TOTAL_TILES: usize = BOARD_LENGTH * BOARD_LENGTH;
const TOTAL_TILES_MINUS_1: usize = TOTAL_TILES - 1;
const ALIGNEMENT_NB: usize = 3;
use crate::error::PlacementError;
use crate::color::Color;

#[derive(PartialEq, Clone, Debug, Copy)]
enum Tile {
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

    fn replace(&mut self, input: (usize, usize), tile: Tile) {
        self.board[input.0 + input.1 * BOARD_LENGTH] = tile;
    }

    fn get(&self, input: (usize, usize)) -> Tile {
        self.board[input.0 + input.1 * BOARD_LENGTH]
    }

    pub fn is_finished(&self) -> (bool, Option<Color>) {
        match self.board.iter().enumerate().map(|(i, x)| {
            if let Tile::Color(color) = x {
                if  i % BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
                    self.board[i..i + ALIGNEMENT_NB].iter().filter(|z| *z == x).count() == ALIGNEMENT_NB {
                        return (true, Some(color))
                }
                if i / BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB {
                    if self.board[i..=i + BOARD_LENGTH * (ALIGNEMENT_NB - 1)]
                        .iter()
                        .enumerate()
                        .filter(|(i2, z)| i2 % BOARD_LENGTH == 0 && *z == x)
                        .count() == ALIGNEMENT_NB {
                        return (true, Some(color))
                    }
                }
                if  i / BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
                    i % BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB {
                        let mut index = 0;
                    if self.board[i..i + BOARD_LENGTH * (ALIGNEMENT_NB - 1) + ALIGNEMENT_NB]
                        .iter()
                        .enumerate()
                        .filter(|(i2, z)| {
                            if *i2 == BOARD_LENGTH * index + index && *z == x {
                                index += 1;
                                return true
                            } 
                            false
                        })
                        .count() == ALIGNEMENT_NB {
                        return (true, Some(color))
                    }
                }
                if  i / BOARD_LENGTH <= BOARD_LENGTH - ALIGNEMENT_NB &&
                    i % BOARD_LENGTH >= ALIGNEMENT_NB - 1 {
                        let mut index = 0;
                    if self.board[i..i + BOARD_LENGTH * (ALIGNEMENT_NB - 1)]
                        .iter()
                        .enumerate()
                        .filter(|(i2, z)| {
                            if *i2 == BOARD_LENGTH * index - index && *z == x {
                                index += 1;
                                return true
                            } 
                            false
                        })
                        .count() == ALIGNEMENT_NB {
                        return (true, Some(color))
                    }
                }
            }
            (false, None)
        }).find(|x| x.0 == true) {
            Some((true, Some(color))) => (true, Some(*color)),
            _ => {
                if !self.board.iter().any(|x| *x == Tile::Empty) {
                    return (true, None)
                }
                (false, None)
            }
        }
    }

    pub fn get_free_three(&self, input: (usize, usize), color: Color) -> i32 {
        let lst = [
            case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
            case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
            case1(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
            case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case1(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case1(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
            case1(self, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),
            case2(self, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
            case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
            case2(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
            case2(self, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
            case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
            case2(self, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
            case2(self, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
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

    pub fn check_double_free_three(&self, input: (usize, usize), color: Color) -> bool {
        self.get_free_three(input, color) > 1
    }
    

    pub fn add_value(&mut self, input: (usize, usize), color: Color) -> Result<(), PlacementError> {
        if input.0 > BOARD_LENGTH - 1 || input.1 > BOARD_LENGTH - 1 {
            return Err(PlacementError::OutOfBounds)
        } else if self.get(input) != Tile::Empty {
            return Err(PlacementError::NotEmpty)
        } else if self.check_double_free_three(input, color) {
            return Err(PlacementError::DoubleFreeThree)
        }
        self.replace(input, Tile::Color(color));
        Ok(())
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

fn case1 (board: &Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 3) > BOARD_LENGTH - 1 || f_x(input.0, -1) > BOARD_LENGTH - 1 || f_y(input.1, 3) > BOARD_LENGTH - 1 || f_y(input.1, -1) > BOARD_LENGTH - 1 {
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

fn case2 (board: &Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 2) > BOARD_LENGTH - 1 || f_x(input.0, -2) > BOARD_LENGTH - 1 || f_y(input.1, 2) > BOARD_LENGTH - 1 || f_y(input.1, -2) > BOARD_LENGTH - 1 {
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

fn case3 (board: &Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 4) > BOARD_LENGTH - 1 || f_x(input.0, -1) > BOARD_LENGTH - 1 || f_y(input.1, 4) > BOARD_LENGTH - 1 || f_y(input.1, -1) > BOARD_LENGTH - 1 {
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

fn case4 (board: &Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 3) > BOARD_LENGTH - 1 || f_x(input.0, -2) > BOARD_LENGTH - 1 || f_y(input.1, 3) > BOARD_LENGTH - 1 || f_y(input.1, -2) > BOARD_LENGTH - 1 {
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

fn case5 (board: &Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 4) > BOARD_LENGTH - 1 || f_x(input.0, -1) > BOARD_LENGTH - 1 || f_y(input.1, 4) > BOARD_LENGTH - 1 || f_y(input.1, -1) > BOARD_LENGTH - 1 {
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