use crate::error::FlagError;
use crate::leakser::leakser;
use once_cell::sync::OnceCell;
use std::{env, process};

#[derive(Debug)]
pub struct Config {
    pub board_length: usize,
    pub alignement_nb: usize,
    pub capture_range: usize,
    pub capture_nb: usize,
    pub visual: bool,
    pub total_tiles: usize,
}

impl Config {
    pub fn new() -> Config {
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
    }
}

pub const CONFIG_ERROR: &str = &"Config is not initialized";
pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[macro_export]
macro_rules! config {
    () => {
        CONFIG.get().expect(CONFIG_ERROR)
    };
}
