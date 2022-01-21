const BOARD_LENGTH: usize = 19;
const CAPTURED_NB: usize = 10;
const CAPTURE_RANGE: usize = 2;
const ALIGNEMENT_NB: usize = 5;
const DEFAULT_BOT: usize = 0;

use crate::error::{FlagError};
use crate::parser::{check_flags, check_args, check_numbers};

fn get_map_flag(flags: &[String], f1: &str, f2: &str, ret: usize) -> usize {
    for (i, x) in flags.iter().enumerate() {
        match x.parse::<usize>() {
            Ok(y) => {
                match flags[i - 1] == f1 || flags[i - 1] == f2 {
                    true => return y,
                    _ => ()
                }
            },
            Err(_) => {
                let flag_split: Vec<&str> = x.split('=').collect();
                if flag_split.len() == 2 {
                    if flag_split[0] == f1 || flag_split[0] == f2 {
                        return flag_split[1].parse::<usize>().unwrap();
                    }
                }
            }
        }
    };
    ret
}


fn get_v_flag(flags: &mut [String]) -> bool {
    for f in flags.iter() {
        if f == "-v" || f == "--visual" {
            return true
        }
   }
   false
}


fn check_helper(flags: &mut [String]) -> Result<(), FlagError> {
    for flag in flags.iter() {
        if flag == "-h" || flag == "--help" {
            print_helper();
            return Err(FlagError::PrintHelper);
        }
    }
    Ok(())
}


fn check_rules(flags: &mut [String]) -> Result<(), FlagError> {
    for flag in flags.iter() {
        if flag == "--rules" {
            print_rules();
            return Err(FlagError::PrintRules);
        }
    }
    Ok(())
}


pub fn leakser(mut flags: &mut [String]) -> Result<(usize, usize, usize, usize, bool, usize), FlagError> {
    if flags.len() > 0 { 
        if flags[0] == "main.rs" {
            flags = &mut flags[1..];
        }
        match check_helper(flags) {
            Err(e) => return Err(e),
            _ => ()
        }
        match check_rules(flags) {
            Err(e) => return Err(e),
            _ => ()
        }
        if !check_args(flags) {
            return Err(FlagError::ErrorTypo)
        }
        if !check_flags(flags) {
            return Err(FlagError::WrongFlag)
        }
    }
    let visual: bool = get_v_flag(flags);
    let board_length = get_map_flag(flags, "-s", "--size", BOARD_LENGTH);
    let captured_nb = get_map_flag(flags, "-c", "--captured", CAPTURED_NB);
    let capture_range = get_map_flag(flags, "-r", "--range", CAPTURE_RANGE);
    let alignement_nb = get_map_flag(flags, "-a", "--alignement", ALIGNEMENT_NB);
    let bot_type = get_map_flag(flags, "-b", "--bot", DEFAULT_BOT);
    match check_numbers(
        board_length,
        captured_nb,
        capture_range,
        alignement_nb,
        bot_type
    ) {
        Err(e) => Err(e),
        _ => Ok((board_length, captured_nb, capture_range, alignement_nb, visual, bot_type))
    }
}


fn print_helper() {
    println!("USAGE: cargo run -- [OPTION] [VALUE]\n");
    println!("VALUE is a positif real number. For more information check\nrules with \"cargo run -- --rules\"\n");
    println!("OPTIONS:");
    println!("\t-s, --size\t\tsize of gomoku\'s board");
    println!("\t-c, --captured\t\tnumber of stones to capture to win");
    println!("\t-r, --range\t\trange used for capture opponent\'s stones");
    println!("\t-a, --alignement\tnumber of stones to align for win");
    println!("\t    --rules\t\tdisplay gomoku\'s rules");
    println!("\t-h, --help\t\tdisplay help information");
    println!("\t-b, --bot\t\tchange bot's algorithm\n\t\t\t\t\t-> 0: PVS\n\t\t\t\t\t-> 1: Minimax")
}


fn print_rules() {
    println!("\n\twhat a beautiful rules");
}
