const BOARD_LENGTH: usize = 19;
const CAPTURED_NB: usize = 10;
const CAPTURE_RANGE: usize = 2;
const ALIGNEMENT_NB: usize = 5;

use crate::error::{FlagError};
use crate::parser::{check_flags, check_args, check_numbers};

fn get_flag(flags: &[String], f1: &str, f2: &str, ret: usize) -> usize {
    for f in flags.iter() {
        let flag = f.split('=').collect::<Vec<&str>>();
        if flag[0] == f1 || flag[0] == f2 {
            return flag[1].parse::<usize>().unwrap();
        }
    }
    ret
}


fn check_helper(flags: &mut [String]) -> Result<(), FlagError> {
    for flag in flags.iter() {
        if flag == "-h" || flag == "--help" {
            return Err(FlagError::PrintHelper);
        }
    }
    Ok(())
}


pub fn leakser(mut flags: &mut [String]) -> Result<(usize, usize, usize, usize), FlagError> {
    if flags[0] == "main.rs" {
        flags = &mut flags[1..];
    }
    match check_helper(flags) {
        Err(e) => return Err(e),
        _ => ()
    };
    if !check_args(flags) {
        return Err(FlagError::ErrorTypo)
    }
    if !check_flags(flags) {
        return Err(FlagError::WrongFlag)
    }
    let board_length = get_flag(flags, "-m", "--map", BOARD_LENGTH);
    let captured_nb = get_flag(flags, "-c", "--captured", CAPTURED_NB);
    let capture_range = get_flag(flags, "-r", "--range", CAPTURE_RANGE);
    let alignement_nb = get_flag(flags, "-a", "--alignement", ALIGNEMENT_NB);
    match check_numbers(
        board_length,
        captured_nb,
        capture_range,
        alignement_nb
    ) {
        Err(e) => Err(e),
        _ => Ok((board_length, captured_nb, capture_range, alignement_nb))
    }
}


pub fn print_helper() {
    println!("usage: cargo run main.rs [-m|--map=<value>]");
    println!("\t\t\t [-c|--captured=<value>]");
    println!("\t\t\t [-r|--range=<value>]");
    println!("\t\t\t [-a|--alignement=<value>]");
}
