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
    MapTooSmall,
    CannotAssignZero,
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
