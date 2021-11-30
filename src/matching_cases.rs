use crate::board::{Board, Tile};
use crate::color::Color;
use crate::heuristic::Coordinates;

/* .XX. */
pub fn two_in_a_row(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 {
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 2) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 2) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2)))
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X..XO || OX..X. */
pub fn ebeebw(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::White),  Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X.XO || OX.X. */
pub fn ebebw(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 3) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 3) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .XXW || WXX. */
pub fn ebbw(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 2) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 2) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X..X. */
pub fn ebeebe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X.X. */
pub fn ebebe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 3) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 3) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X...X. */
pub fn ebeeebe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 5) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 5) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get((f_x(coordinates.x, 5), f_y(coordinates.y, 5))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* O.XXX.O */
pub fn webbbew(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -2) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -2) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -2), f_y(coordinates.y, -2))), 
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X.X.X. */
pub fn ebebebe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 5) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 5) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get((f_x(coordinates.x, 5), f_y(coordinates.y, 5))), 
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X..XX. || .XX..X. */
pub fn ebeebbe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 5) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 5) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get((f_x(coordinates.x, 5), f_y(coordinates.y, 5))), 
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .XX.XO || OX.XX. */
pub fn ebbebw(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* OXX.X. || .X.XXO */
pub fn wbbebe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .XXXO || OXXX. */
pub fn ebbbw(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 3) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 3) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
    ) {
        (x, Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White),  Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .X.XX. || .XX.X. */
pub fn ebebbe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }
        },
        _ => 0
    }
}

/* .XXX. */
pub fn ebbbe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 3) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 3) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black),Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        _ => 0
    }
}

/* .XX.XX. */
pub fn ebbebbe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 5) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 5) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get((f_x(coordinates.x, 5), f_y(coordinates.y, 5))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        _ => 0
    }
}

/* .X.XXX. || .XXX.X. */
pub fn ebebbbe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 5) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 5) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
        board.get((f_x(coordinates.x, 5), f_y(coordinates.y, 5))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty, Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        _ => 0
    }
}

/* .XXXXO || OXXXX. */
pub fn ebbbbw(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::White)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::White), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        (x, Tile::Color(Color::Black), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        _ => 0
    }
}

/* .XXXX. */
pub fn ebbbbe(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, -1) >= size || f_x(coordinates.x, 4) >= size || f_y(coordinates.y, -1) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((f_x(coordinates.x, -1), f_y(coordinates.y, -1))), 
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Empty, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Empty) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Empty, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Empty) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        _ => 0
    }
}

/* XXXXX */
pub fn bbbbb(board: &Board, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize, coordinates: &Coordinates, color: Color) -> i32 { 
    let size = board.get_size();
    if f_x(coordinates.x, 4) >= size || f_y(coordinates.y, 4) >= size {
        return 0
    }
    match (
        color,
        board.get((coordinates.x, coordinates.y)),
        board.get((f_x(coordinates.x, 1), f_y(coordinates.y, 1))),
        board.get((f_x(coordinates.x, 2), f_y(coordinates.y, 2))),
        board.get((f_x(coordinates.x, 3), f_y(coordinates.y, 3))),
        board.get((f_x(coordinates.x, 4), f_y(coordinates.y, 4))),
    ) {
        (x, Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black), Tile::Color(Color::Black)) => {
            if x == Color::Black {
                1
            } else {
                -1
            }
        },
        (x, Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White), Tile::Color(Color::White)) => {
            if x == Color::Black {
                -1
            } else {
                1
            }   
        },
        _ => 0
    }
}