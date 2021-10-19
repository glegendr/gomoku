use crate::color::Color;

const CAPTURED_NB: usize = 10;

#[derive(PartialEq, Clone)]
pub enum PlayerType {
    Bot,
    Human
}

#[derive(PartialEq, Clone)]
pub struct Player {
    pub color: Color,
    pub player_type: PlayerType,
    captured: usize,
}

impl Player {
    pub fn new(color: Color, player_type: PlayerType) -> Player {
        Player {color, player_type, captured: 0}
    }
}

pub struct Players {
    pub player1: Player,
    pub player2: Player,
    pub current_player: Player
}

impl Players {
    pub fn next_player(&mut self) -> () {
        match self.current_player == self.player1 {
            true => self.current_player = self.player2.clone(),
            _ => self.current_player = self.player1.clone()
        }
    }

    pub fn is_finished(&self) -> (bool, Option<Color>) {
        match (self.player1.captured, self.player2.captured) {
            (CAPTURED_NB, _) => (true, Some(self.player1.color)),
            (_, CAPTURED_NB) => (true, Some(self.player2.color)),
            _ => (false, None)
        }
    }
}