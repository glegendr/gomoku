use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlacementError {
    OutOfBounds,
    NotEmpty,
    DoubleFreeThree
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FlagError {
    WrongFlag,
    ErrorTypo,
    MapTooBig,
    CapturedTooBig,
    RangeTooBig,
    AlignementTooBig,
    AlignementTooSmall,
    MapTooSmall,
    CannotAssignZero,
    PrintRules,
    PrintHelper
}

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlacementError::OutOfBounds =>  write!(f, "Out Of Bounds"),
            PlacementError::NotEmpty => write!(f, "Not Empty"),
            PlacementError::DoubleFreeThree => write!(f, "Double Free Three")
        }
    }
}

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlagError::WrongFlag => write!(f, "Wrong flag"),
            FlagError::ErrorTypo => write!(f, "Error typo"),
            FlagError::MapTooBig => write!(f, "Map is too big"),
            FlagError::CapturedTooBig => write!(f, "Captured is too big"),
            FlagError::RangeTooBig => write!(f, "Range is too big"),
            FlagError::AlignementTooBig => write!(f, "Alignement is too big"),
            FlagError::AlignementTooSmall => write!(f, "Alignement is too small"),
            FlagError::MapTooSmall => write!(f, "Map is too small"),
            FlagError::CannotAssignZero => write!(f, "Cannot assign value at zero"),
            FlagError::PrintRules => write!(f, ""),
            FlagError::PrintHelper => write!(f, "")
        }
    }
}
