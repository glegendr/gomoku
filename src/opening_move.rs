use board::{Input, Board, Tile};
use players::*;
use color::{Color};

pub fn opening_move(board: &Board, players: &Players, turn_count: usize) -> Option<Input> {
    let middle_input: Input = (board.get_size() / 2, board.get_size() / 2);
    println!("{}", verify_input(board, middle_input, 0, 1, Color::Black));
    match turn_count {
        1 => Some(middle_input),
        2 => opening_second_move(board, middle_input),
        3 => opening_third_move(board, middle_input),
        _ => None
    }
}

fn add_input(middle_input: Input, x: i32, y: i32) -> Option<Input> {
    Some(((middle_input.0 as i32 + x) as usize, (middle_input.1 as i32 + y) as usize))
}

fn verify_input(
    board: &Board,
    middle_input: Input,
    x: i32,
    y: i32,
    color: Color
) -> bool {
    if board.get(((middle_input.0 as i32 + x) as usize, (middle_input.1 as i32 + y) as usize)) == Tile::Color(color.get_inverse_color()) {
            return true
        }
        false
}

fn opening_second_move(board: &Board, middle_input: Input) -> Option<Input> {
    if board.get(middle_input) == Tile::Color(Color::Black) {
        return add_input(middle_input, 0, 1);
    }
    None
}

fn opening_third_move(board: &Board, middle_input: Input) -> Option<Input> {
    if board.get(middle_input) == Tile::Color(Color::Black) {
        match third_move_rep_1(board, middle_input, Color::Black) {
            (true, Some(input)) => return Some(input),
            (_, _) => (),
        };
        match third_move_rep_2(board, middle_input, Color::Black) {
            (true, Some(input)) => return Some(input),
            (_, _) => (),
        };
        match third_move_rep_3(board, middle_input, Color::Black) {
            (true, Some(input)) => return Some(input),
            (_, _) => (),
        };
        match third_move_rep_4(board, middle_input, Color::Black) {
            (true, Some(input)) => return Some(input),
            (_, _) => (),
        };
    }
    let mut adv_move:(usize, usize) = (0, 0);
    let mut ret:(i32, i32) = (0, 0);
    for (i, x) in board.get_board().iter().enumerate() {
        if *x == Tile::Color(Color::White) {
            adv_move = board.get_input(i);
            break;
        }
    }
    if (middle_input.0 as i32) - (adv_move.0 as i32) < 0 {
        ret.0 = -1;
    } else if (middle_input.0 as i32) - (adv_move.0 as i32) >= 0 {
        ret.0 = 1;
    } else {
        return None
    }
    if (middle_input.1 as i32) - (adv_move.1 as i32) < 0 {
        ret.1 = 1;
    } else if (middle_input.1 as i32) - (adv_move.1 as i32) >= 0 {
        ret.1 = -1;
    } else {
        return None
    }
    add_input(middle_input, ret.0, ret.1)
}

fn third_move_rep_1(
    board: &Board,
    middle_input: Input,
    color: Color
) -> (bool, Option<Input>) {
    if verify_input(board, middle_input, 0, 1, color) == true {
        return (true, add_input(middle_input, -1, 1));
    }
    if verify_input(board, middle_input, -1, 0, color) == true {
        return (true, add_input(middle_input, -1, -1));
    }
    if verify_input(board, middle_input, 0, -1, color) == true {
        return (true, add_input(middle_input, 1, -1));
    }
    if verify_input(board, middle_input, 1, 0, color) == true {
        return (true, add_input(middle_input, 1, 1));
    }
    (false, None)
}

fn third_move_rep_2(
    board: &Board,
    middle_input: Input,
    color: Color
) -> (bool, Option<Input>) {
    if verify_input(board, middle_input, -1, 1, color) == true {
        return (true, add_input(middle_input, 1, 0));
    }
    if verify_input(board, middle_input, -1, 1, color) == true {
        return (true, add_input(middle_input, 0, 1));
    }
    if verify_input(board, middle_input, 1, -1, color) == true {
        return (true, add_input(middle_input, -1, 0));
    }
    if verify_input(board, middle_input, 1, 1, color) == true {
        return (true, add_input(middle_input, 0, -1));
    }
    (false, None)
}

fn third_move_rep_3(
    board: &Board,
    middle_input: Input,
    color: Color
) -> (bool, Option<Input>) {
    if verify_input(board, middle_input, 0, 2, color) == true {
        return (true, add_input(middle_input, -1, -1));
    }
    if verify_input(board, middle_input, -2, 0, color) == true {
        return (true, add_input(middle_input, 1, -1));
    }
    if verify_input(board, middle_input, 0, -2, color) == true {
        return (true, add_input(middle_input, 1, 1));
    }
    if verify_input(board, middle_input, 2, 0, color) == true {
        return (true, add_input(middle_input, -1, 1));
    }
    (false, None)
}

fn third_move_rep_4(
    board: &Board,
    middle_input: Input,
    color: Color
) -> (bool, Option<Input>) {
    if verify_input(board, middle_input, -1, 2, color) == true {
        return (true, add_input(middle_input, 1, 1));
    }
    if verify_input(board, middle_input, -1, -2, color) == true {
        return (true, add_input(middle_input, 1, -1));
    }
    if verify_input(board, middle_input, 1, -2, color) == true {
        return (true, add_input(middle_input, -1, -1));
    }
    if verify_input(board, middle_input, 1, 2, color) == true {
        return (true, add_input(middle_input, -1, 1));
    }
    (false, None)
}
