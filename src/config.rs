use crate::leakser::leakser;
use std::{env, process};
use crate::error::FlagError;

#[derive(Debug)]
pub struct Config {
    pub board_length: usize,
    pub alignement_nb: usize,
    pub capture_range: usize,
    pub capture_nb: usize,
    pub visual: bool,
    pub total_tiles: usize,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut args: Vec<String> = env::args().collect();
        println!("CREATING CONFIG");
        match leakser(&mut args[1..]) {
            Ok((m, c, r, a, v)) => Config {
                board_length: m,
                alignement_nb: a,
                capture_range: r,
                capture_nb: c,
                visual: v,
                total_tiles: m * m,
            },
            Err(e) => {
                println!("{}", e);
                if e != FlagError::PrintHelper || e != FlagError::PrintRules {
                    println!("for more information use \"cargo run -- --help\"");
                }
                process::exit(1);
            }
        }
    };
}