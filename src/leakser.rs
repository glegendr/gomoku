const BOARD_LENGTH: usize = 19;
const CAPTURED_NB: usize = 10;
const CAPTURE_RANGE: usize = 2;
const ALIGNEMENT_NB: usize = 5;
const MINMAX_DEPTH: usize = 5;
pub const DEPTH_SUGGESTION: usize = 4;

pub const BOARD_LENGTH_LIMIT: usize = 50;
pub const CAPTURED_NB_LIMIT: usize = 999;
const CAPTURE_RANGE_LIMIT: usize = BOARD_LENGTH_LIMIT - 2;
pub const MINMAX_DEPTH_LIMIT: usize = 10;

const MORPION_S: usize = 3;
const MORPION_C: usize = 1;
const MORPION_R: usize = 0;
const MORPION_A: usize = 3;

const TENTEN_S: usize = 10;
const TENTEN_C: usize = 10;
const TENTEN_R: usize = 2;
const TENTEN_A: usize = 5;

use crate::error::{FlagError};
use crate::players::*;
use crate::color::{Color};

struct MapFlag {
    lst_flag: Vec<String>,
    size: usize,
    captured_nb: usize,
    range: usize,
    alignement_nb: usize,
    depth: usize
}

struct OnOffFlag {
    lst_flag: Vec<String>,
    visual: bool,
    special_rule: bool,
    morpion_rule: bool,
    tenten_rule: bool,
    suggestion: bool
}

struct PlayerFlag {
    lst_flag: Vec<String>,
    player1: Player,
    player2: Player,
}

impl MapFlag {
    fn new() -> MapFlag {
        MapFlag {
            lst_flag: vec![
                "-s".to_string(), "--size".to_string(),
                "-c".to_string(), "--captured".to_string(),
                "-r".to_string(), "--range".to_string(),
                "-a".to_string(), "--alignement".to_string(),
                "-d".to_string(), "--depth".to_string(),
            ],
            size: BOARD_LENGTH,
            captured_nb: CAPTURED_NB,
            range: CAPTURE_RANGE,
            alignement_nb: ALIGNEMENT_NB,
            depth: MINMAX_DEPTH,
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
            "-d" | "--depth" => self.depth = value,
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
        let d = self.depth;
        if m > BOARD_LENGTH_LIMIT {
            Err(FlagError::MapTooBig)
        } else if c > CAPTURED_NB_LIMIT || c <= 0 {
            Err(FlagError::CapturedTooBig)
        } else if r > CAPTURE_RANGE_LIMIT {
            Err(FlagError::RangeTooBig(CAPTURE_RANGE_LIMIT))
        } else if m < a {
            Err(FlagError::AlignementTooBig(m))
        } else if m < 3 || m < r + 2 {
            Err(FlagError::MapTooSmall)
        }  else if a < 2 {
            Err(FlagError::AlignementTooSmall)
        } else if r >= a {
            Err(FlagError::RangeTooBig(a + 1))
        } else if d > MINMAX_DEPTH_LIMIT || d <= 0 {
            Err(FlagError::IncorectDepth)
        } else {
            Ok(())
        }
    }
}

impl OnOffFlag {
    fn new() -> OnOffFlag {
        OnOffFlag {
            lst_flag: vec![
                "-v".to_string(), "--visual".to_string(),
                "--morpion".to_string(), "--MORPION".to_string(),
                "--tenten".to_string(), "--TENTEN".to_string(),
                "--suggestion".to_string()
            ],
            visual: false,
            special_rule: false,
            morpion_rule: false,
            tenten_rule: false,
            suggestion: false
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

    fn get_suggestion_flag(&self) -> bool {
        self.suggestion
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
            "--suggestion" => self.suggestion = !self.get_suggestion_flag(),
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

impl PlayerFlag {
    fn new() -> PlayerFlag {
        PlayerFlag {
            lst_flag: vec![
                "-p1".to_string(), "--player1".to_string(),
                "-p2".to_string(), "--player2".to_string()
            ],
            player1: Player::new(Color::Black, PlayerType::Bot(Algorithm::basic_algorithm())),
            player2: Player::new(Color::White, PlayerType::Human)
        }
    }

    fn get_lst_flag(&self) -> &Vec<String> {
        &self.lst_flag
    }

    fn get_player1(&self) -> Player {
        self.player1
    }

    fn get_player2(&self) -> Player {
        self.player2
    }

    fn assign_player_type(&self, color: Color, value: &str) -> Player {
        match value.to_lowercase().as_str() {
            "human" => Player::new(color, PlayerType::Human),
            "bot" => Player::new(color, PlayerType::Bot(Algorithm::basic_algorithm())),
            "pvs" => Player::new(color, PlayerType::Bot(Algorithm::Pvs)),
            "minimax" => Player::new(color, PlayerType::Bot(Algorithm::Minimax)),
            _ => unreachable!()
        }
    }

    fn get_flag(&mut self, flag: &str, value: &str) {
        match flag {
            "-p1" | "--player1" => self.player1 = self.assign_player_type(Color::Black, value),
            "-p2" | "--player2" => self.player2 = self.assign_player_type(Color::White, value),
            _ => ()
        }
    }

    fn parse(&self, flag: &str) -> bool {
        if self.get_lst_flag().iter().any(|x| *x == flag) {
           return true;
        }
        false
    }

    fn parse_value(&self, value: &str) -> bool {
        match value.to_lowercase().as_str() {
            "human" |
            "pvs" |
            "bot" |
            "minimax" => true,
            _ => false
        }
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
    on_off_flag: OnOffFlag,
    player_flag:PlayerFlag
) -> Result<(usize, usize, usize, usize, bool, Player, Player, usize, bool), (FlagError, usize)> {
    if on_off_flag.get_morpion_rule() == true {
        Ok((
            MORPION_S,
            MORPION_C,
            MORPION_R,
            MORPION_A,
            on_off_flag.get_visual_flag(),
            player_flag.get_player1(),
            player_flag.get_player2(),
            map_flag.depth,
            on_off_flag.get_suggestion_flag()
        ))
    } else if on_off_flag.get_tenten_rule() == true {
        Ok((
            TENTEN_S,
            TENTEN_C,
            TENTEN_R,
            TENTEN_A,
            on_off_flag.get_visual_flag(),
            player_flag.get_player1(),
            player_flag.get_player2(),
            map_flag.depth,
            on_off_flag.get_suggestion_flag()
        ))
    } else {
        Ok((
            map_flag.get_size(),
            map_flag.get_captured_nb(),
            map_flag.get_range(),
            map_flag.get_alignement_nb(),
            on_off_flag.get_visual_flag(),
            player_flag.get_player1(),
            player_flag.get_player2(),
            map_flag.depth,
            on_off_flag.get_suggestion_flag()
        ))
    }
}

pub fn leakser(
    flags: &mut [String]
) -> Result<(usize, usize, usize, usize, bool, Player, Player, usize, bool), (FlagError, usize)> {
    match check_helper(flags) {
        Err(e) => return Err((e, usize::MAX)),
        _ => ()
    }
    match check_rules(flags) {
        Err(e) => return Err((e, usize::MAX)),
        _ => ()
    }

    let mut i = 0;
    let mut map_flag: MapFlag = MapFlag::new();
    let mut on_off_flag: OnOffFlag = OnOffFlag::new();
    let mut player_flag: PlayerFlag = PlayerFlag::new();
    while i < flags.len() {
        if i == 0 && flags[i] == "main.rs" {
            i += 1;
            if i >= flags.len() {
                break;
            }
        }
        if map_flag.parse(flags[i].as_str()) == true {
            if i >= flags.len() - 1 {
                return Err((FlagError::FlagNeedValue, i));
            }
            match flags[i + 1].parse::<usize>() {
                Ok(value) => map_flag.get_flag(flags[i].as_str(), value),
                _ => return Err((FlagError::NoNumberValue, i + 1))
            }
            i += 1;
        } else if on_off_flag.parse(flags[i].as_str()) == true {
            on_off_flag.get_flag(flags[i].as_str())
        } else if player_flag.parse(flags[i].as_str()) == true {
            if i >= flags.len() - 1 {
                return Err((FlagError::FlagNeedValue, i));
            }
            match player_flag.parse_value(flags[i + 1].as_str()) {
                true => player_flag.get_flag(flags[i].as_str(), flags[i + 1].as_str()),
                _ => return Err((FlagError::IncorrectValue, i + 1))
            }
            i += 1;
        } else {
            return Err((FlagError::WrongFlag, i))
        }
        i += 1;
    }
    match map_flag.parse_values() {
        Err(e) => return Err((e, usize::MAX)),
        _ => ()
    }
    assign_values(map_flag, on_off_flag, player_flag)
}

fn print_helper() {
    println!("USAGE: cargo run --release [--] [OPTIONS]\n");
    println!("OPTIONS:");
    println!("\t-s, --size <Value>\t\tsize of gomoku's board");
    println!("\t-c, --captured <Value>\t\tnumber of stones to capture to win");
    println!("\t-r, --range <Value>\t\trange used for capture opponent's stones");
    println!("\t-a, --alignement <Value>\tnumber of stones to align for win");
    println!("\t-d, --depth\t\t\tset minimax depth value");
    println!("\t-p1 --player1 <Player>\t\tchange Player type (human/bot/pvs/minimax)");
    println!("\t-p2 --player2 <Player>\t\tchange Player type (human/bot/pvs/minimax)");
    println!("\t    --suggestion\t\tprint move suggestion for human player");
    println!("\t-v, --visual\t\t\toutput is a graphical window");
    println!("\t    --morpion\t\t\tset value for a morpion game");
    println!("\t    --tenten\t\t\tset value with a ten's map");
    println!("\t    --rules\t\t\tdisplay gomoku\'s rules");
    println!("\t-h, --help\t\t\tdisplay help information");
}


fn print_rules() {
    println!("\nThe rules are simple, two players take turns placing stones of their color on an intersection of the board.");
    println!("The game ends when one player manages to align five stones or capture 10 opponent's stones.");
    println!("The game is played on a 19x19 Goban, without limit to the number of stones.\n");
    println!("A player that manages to align five stones only wins if the opponent can not break this alignment by capturing a pair.");
    println!("To capture a pair of your opponent's stones and remove them from the board, flank them with your own stones.");
    println!("Example:\n\t. . . .\n\tX O O A\n\t. . . .\n");
    println!("In this scenario, by playing in A, X captures the O pair and removes the stones from the game.");
    println!("The now-free intersections can be played on as if they were never occupied.\n");
    println!("It is forbidden to play a move that introduces two free-three alignments, which would guarantee a win by alignment.");
    println!("A free-three is an alignement of three stones that, if not immediately blocked, allows for an indefendable alignment of four stones");
    println!("Example:\n\t. . . . .\n\t. X . . .\n\t. . X . .\n\t. . . X .\n\t. . . . .\n");
    println!("A double-three is a move that introduces two simultaneous free-three alignments. This is an indefendable scenario.");
    println!("Example:\n\t. . . . . . . .\n\t. X . . . . . .\n\t. . X . . . . .\n\t. . . . . . . .\n\t. . . . A X X .\n");
    println!("In this scenario, by playing in A, X would introduce a double-three, therefore this is a forbidden move.");
}
