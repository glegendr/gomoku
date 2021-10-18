use std::{fmt, io};

#[derive(PartialEq, Clone, Copy, Debug)]
enum PlacementError {
    OutOfBounds,
    NotEmpty,
    DoubleFreeThree
}


#[derive(PartialEq, Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black =>  write!(f, "X"),
            Color::White => write!(f, "O")
        }
    }
}

#[derive(PartialEq, Clone)]
enum PlayerType {
    Bot,
    Human
}

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
struct Board {
    board: Vec<Tile>
}

impl Board {
    fn new(vec: Vec<Tile>) -> Board {
        Board {board: vec}
    }

    fn replace(&mut self, input: (usize, usize), tile: Tile) {
        self.board[input.0 + input.1 * 19] = tile;
    }

    fn get(&self, input: (usize, usize)) -> Tile{
        self.board[input.0 + input.1 * 19]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let my_str: String = self.board.iter().enumerate().fold("  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5  6  7  8\n0 ".to_string(), |acc, (i, x)| {
            match ((i + 1) % 19, i) {
                (_, 360) => format!("{}{}", acc, x),
                (0, _) => format!("{}{}\n{} ", acc, x, ((i / 19) + 1) % 10),
                _ => format!("{}{}  ", acc, x)
            }
        });
        write!(f, "{}", my_str)
    }
}


#[derive(PartialEq, Clone)]
struct Player {
    color: Color,
    player_type: PlayerType
}

struct Players {
    player1: Player,
    player2: Player,
    current_player: Player
}

impl Players {
    fn next_player(&mut self) -> () {
        match self.current_player == self.player1 {
            true => self.current_player = self.player2.clone(),
            _ => self.current_player = self.player1.clone()
        }
    }
}

fn get_free_three(board: &mut Board, input: (usize, usize), color: Color) -> i32 {
    let lst = [
        case1(board, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
        case1(board, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
        case1(board, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
        case1(board, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
        case1(board, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
        case1(board, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
        case1(board, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
        case1(board, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),
        case2(board, input, color, |x, y| (x as i32 - y) as usize, |x, _| x),
        case2(board, input, color, |x, y| (x as i32 + y) as usize, |x, _| x),
        case2(board, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 - y) as usize),
        case2(board, input, color, |x, y| (x as i32 - y) as usize, |x, y| (x as i32 + y) as usize),
        case2(board, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 - y) as usize),
        case2(board, input, color, |x, y| (x as i32 + y) as usize, |x, y| (x as i32 + y) as usize),
        case2(board, input, color, |x, _|  x, |x, y| (x as i32 - y) as usize),
        case2(board, input, color, |x, _| x, |x, y| (x as i32 + y) as usize),
    ];
    lst.iter().filter(|x| **x == true).count() as i32
}

fn case1 (board: &mut Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 3) > 18 || f_x(input.0, -1) > 18 || f_y(input.1, 3) > 18 || f_y(input.1, -1) > 18 {
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

fn case2 (board: &mut Board, input: (usize, usize), color: Color, f_x: fn(usize, i32) -> usize, f_y: fn(usize, i32) -> usize) -> bool {
    if f_x(input.0, 2) > 18 || f_x(input.0, -2) > 18 || f_y(input.1, 2) > 18 || f_y(input.1, -2) > 18 {
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

fn check_double_free_three(board: &mut Board, input: (usize, usize), color: Color) -> bool {
    get_free_three(board, input, color) > 0
}

fn add_value(board: &mut Board, input: (usize, usize), color: Color) -> Result<(), PlacementError> {
    if input.0 > 18 || input.1 > 18 {
        return Err(PlacementError::OutOfBounds)
    } else if board.get(input) != Tile::Empty {
        return Err(PlacementError::NotEmpty)
    } else if check_double_free_three(board, input, color) {
        return Err(PlacementError::DoubleFreeThree)
    }
    board.replace(input, Tile::Color(color));
    Ok(())
}

fn get_human_input(player_color: Color) -> (usize, usize) {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let vec: Vec<i32> = guess.trim().split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (vec[0] as usize, vec[1] as usize)
}

fn main() {
    let mut board: Board = Board::new(vec![Tile::Empty; 361]);
    let player1 =  Player{color: Color::Black, player_type: PlayerType::Human};
    let player2 =  Player{color: Color::White, player_type: PlayerType::Human};
    let mut players = Players{player1: player1.clone(), player2, current_player: player1};
    let mut i = 0;
    loop {
        let input = match players.current_player.player_type {
            PlayerType::Human => get_human_input(players.current_player.color),
            PlayerType::Bot => (i*14, i*2)
        };
        match add_value(&mut board, input, players.current_player.color) {
            Ok(_) => players.next_player(),
            Err(e) => println!("{:?}", e)
        };
        println!("{}", board);
        i += 1;
    }
}