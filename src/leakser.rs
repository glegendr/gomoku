const BOARD_LENGTH: usize = 19;
const CAPTURED_NB: usize = 10;
const CAPTURE_RANGE: usize = 2;
const ALIGNEMENT_NB: usize = 5;

const BOARD_LENGTH_LIMIT: usize = 19;
const CAPTURED_NB_LIMIT: usize = 10;
const CAPTURE_RANGE_LIMIT: usize = 2;
const ALIGNEMENT_NB_LIMIT: usize = 5;

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

struct OnOffFlag {
    lst_flag: Vec<String>,
    visual: bool,
    special_rule: bool,
    morpion_rule: bool,
    tenten_rule: bool
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

    fn parse_values(&self) -> Result<(), FlagError> {
        let m = self.get_size();
        let c = self.get_captured_nb();
        let r = self.get_range();
        let a = self.get_alignement_nb();
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
        if m < 3 || m < a || m < r + 2 {
            return Err(FlagError::MapTooSmall);
        }
        if c == 0 || a == 0 {
            return Err(FlagError::CannotAssignZero);
        }
        if r >= a {
            return Err(FlagError::RangeTooBig);
        }
        Ok(())
    }
}

impl OnOffFlag {
    fn new() -> OnOffFlag {
        OnOffFlag {
            lst_flag: vec![
                "-v".to_string(), "--visual".to_string(),
                "--morpion".to_string(), "--MORPION".to_string(),
                "--tenten".to_string(), "--TENTEN".to_string()
            ],
            visual: false,
            special_rule: false,
            morpion_rule: false,
            tenten_rule: false
        }
    }

    fn get_lst_flag(&self) -> &Vec<String> {
        &self.lst_flag
    }

    fn get_visual_flag(&self) -> bool {
        self.visual
    }

    fn get_special_rule(&self) -> bool {
        self.special_rule
    }

    fn get_morpion_rule(&self) -> bool {
        self.morpion_rule
    }

    fn get_tenten_rule(&self) -> bool {
        self.tenten_rule
    }

    fn assign_special_rule(&mut self) -> bool {
        match self.get_special_rule() {
            true => false,
            _ => {
                self.special_rule = true;
                true
            }
        }
    }

    fn get_flag(&mut self, flag: &str) {
        match flag {
            "-v" | "--visual" => self.visual = !self.get_visual_flag(),
            "--morpion" | "--MORPION" => self.morpion_rule = self.assign_special_rule(),
            "--tenten" | "--TENTEN" => self.tenten_rule = self.assign_special_rule(),
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

fn assign_values(
    map_flag: MapFlag,
    on_off_flag: OnOffFlag
) -> Result<(usize, usize, usize, usize, bool), FlagError> {
    if on_off_flag.get_morpion_rule() == true {
        Ok((
            MORPION_S,
            MORPION_C,
            MORPION_R,
            MORPION_A,
            on_off_flag.get_visual_flag()
        ))
    } else if on_off_flag.get_tenten_rule() == true {
        Ok((
            TENTEN_S,
            TENTEN_C,
            TENTEN_R,
            TENTEN_A,
            on_off_flag.get_visual_flag()
        ))
    } else {
        Ok((
            map_flag.get_size(),
            map_flag.get_captured_nb(),
            map_flag.get_range(),
            map_flag.get_alignement_nb(),
            on_off_flag.get_visual_flag()
        ))
    }
}

pub fn leakser(mut flags: &mut [String]) -> Result<(usize, usize, usize, usize, bool), FlagError> {
    match check_helper(flags) {
        Err(e) => return Err(e),
        _ => ()
    }
    match check_rules(flags) {
        Err(e) => return Err(e),
        _ => ()
    }

    let mut i = 0;
    let mut map_flag: MapFlag = MapFlag::new();
    let mut on_off_flag: OnOffFlag = OnOffFlag::new();
    while i < flags.len() {
        if i == 0 && flags[i] == "main.rs" {
            i += 1;
        }
        if map_flag.parse(flags[i].as_str()) == true {
            if i >= flags.len() - 1 {
                return Err(FlagError::ErrorTypo);
            }
            match flags[i + 1].parse::<usize>() { // faut parser le usize
                Ok(value) => map_flag.get_flag(flags[i].as_str(), value),
                _ => return Err(FlagError::ErrorTypo)
            }
            i += 1;
        } else if on_off_flag.parse(flags[i].as_str()) == true {
            on_off_flag.get_flag(flags[i].as_str())
        } else {
            return Err(FlagError::WrongFlag)
        }
        i += 1;
    }
    match map_flag.parse_values() {
        Err(e) => return Err(e),
        _ => ()
    }
    assign_values(map_flag, on_off_flag)
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
