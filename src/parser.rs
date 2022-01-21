const BOARD_LENGTH_LIMIT: usize = 19;
const CAPTURED_NB_LIMIT: usize = 10;
const CAPTURE_RANGE_LIMIT: usize = 2;
const ALIGNEMENT_NB_LIMIT: usize = 5;

use crate::{
    error::FlagError,
    players::Algorithm
};


pub fn check_args(flags: &[String]) -> bool {
    match flags.iter().map(|x| {
        match x.parse::<usize>() {
            Ok(_) => return true,
            Err(_) => {
                if x.chars().next() != Some('-') {
                    return false;
                }
           }
        };
        if x.split('=').collect::<Vec<&str>>().len() == 2 {
            match x.split('=').collect::<Vec<&str>>()[1].parse::<usize>() {
                Ok(_) => return true,
                _ => return false
            };
        }
        true
    }).find(|x| *x == false) {
        Some(false) => false,
        _ => true
    }
}


fn check_map_flag_exist(flag: &str) -> bool {
    let lst_flags: Vec<&str> = vec![
        "-s", "--size",
        "-c", "--captured",
        "-r", "--range",
        "-a", "--alignement",
        "-v", "--visual",
        "-b", "--bot"
    ];
    if lst_flags.iter().any(|x| *x == flag) {
        return true;
    }
    false
}


fn check_true_false_flag_exist(flag: &str) -> bool {
    let lst_flags: Vec<&str> = vec![
        "-v", "--visual"
    ];
    if lst_flags.iter().any(|x| *x == flag) {
        return true;
    }
    false
}


pub fn check_flags(flags: &[String]) -> bool {
    match flags.iter().enumerate().map(|(i, x)| {
        match x.parse::<usize>() {
            Ok(_) => {
                if i == 0 {
                    return false;
                }
                return check_map_flag_exist(flags[i - 1].as_str());
            },
            Err(_) => {
                if x.split('=').collect::<Vec<&str>>().len() == 2 {
                    return check_map_flag_exist(x.split('=').collect::<Vec<&str>>()[0]);
                } else if check_true_false_flag_exist(x) == false {
                    if i + 1 >= flags.len() {
                        return false
                    }
                    match flags[i + 1].parse::<usize>() {
                        Ok(_) => (),
                        Err(_)=> return false
                    }
                    return check_map_flag_exist(x);
                } else {
                    true
                }
            }
        }
    }).find(|x| *x == false) {
        Some(false) => false,
        _ => true
    }
}


pub fn check_numbers(m: usize, c: usize, r: usize, a: usize, b: usize) -> Result<(), FlagError> {
    if m < 3 {
        return Err(FlagError::MapTooSmall);
    }
    if c == 0 || a == 0 {
        return Err(FlagError::CannotAssignZero);
    }
    if m > BOARD_LENGTH_LIMIT {
        return Err(FlagError::MapTooBig);
    }
    if c > CAPTURED_NB_LIMIT {
        return Err(FlagError::CapturedTooBig);
    }
    if r > CAPTURE_RANGE_LIMIT {
        return Err(FlagError::RangeTooBig);
    }
    if a > ALIGNEMENT_NB_LIMIT {
        return Err(FlagError::AlignementTooBig);
    }
    if a < 4 && m > 3 {
        return Err(FlagError::AlignementTooSmall);
    }
    if m <= r + 1 || m < a {
        return Err(FlagError::MapTooSmall);
    }
    if r >= a {
        return Err(FlagError::RangeTooBig);
    }
    if b > Algorithm::length() {
        return Err(FlagError::AlgorithmNonDefined);
    }
    Ok(())
}
