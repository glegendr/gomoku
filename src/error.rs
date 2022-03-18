use std::fmt;
use crate::leakser::{
    BOARD_LENGTH_LIMIT,
    CAPTURED_NB_LIMIT,
    MINMAX_DEPTH_LIMIT
};

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
    RangeTooBig(usize),
    AlignementTooBig(usize),
    AlignementTooSmall,
    MapTooSmall,
    FlagNeedValue,
    NoNumberValue,
    IncorrectValue,
    PrintRules,
    PrintHelper,
    IncorectDepth
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
            FlagError::MapTooBig => write!(f, "Size must be countained between 3 and {}", BOARD_LENGTH_LIMIT),
            FlagError::CapturedTooBig => write!(f, "Capture number must be countained between 1 and {}", CAPTURED_NB_LIMIT),
            FlagError::RangeTooBig(max) => write!(f, "Range must be countained between 0 and {}", max),
            FlagError::AlignementTooBig(max) => write!(f, "Alignement must be countained between 2 and {}", max),
            FlagError::AlignementTooSmall => write!(f, "Your alignement number is too small"),
            FlagError::MapTooSmall => write!(f, "Your map is too small"),
            FlagError::FlagNeedValue => write!(f, "This flag need a value"),
            FlagError::NoNumberValue => write!(f, "This is not a number"),
            FlagError::IncorrectValue => write!(f, "That's an incorrect value"),
            FlagError::PrintRules => write!(f, ""),
            FlagError::PrintHelper => write!(f, ""),
            FlagError::IncorectDepth => write!(f, "Depth must be countained between 1 and {}", MINMAX_DEPTH_LIMIT)
        }
    }
}
