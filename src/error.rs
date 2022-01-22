use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlacementError {
    OutOfBounds,
    NotEmpty,
    IncorrectPlacement,
    DoubleFreeThree
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FlagError {
    WrongFlag,
    MapTooBig,
    CapturedTooBig,
    RangeTooBig,
    AlignementTooBig,
    AlignementTooSmall,
    MapTooSmall,
    FlagNeedValue,
    NoNumberValue,
    IncorrectValue,
    PrintRules,
    PrintHelper,
}

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlacementError::OutOfBounds =>  write!(f, "Out Of Bounds"),
            PlacementError::NotEmpty => write!(f, "Not Empty"),
            PlacementError::IncorrectPlacement => write!(f, "Incorrect placement"),
            PlacementError::DoubleFreeThree => write!(f, "Double Free Three")
        }
    }
}

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlagError::WrongFlag => write!(f, "This flag doesn't exist"),
            FlagError::MapTooBig => write!(f, "Your map is too big"),
            FlagError::CapturedTooBig => write!(f, "Your captured number is too big"),
            FlagError::RangeTooBig => write!(f, "Your range is too big"),
            FlagError::AlignementTooBig => write!(f, "Your alignement number is too big"),
            FlagError::AlignementTooSmall => write!(f, "Your alignement number is too small"),
            FlagError::MapTooSmall => write!(f, "Your map is too small"),
            FlagError::FlagNeedValue => write!(f, "This flag need a value"),
            FlagError::NoNumberValue => write!(f, "This is not a number"),
            FlagError::IncorrectValue => write!(f, "That's an incorrect value"),
            FlagError::PrintRules => write!(f, ""),
            FlagError::PrintHelper => write!(f, "")
        }
    }
}
