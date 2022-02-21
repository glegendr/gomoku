use crate::board::{Board, Tile};
use crate::color::Color;
use crate::heuristic::Coordinates;


/* ORDER */
const BBBBB_SCORE: i32 = ((i32::MAX as f64) * (1.0 / 2.0)) as i32;
/* Live 4 */
const EBBBBE_SCORE: i32 = ((BBBBB_SCORE as f64) / 6.66) as i32;
const EBEBBBE_SCORE: i32 = ((BBBBB_SCORE as f64) / 6.66) as i32;
const EBBEBBE_SCORE: i32 = ((BBBBB_SCORE as f64) / 6.66) as i32;
/* Dead 4 */
const EBBBBW_SCORE: i32 = ((EBBBBE_SCORE as f64) / 1.5) as i32;
const EBBBE_SCORE: i32 = ((EBBBBE_SCORE as f64) / 1.5) as i32;
const EBEBEBE_SCORE: i32 = ((EBBBBE_SCORE as f64) / 1.5) as i32;
/* Live 3 */
const EBEBBE_SCORE: i32 = ((EBBBBE_SCORE as f64) / 1.5) as i32;
const EBEEBBE_SCORE: i32 = ((EBBBBE_SCORE as f64) / 1.5) as i32;
/* Dead 3 */
const EBBBW_SCORE: i32 = ((EBBBBW_SCORE as f64) / 2.0) as i32;
const WEBBBEW_SCORE: i32 = ((EBBBBW_SCORE as f64) / 2.0) as i32;
const EBBEBW_SCORE: i32 = ((EBBBBW_SCORE as f64) / 2.0) as i32;
const WBBEBE_SCORE: i32 = ((EBBBBW_SCORE as f64) / 2.0) as i32;
/* Live 2 */
const EBEBE_SCORE: i32 = ((EBBBW_SCORE as f64) / 5.0) as i32;
const EBBE_SCORE: i32 = ((EBBBW_SCORE as f64) / 5.0) as i32;
const EBEEBE_SCORE: i32 = ((EBBBW_SCORE as f64) / 5.0) as i32;
const EBEEEBE_SCORE: i32 = ((EBBBW_SCORE as f64) / 5.0) as i32;
/* Dead 2 */
const EBEEBW_SCORE: i32 = ((EBEBE_SCORE as f64) / 3.33) as i32;
const EBEBW_SCORE: i32 = ((EBEBE_SCORE as f64) / 3.33) as i32;
const EBBW_SCORE: i32 = ((EBEBE_SCORE as f64) / 3.33) as i32;

pub fn check_5_and_more(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<(i32, usize)> {
    match (
        board.get_protected((f_x(coordinates.x, -2), f_y(coordinates.y, -2))), 
        board.get_protected((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get_protected((coordinates.x, coordinates.y)),
        board.get_protected((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get_protected((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get_protected((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get_protected((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get_protected((f_x(coordinates.x, 5), f_y(coordinates.y, 5)))
    ) {
        /* O.XXX.O */
        (Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::White)), _) => {
            if color == Color::Black {
                return Some((WEBBBEW_SCORE, 4))
            } else {
                return Some((-WEBBBEW_SCORE, 4))
            }
        },
        (Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), _) => {
            if color == Color::Black {
                return Some((-WEBBBEW_SCORE, 4))
            } else {
                return Some((WEBBBEW_SCORE, 4))
            }
        },
        /* XXXXX */
        (_, _, Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), _) => {
            if color == Color::Black {
                return Some((BBBBB_SCORE, 5))
            } else {
                return Some((-BBBBB_SCORE, 5))
            }
        },
        (_, _, Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), _) => {
            if color == Color::Black {
                return Some((-BBBBB_SCORE, 5))
            } else {
                return Some((BBBBB_SCORE, 5))
            }   
        },
        /* ---------------------------- 5 ----------------------------*/
        /* .X.XXX. || .XXX.X. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBEBBBE_SCORE, 5))
            } else {
                return Some((-EBEBBBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBEBBBE_SCORE, 5))
            } else {
                return Some((-EBEBBBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBEBBBE_SCORE, 5))
            } else {
                return Some((EBEBBBE_SCORE, 5))
            }   
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBEBBBE_SCORE, 5))
            } else {
                return Some((EBEBBBE_SCORE, 5))
            }   
        },
        /* .XX.XX. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBBEBBE_SCORE, 5))
            } else {
                return Some((-EBBEBBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBBEBBE_SCORE, 5))
            } else {
                return Some((EBBEBBE_SCORE, 5))
            }   
        },
        /* .X..XX. || .XX..X. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBEEBBE_SCORE, 5))
            } else {
                return Some((-EBEEBBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBEEBBE_SCORE, 5))
            } else {
                return Some((-EBEEBBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBEEBBE_SCORE, 5))
            } else {
                return Some((EBEEBBE_SCORE, 5))
            }   
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBEEBBE_SCORE, 5))
            } else {
                return Some((EBEEBBE_SCORE, 5))
            }
        },
        /* .X.X.X. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBEBEBE_SCORE, 5))
            } else {
                return Some((-EBEBEBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBEBEBE_SCORE, 5))
            } else {
                return Some((EBEBEBE_SCORE, 5))
            }
        },
        /* .X...X. */
        (_, Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Color(Color::Black)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((EBEEEBE_SCORE, 5))
            } else {
                return Some((-EBEEEBE_SCORE, 5))
            }
        },
        (_, Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Color(Color::White)), Some(Tile::Empty)) => {
            if color == Color::Black {
                return Some((-EBEEEBE_SCORE, 5))
            } else {
                return Some((EBEEEBE_SCORE, 5))
            }
        },
        _ => ()
    };
    None
}

pub fn get_cases_size_2(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> Option<i32> {
    let size = board.get_size();
    let first_match;
    let last_match;
    if f_x(coordinates.x, 1) >= size || f_y(coordinates.y, 1) >= size {
        return None
    }
    if f_x(coordinates.x, -1) >= size || f_y(coordinates.y, -1) >= size {
        first_match = Tile::OutOfBounds;
    } else {
        first_match = board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1)))
    }
    if f_x(coordinates.x, 2) >= size || f_y(coordinates.y, 2) >= size {
        last_match = Tile::OutOfBounds;
    } else {
        last_match = board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2)))
    }
    match (
        first_match,
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        last_match
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
        /* .XXO || OXX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(EBBW_SCORE)
            } else {
                Some(-EBBW_SCORE)
            }
        },
        (Tile::Color(Color::White) | Tile::OutOfBounds, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBW_SCORE)
            } else {
                Some(-EBBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(-EBBW_SCORE)
            } else {
                Some(EBBW_SCORE)
            }
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
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
    let first_match;
    let last_match;
    if f_x(coordinates.x, 2) >= size || f_y(coordinates.y, 2) >= size {
        return None
    }
    if f_x(coordinates.x, -1) >= size || f_y(coordinates.y, -1) >= size {
        first_match = Tile::OutOfBounds;
    } else {
        first_match = board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1)))
    }
    if f_x(coordinates.x, 3) >= size || f_y(coordinates.y, 3) >= size {
        last_match = Tile::OutOfBounds;
    } else {
        last_match = board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3)))
    }
    match (
        first_match,
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        last_match
    ) {
        /* .X.XO || OX.X. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(EBEBW_SCORE)
            } else {
                Some(-EBEBW_SCORE)
            }
        },
        (Tile::Color(Color::White) | Tile::OutOfBounds, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEBW_SCORE)
            } else {
                Some(-EBEBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(-EBEBW_SCORE)
            } else {
                Some(EBEBW_SCORE)
            }
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
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
        (Tile::Color(Color::White) | Tile::OutOfBounds, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBBW_SCORE)
            } else {
                Some(-EBBBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(EBBBW_SCORE)
            } else {
                Some(-EBBBW_SCORE)
            }
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBBBW_SCORE)
            } else {
                Some(EBBBW_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
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
    let first_match;
    let last_match;
    if f_x(coordinates.x, 3) >= size || f_y(coordinates.y, 3) >= size {
        return None
    }
    if f_x(coordinates.x, -1) >= size || f_y(coordinates.y, -1) >= size {
        first_match = &Tile::OutOfBounds;
    } else {
        first_match = board.get_ref((f_x(coordinates.x, -1), f_y(coordinates.y, -1)))
    }
    if f_x(coordinates.x, 4) >= size || f_y(coordinates.y, 4) >= size {
        last_match = &Tile::OutOfBounds;
    } else {
        last_match = board.get_ref((f_x(coordinates.x, 4), f_y(coordinates.y, 4)))
    }
    match (
        first_match,
        board.get_ref((coordinates.x, coordinates.y)),
        board.get_ref((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get_ref((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get_ref((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        last_match
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
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(EBBBBW_SCORE)
            } else {
                Some(-EBBBBW_SCORE)
            }
        },
        (Tile::Color(Color::White) | Tile::OutOfBounds, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBBBW_SCORE)
            } else {
                Some(-EBBBBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(-EBBBBW_SCORE)
            } else {
                Some(EBBBBW_SCORE)
            }   
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
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
        (Tile::Color(Color::White) | Tile::OutOfBounds, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(WBBEBE_SCORE)
            } else {
                Some(-WBBEBE_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(WBBEBE_SCORE)
            } else {
                Some(-WBBEBE_SCORE)
            }
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-WBBEBE_SCORE)
            } else {
                Some(WBBEBE_SCORE)
            }   
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(-WBBEBE_SCORE)
            } else {
                Some(WBBEBE_SCORE)
            }
        },
        /* .XX.XO || OX.XX. */
        (Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(EBBEBW_SCORE)
            } else {
                Some(-EBBEBW_SCORE)
            }
        },
        (Tile::Color(Color::White) | Tile::OutOfBounds, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBBEBW_SCORE)
            } else {
                Some(-EBBEBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(-EBBEBW_SCORE)
            } else {
                Some(EBBEBW_SCORE)
            }   
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Empty) => {
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
        (Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(EBEEBW_SCORE)
            } else {
                Some(-EBEEBW_SCORE)
            }
        },
        (Tile::Color(Color::White) | Tile::OutOfBounds,  Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if color == Color::Black {
                Some(EBEEBW_SCORE)
            } else {
                Some(-EBEEBW_SCORE)
            }
        },
        (Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black) | Tile::OutOfBounds) => {
            if color == Color::Black {
                Some(-EBEEBW_SCORE)
            } else {
                Some(EBEEBW_SCORE)
            }
        },
        (Tile::Color(Color::Black) | Tile::OutOfBounds, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if color == Color::Black {
                Some(-EBEEBW_SCORE)
            } else {
                Some(EBEEBW_SCORE)
            }
        },
        _ => None
    }
}