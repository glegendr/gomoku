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

pub fn leakser(flags: &[String]) -> Result<(), FlagError> {
    if !check_args(flags) {
        return Err(FlagError::ErrorTypo)
    }
    if !check_flags(flags) {
        return Err(FlagError::WrongFlag)
    }
    println!("{}", get_flag(flags, "-m", "--map", BOARD_LENGTH));
    println!("{}", get_flag(flags, "-c", "--captured", CAPTURED_NB));
    println!("{}", get_flag(flags, "-r", "--range", CAPTURE_RANGE));
    println!("{}", get_flag(flags, "-a", "--alignement", ALIGNEMENT_NB));
    return check_numbers(
        get_flag(flags, "-m", "--map", BOARD_LENGTH),
        get_flag(flags, "-c", "--captured", CAPTURED_NB),
        get_flag(flags, "-r", "--range", CAPTURE_RANGE),
        get_flag(flags, "-a", "--alignement", ALIGNEMENT_NB)
    )
}


pub fn print_helper() {
    println!("usage: cargo run main.rs [-m|--map=<value>]");
    println!("\t\t\t [-c|--captured=<value>]");
    println!("\t\t\t [-r|--range=<value>]");
    println!("\t\t\t [-a|--alignement=<value>]");
}
