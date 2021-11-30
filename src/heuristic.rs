use crate::board::*;
use crate::players::*;
use crate::color::*;
use crate::matching_cases::*;

#[derive(Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
    size: usize,
    start: Input,
    mode: ((usize, i32), (usize, i32))
}

pub enum Mode {
    Vertically,
    Horizontaly,
    Diagoneso,
    Diagonose,
}

impl Coordinates {
    fn new(size: usize, start: Input, mode: Mode) -> Coordinates {
        match mode {
            Mode::Horizontaly | Mode::Vertically => {
                Coordinates {x: start.0, y: start.1, size, start, mode: ((1, 1), (0, 1))}
            },
            Mode::Diagoneso => {
                Coordinates {x: start.0, y: start.1, size, start, mode: ((0, 1), (1, 1))}
            }
            _ => {
                Coordinates {x: start.0, y: start.1, size, start, mode: ((0, 1), (1, -1))}
            }
        }
    }

    fn to_index(&self) -> usize {
        self.x + self.y * self.size
    }

    fn drift(&mut self, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, nb: i32) {
        if f_x(self.x, nb) >= self.size && f_y(self.y, nb) >= self.size {
            if self.mode.0.0 == 0 {
                if (self.start.0 as i32 + self.mode.0.1) < self.size as i32 {
                    self.start.0 = (self.start.0 as i32 + self.mode.0.1) as usize;
                }
            } else {
                if (self.start.1 as i32 + self.mode.0.1) < self.size as i32 {
                    self.start.1 = (self.start.1 as i32 + self.mode.0.1) as usize;
                }
            }
            if self.mode.1.0 == 0 {
                if (self.start.0 as i32 + self.mode.1.1) < 0 {
                    self.start.0 = 0;
                } else {
                    self.start.0 = (self.start.0 as i32 + self.mode.1.1) as usize;
                }
            } else {
                if (self.start.1 as i32 + self.mode.1.1) < 0 {
                    self.start.1 = 0;
                } else {
                    self.start.1 = (self.start.1 as i32 + self.mode.1.1) as usize;
                }
            }
            self.x = self.start.0;
            self.y = self.start.1;
            return ;
        }
        if f_x(self.x, nb) >= self.size {
            if self.mode.0.0 == 0 {
                self.start.0 = (self.start.0 as i32 + self.mode.0.1) as usize;
            } else {
                self.start.1 = (self.start.1 as i32 + self.mode.0.1) as usize;
            }
            self.x = self.start.0;
            self.y = self.start.1;
            return ;
        } else {
            self.x = f_x(self.x, nb);
        }
        if f_y(self.y, nb) >= self.size {
            if self.mode.1.0 == 0 {
                if (self.start.0 as i32 + self.mode.1.1) < 0 {
                    self.start.0 = 0;
                } else {
                    self.start.0 = (self.start.0 as i32 + self.mode.1.1) as usize;
                }
            } else {
                if (self.start.1 as i32 + self.mode.1.1) < 0 {
                    self.start.1 = 0;
                } else {
                    self.start.1 = (self.start.1 as i32 + self.mode.1.1) as usize;
                }
            }
            self.x = self.start.0;
            self.y = self.start.1;
            return ;
        } else {
            self.y = f_y(self.y, nb);
        }
    }
}

fn get_cases(raw_board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color, current_player_color: Color) -> (i32, usize) { // ebbbbe
    match bbbbb(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-150000, 4), 
        1 => return (150000, 4),
        _ => (), 
    }
    match ebbbbe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-100000, 4), 
        1 => return (100000, 4),
        _ => (), 
    }
    match ebbbbw(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-80000, 4), 
        1 => return (80000, 4),
        _ => (), 
    }
    match ebebbbe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-80000, 5), 
        1 => return (80000, 5),
        _ => (), 
    }
    match ebbebbe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-75000, 5), 
        1 => return (75000, 5),
        _ => (), 
    }
    match ebbbe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-50000, 3), 
        1 => return (50000, 3),
        _ => (), 
    }
    match ebebbe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-45000, 3), 
        1 => return (45000, 3),
        _ => (), 
    }
    match ebbbw(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-35000, 3), 
        1 => return (35000, 3),
        _ => (), 
    }
    match wbbebe(raw_board, f_x, f_y, coordinates, color) {
        -1 => {
            if color == current_player_color {
                return (-15000, 4)
            }
            return (-30000, 4)
        }, 
        1 => {
            if color == current_player_color {
                return (30000, 4)
            }
            return (15000, 4)
        },
        _ => (), 
    }
    match ebbebw(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-20000, 4), 
        1 => return (20000, 4),
        _ => (), 
    }
    match ebeebbe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-25000, 5), 
        1 => return (25000, 5),
        _ => (), 
    }
    match ebebebe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-20000, 5), 
        1 => return (20000, 5), 
        _ => (), 
    }
    match webbbew(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-15000, 4), 
        1 => return (15000, 4), 
        _ => (), 
    }
    match ebeeebe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-7500, 5), 
        1 => return (7500, 5), 
        _ => (), 
    }
    match ebebe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-7500, 3), 
        1 => return (7500, 3), 
        _ => (), 
    }
    match ebeebe(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-7500, 4), 
        1 => return (7500, 4), 
        _ => (), 
    }
    match ebbw(raw_board, f_x, f_y, coordinates, color) { // define Who play to get score
        -1 => {
            if color == current_player_color {
                return (10000, 2)
            }
            return (-10000, 2)
        },
        1 => {
            if color == current_player_color {
                return (5000, 2)
            }
            return (-10000, 2)
        },
        _ => (), 
    }
    match ebebw(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-10000, 3), 
        1 => return (10000, 3), 
        _ => (), 
    }
    match ebeebw(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-5000, 4), 
        1 => return (5000, 4), 
        _ => (), 
    }
    match two_in_a_row(raw_board, f_x, f_y, coordinates, color) {
        -1 => return (-10000, 2), 
        1 => return (10000, 2), 
        _ => (), 
    };
        (0, 1)
}

fn add(x: usize, y: i32) -> usize {
    (x as i32 + y) as usize
}

fn sub(x: usize, y: i32) -> usize {
    if y > (x as i32) {
        return usize::MAX
    }
    ((x as i32) - y) as usize
}

fn skip(x: usize, _:i32) -> usize {
    x
}

pub fn iter_on_board(raw_board: &Board, mode: Mode, color: Color, current_player_color: Color) -> i32 {
    let (f_x, f_y, start): (fn(usize, i32) -> usize, fn(usize, i32) -> usize, Input) = match mode {
        Mode::Vertically => (skip, add, (0, 0)),
        Mode::Horizontaly => (add, skip, (0, 0)),
        Mode::Diagoneso => (sub, add, (0, 0)),
        Mode::Diagonose => (add, add, (0, raw_board.get_size() - 1))
    };
    let mut note: i32 = 0;
    let board = raw_board.get_board();
    let mut i: usize = 0;
    let mut coordinates = Coordinates::new(raw_board.get_size(), start, mode);
    while i < raw_board.get_total_tiles() {
        // println!("{:?}", coordinates);
        if board[coordinates.to_index()] == Tile::Empty {
            coordinates.drift(f_x, f_y, 1);
            i += 1;
            continue
        }
        let (curr_note, skip) = get_cases(&raw_board, f_x, f_y, &coordinates, color, current_player_color);
        note += curr_note;
        coordinates.drift(f_x, f_y, skip as i32);
        i += skip;
    }
    note
}

fn get_distance(board: &Board, distance: i32, input: Input) -> bool {
    for y in -distance..=distance {
        if (input.1 as i32) + y < 0 {
            continue;
        } else if (input.1 as i32) + y >= board.get_size() as i32 {
            break;
        }
        for x in -distance..=distance {
            if (input.0 as i32) + x < 0 || (y != -distance && y != distance && x != -distance && x != distance){
                continue;
            } else if (input.0 as i32) + x >= board.get_size() as i32 {
                break;
            }
            if let Tile::Color(_) = board.get((((input.0 as i32) + x) as usize, ((input.1 as i32) + y) as usize)) {
                return true
            }
            
        }

    }
    false
}

pub fn pruning_heuristic(input: Input, board: &Board) -> bool {
    for distance in 1..=1 {
        if get_distance(board, distance, input) {
            return true
        }
    }
    false
}

pub fn heuristic(board: &Board, players: &Players, default_color: Color) -> i32 {
    let me = players.get_player(default_color);
    match players.is_finished() {
        (true, Some(color)) => {
            if color == default_color {
                return i32::MAX
            }
                return i32::MIN
        },
        _ => ()
    }
    match board.is_finished(me) {
        (true, Some(color)) => {
            if color == default_color {
                return i32::MAX
            }
                return i32::MIN
        },
        (true, _) => {
            return 0
        }
        _ => ()
    }
    let opponent = players.get_player(default_color.get_inverse_color());
    let mut eval = 0;
    eval += ((me.get_player_captured().pow(2) as f64 / players.get_captured_nb().pow(2) as f64) * (i32::MAX as f64)) as i32;
    eval -= ((opponent.get_player_captured().pow(2) as f64 / players.get_captured_nb().pow(2) as f64) * (i32::MAX as f64)) as i32;
    eval += iter_on_board(board, Mode::Horizontaly, default_color, players.get_current_player().get_player_color());
    eval += iter_on_board(board, Mode::Vertically, default_color, players.get_current_player().get_player_color());
    eval += iter_on_board(board, Mode::Diagoneso, default_color, players.get_current_player().get_player_color());
    eval += iter_on_board(board, Mode::Diagonose, default_color, players.get_current_player().get_player_color());
    eval
    // gagner / perdu capture prochain tour
    // gagner / perdu alignement prochain tour
    // + proche de pièces capturer = + de points 
    // + proche de pièces capturer pour l'adv = - de points
    // x pts * nb de free_three
}
