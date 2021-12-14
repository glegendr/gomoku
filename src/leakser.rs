const BOARD_LENGTH: usize = 19;
const CAPTURED_NB: usize = 10;
const CAPTURE_RANGE: usize = 2;
const ALIGNEMENT_NB: usize = 5;

const MORPION_S: usize = 3;
const MORPION_C: usize = 1;
const MORPION_R: usize = 0;
const MORPION_A: usize = 3;

const TENTEN_S: usize = 10;
const TENTEN_C: usize = 10;
const TENTEN_R: usize = 2;
const TENTEN_A: usize = 5;

use crate::error::{FlagError};
use crate::parser::{check_flags, check_args, check_numbers};

struct MapFlag {
    lst_flag: Vec<String>,
    size: usize,
    captured_nb: usize,
    range: usize,
    alignement_nb: usize
}

impl MapFlag {
    fn new() -> MapFlag {
        MapFlag {
            lst_flag: vec![
                "-s".to_string(), "--size".to_string(),
                "-c".to_string(), "--captured".to_string(),
                "-r".to_string(), "--range".to_string(),
                "-a".to_string(), "--alignement".to_string()
            ],
            size: BOARD_LENGTH,
            captured_nb: CAPTURED_NB,
            range: CAPTURE_RANGE,
            alignement_nb: ALIGNEMENT_NB
        }
    }

    fn get_lst_flag(&self) -> &Vec<String> {
        &self.lst_flag
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_captured_nb(&self) -> usize {
        self.captured_nb
    }

    fn get_range(&self) -> usize {
        self.range
    }

    fn get_alignement_nb(&self) -> usize {
        self.alignement_nb
    }

    fn get_flag(&mut self, flag: &str, value: usize) {
        match flag {
            "-s" | "--size" => self.size = value,
            "-c" | "--captured" => self.captured_nb = value,
            "-r" | "--range" => self.range = value,
            "-a" | "--alignement" => self.alignement_nb = value,
            _ => ()
        }
    }

    fn parse(&self, flag: &str) -> bool {
        if self.get_lst_flag().iter().any(|x| *x == flag) {
           return true;
        }
        false
    }
}
/*
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

fn morpion_rule(flags: &mut [String]) -> bool {
    for flag in flags.iter() {
        if flag == "--morpion" {
            return true
        }
    }
   false 
}


fn tenten_rule(flags: &mut [String]) -> bool {
    for flag in flags.iter() {
        if flag == "--tenten" {
            return true
        }
    }
   false 
}


fn special_rule(flags: &mut [String]) -> Result<(usize, usize, usize, usize), FlagError> {
    if morpion_rule(flags) == true {
        return Ok((MORPION_S, MORPION_C, MORPION_R, MORPION_A))
    }
    if tenten_rule(flags) == true {
        return Ok((TENTEN_S, TENTEN_C, TENTEN_R, TENTEN_A))
    }
    Err(FlagError::NoSpecialRule)
}

*/
pub fn leakser(mut flags: &mut [String]) -> Result<(usize, usize, usize, usize, bool), FlagError> {
    let mut i = 0;
    let mut map_flag: MapFlag = MapFlag::new();
    while i < flags.len() {
        if map_flag.parse(flags[i].as_str()) == true {
            if i >= flags.len() {
                return Err(FlagError::ErrorTypo);
            }
            match flags[i + 1].parse::<usize>() {
                Ok(value) => map_flag.get_flag(flags[i].as_str(), value),
                _ => return Err(FlagError::ErrorTypo)
            }
            i += 1;
        } else {
            return Err(FlagError::WrongFlag)
        }
        i += 1;
    }
    Ok((
        map_flag.get_size(),
        map_flag.get_captured_nb(),
        map_flag.get_range(),
        map_flag.get_alignement_nb(),
        false
    ))
    /*
    let visual: bool = get_v_flag(flags);
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
        match special_rule(flags) {
            Ok((s, c, r, a)) => return Ok((s, c, r, a, visual)),
            _ => ()
        }
    }
    let board_length = get_map_flag(flags, "-s", "--size", BOARD_LENGTH);
    let captured_nb = get_map_flag(flags, "-c", "--captured", CAPTURED_NB);
    let capture_range = get_map_flag(flags, "-r", "--range", CAPTURE_RANGE);
    let alignement_nb = get_map_flag(flags, "-a", "--alignement", ALIGNEMENT_NB);
    match check_numbers(
        board_length,
        captured_nb,
        capture_range,
        alignement_nb
    ) {
        Err(e) => Err(e),
        _ => Ok((board_length, captured_nb, capture_range, alignement_nb, visual))
    }
    */
}


fn print_helper() {
    println!("USAGE: cargo run --release -- [OPTION]\n");
    println!("OPTIONS:");
    println!("\t-s, --size <Value>\t\tsize of gomoku\'s board");
    println!("\t-c, --captured <Value>\t\tnumber of stones to capture to win");
    println!("\t-r, --range <Value>\t\trange used for capture opponent\'s stones");
    println!("\t-a, --alignement <Value>\tnumber of stones to align for win");
    println!("\t-v, --visual\t\t\tOutput is a graphical window");
    println!("\t    --morpion\t\t\tSet value for a morpion game");
    println!("\t    --tenten\t\t\tSet value with a ten\'s map");
    println!("\t    --rules\t\t\tdisplay gomoku\'s rules");
    println!("\t-h, --help\t\t\tdisplay help information");
}


fn print_rules() {
    println!("\n\twhat a beautiful rules");
}
