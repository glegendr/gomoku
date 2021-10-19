use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
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