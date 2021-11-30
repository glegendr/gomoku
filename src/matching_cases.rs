use crate::board::{Board, Tile};
use crate::color::Color;
use crate::heuristic::Coordinates;

/* SPECIAL */
const WEBBBEW_SCORE: i32 = 15000;
const BBBBB_SCORE: i32 = 150000;

/* 2 */
const EBBE_SCORE: i32 = 10000;
const EBBW_SCORE: i32 = 10000; // SPECIAL

/* 3 */
const EBEBW_SCORE: i32 = 10000;
const EBEBE_SCORE: i32 = 7500;
const EBBBW_SCORE: i32 = 35000;
const EBBBE_SCORE: i32 = 50000;

/* 4 */
const EBBBBE_SCORE: i32 = 100000;
const EBBBBW_SCORE: i32 = 80000;
const EBEBBE_SCORE: i32 = 45000;
const WBBEBE_SCORE: i32 = 15000; // SPECIAL
const EBBEBW_SCORE: i32 = 20000;
const EBEEBE_SCORE: i32 = 7500;
const EBEEBW_SCORE: i32 = 5000;

/* 5 */
const EBEBBBE_SCORE: i32 = 80000;
const EBBEBBE_SCORE: i32 = 75000;
const EBEEBBE_SCORE: i32 = 25000;
const EBEBEBE_SCORE: i32 = 20000;
const EBEEEBE_SCORE: i32 = 7500;

/* O.XXX.O */
pub fn webbbew(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> { 
    let size = board.get_size();
    if f_x(coordinates.x, -2) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -2) >= size || f_y(coordinates.y, 4) >= size {
        return None
    }
    match (
        board.get((f_x(coordinates.x, -2), f_y(coordinates.y, -2))), 
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4)))
    ) {
        (Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(WEBBBEW_SCORE)
            } else {
                Some(-WEBBBEW_SCORE)
            }
        },
        (Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-WEBBBEW_SCORE)
            } else {
                Some(WEBBBEW_SCORE)
            }
        },
        _ => None
    }
}

/* XXXXX */
pub fn bbbbb(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> { 
    let size = board.get_size();
    if f_x(coordinates.x, 4) >= size || f_y(coordinates.y, 4) >= size {
        return None
    }
    match (
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(BBBBB_SCORE)
            } else {
                Some(-BBBBB_SCORE)
            }
        },
        (Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(-BBBBB_SCORE)
            } else {
                Some(BBBBB_SCORE)
            }   
        },
        _ => None
    }
}

pub fn get_cases_size_2(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> {
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 2) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 2) >= size {
        return None
    }
    match (
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))),
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2)))
    ) {
        /* .XX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBE_SCORE)
            } else {
                Some(-EBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBE_SCORE)
            } else {
                Some(EBBE_SCORE)
            }
        },
        /* .XXW || WXX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(EBBW_SCORE)
            } else {
                Some(-EBBW_SCORE)
            }
        },
        (Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBW_SCORE)
            } else {
                Some(-EBBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-EBBW_SCORE)
            } else {
                Some(EBBW_SCORE)
            }
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBW_SCORE)
            } else {
                Some(EBBW_SCORE)
            }
        },
        _ => None
    }
}
pub fn get_cases_size_3(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> {
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 3) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 3) >= size {
        return None
    }
    match (
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))),
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3)))
    ) {
        /* .X.XO || OX.X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(EBEBW_SCORE)
            } else {
                Some(-EBEBW_SCORE)
            }
        },
        (Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBW_SCORE)
            } else {
                Some(-EBEBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-EBEBW_SCORE)
            } else {
                Some(EBEBW_SCORE)
            }
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBW_SCORE)
            } else {
                Some(EBEBW_SCORE)
            }
        },
        /* .X.X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBE_SCORE)
            } else {
                Some(-EBEBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBE_SCORE)
            } else {
                Some(EBEBE_SCORE)
            }
        },
        /* .XXXO || OXXX. */
        (Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBBW_SCORE)
            } else {
                Some(-EBBBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(EBBBW_SCORE)
            } else {
                Some(-EBBBW_SCORE)
            }
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBBW_SCORE)
            } else {
                Some(EBBBW_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-EBBBW_SCORE)
            } else {
                Some(EBBBW_SCORE)
            }
        },
        /* .XXX. */
        (Tile::Empty, Tile::Color(Color::Black),Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBBE_SCORE)
            } else {
                Some(-EBBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBBE_SCORE)
            } else {
                Some(EBBBE_SCORE)
            }   
        },

        _ => None
    }
}

pub fn get_cases_size_4(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> {
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return None
    }
    match (
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))),
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4)))
    ) {
        /* .XXXX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBBBE_SCORE)
            } else {
                Some(-EBBBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBBBE_SCORE)
            } else {
                Some(EBBBBE_SCORE)
            }   
        },
        /* .XXXXO || OXXXX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(EBBBBW_SCORE)
            } else {
                Some(-EBBBBW_SCORE)
            }
        },
        (Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBBBW_SCORE)
            } else {
                Some(-EBBBBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-EBBBBW_SCORE)
            } else {
                Some(EBBBBW_SCORE)
            }   
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBBBW_SCORE)
            } else {
                Some(EBBBBW_SCORE)
            }   
        },
        /* .X.XX. || .XX.X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBBE_SCORE)
            } else {
                Some(-EBEBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBBE_SCORE)
            } else {
                Some(-EBEBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBBE_SCORE)
            } else {
                Some(EBEBBE_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBBE_SCORE)
            } else {
                Some(EBEBBE_SCORE)
            }
        },
        /* OXX.X. || .X.XXO */
        (Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(WBBEBE_SCORE)
            } else {
                Some(-WBBEBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(WBBEBE_SCORE)
            } else {
                Some(-WBBEBE_SCORE)
            }
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-WBBEBE_SCORE)
            } else {
                Some(WBBEBE_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-WBBEBE_SCORE)
            } else {
                Some(WBBEBE_SCORE)
            }
        },
        /* .XX.XO || OX.XX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(EBBEBW_SCORE)
            } else {
                Some(-EBBEBW_SCORE)
            }
        },
        (Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBEBW_SCORE)
            } else {
                Some(-EBBEBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-EBBEBW_SCORE)
            } else {
                Some(EBBEBW_SCORE)
            }   
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBEBW_SCORE)
            } else {
                Some(EBBEBW_SCORE)
            }
        },
        /* .X..X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEEBE_SCORE)
            } else {
                Some(-EBEEBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEEBE_SCORE)
            } else {
                Some(EBEEBE_SCORE)
            }
        },
        /* .X..XO || OX..X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if color == Color::Black {
                Some(EBEEBW_SCORE)
            } else {
                Some(-EBEEBW_SCORE)
            }
        },
        (Tile::Color(Color::White),  Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEEBW_SCORE)
            } else {
                Some(-EBEEBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if color == Color::Black {
                Some(-EBEEBW_SCORE)
            } else {
                Some(EBEEBW_SCORE)
            }
        },
        (Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEEBW_SCORE)
            } else {
                Some(EBEEBW_SCORE)
            }
        },
        _ => None
    }
}

pub fn get_cases_size_5(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> {
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 5) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 5) >= size {
        return None
    }
    match (
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))),
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get((f_x(coordinates.x, 5), f_y(coordinates.y, 5)))
    ) {
        /* .X.XXX. || .XXX.X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBBBE_SCORE)
            } else {
                Some(-EBEBBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBBBE_SCORE)
            } else {
                Some(-EBEBBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBBBE_SCORE)
            } else {
                Some(EBEBBBE_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBBBE_SCORE)
            } else {
                Some(EBEBBBE_SCORE)
            }   
        },
        /* .XX.XX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBEBBE_SCORE)
            } else {
                Some(-EBBEBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBEBBE_SCORE)
            } else {
                Some(EBBEBBE_SCORE)
            }   
        },
        /* .X..XX. || .XX..X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEEBBE_SCORE)
            } else {
                Some(-EBEEBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEEBBE_SCORE)
            } else {
                Some(-EBEEBBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEEBBE_SCORE)
            } else {
                Some(EBEEBBE_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEEBBE_SCORE)
            } else {
                Some(EBEEBBE_SCORE)
            }
        },
        /* .X.X.X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBEBE_SCORE)
            } else {
                Some(-EBEBEBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEBEBE_SCORE)
            } else {
                Some(EBEBEBE_SCORE)
            }
        },
        /* .X...X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEEEBE_SCORE)
            } else {
                Some(-EBEEEBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEEEBE_SCORE)
            } else {
                Some(EBEEEBE_SCORE)
            }
        },
        _ => None
    }
}