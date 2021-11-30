use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug, Eq, Ord, PartialOrd, Hash)]
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

impl Color {
    pub fn get_inverse_color(&self) -> Color {
        match self {
            Color::Black => Color::White,
            _ => Color::Black
        }
    }
}