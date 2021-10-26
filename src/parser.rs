const BOARD_LENGTH_LIMIT: usize = 19;
const CAPTURED_NB_LIMIT: usize = 10;
const CAPTURE_RANGE_LIMIT: usize = 2;
const ALIGNEMENT_NB_LIMIT: usize = 5;

use crate::error::{FlagError};


pub fn check_args(flags: &[String]) -> bool {
    match flags.iter().map(|x| {
        if x.chars().next() != Some('-') {
            return false
        }
        if x.split('=').collect::<Vec<&str>>().len() != 2 {
            return false
        }
        match x.split('=').collect::<Vec<&str>>()[1].parse::<usize>() {
            Ok(_) => return true,
            _ => return false
        };
    }).find(|x| *x == false) {
        Some(false) => false,
        _ => true
    }
}


pub fn check_flags(flags: &[String]) -> bool {
    let lst_flags: Vec<&str> = vec![
        "-m", "--map",
        "-c", "--captured",
        "-r", "--range",
        "-a", "--alignement"
    ];
    match flags.iter().map(|x| {
        let flag: Vec<&str> = x.split('=').collect();
        if lst_flags.iter().any(|z| *z == flag[0]) {
            return true
        }
        false
    }).find(|x| *x == false) {
        Some(false) => false,
        _ => true
    }
}


pub fn check_numbers(m: usize, c: usize, r: usize, a: usize) -> Result<(), FlagError> {
    if m > BOARD_LENGTH_LIMIT {
        return Err(FlagError::TaMere);
    }
    if c > CAPTURED_NB_LIMIT {
        return Err(FlagError::TaMere);
    }
    if r > CAPTURE_RANGE_LIMIT {
        return Err(FlagError::TaMere);
    }
    if a > ALIGNEMENT_NB_LIMIT {
        return Err(FlagError::TaMere);
    }
    if m < r || m < a {
        return Err(FlagError::TaMere);
    }
    if r > a {
        return Err(FlagError::TaMere);
    }
    Ok(())
}
