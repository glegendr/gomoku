const BOARD_LENGTH: usize = 5;
const CAPTURED_NB: usize = 10;
const ALIGNEMENT_NB: usize = 3;

use crate::players::{CAPTURE_RANGE};
use crate::error::{FlagError};
use crate::parser::{check_flags};

//create a generic function
/*fn get_m_flag(flags: &Vec<String>) -> usize {
    for (i, f) in flags.iter().enumerate() {
        if i + 1 >= flags.len() {
            break; //error a mettre
        }
        if f == "-m" || f == "--map" {
            //return parse(flags[i]);
            return flags[i + 1].parse::<usize>().unwrap();
        }
    }
    BOARD_LENGTH
}
*/

pub fn leakser(flags: &[String]) -> Result<(), FlagError> {
    if !check_flags(flags) {
        return Err(FlagError::WrongFlag)
    }
    Ok(())
}
