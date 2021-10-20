use crate::color::Color;

const CAPTURED_NB: usize = 10;
pub const CAPTURE_RANGE: usize = 1;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerType {
    Bot,
    Human
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Player {
    color: Color,
    player_type: PlayerType,
    captured: usize,
}

impl Player {
    pub fn new(color: Color, player_type: PlayerType) -> Player {
        Player {color, player_type, captured: 0}
    }

    pub fn get_player_type(&self) -> PlayerType {
        self.player_type
    }

    pub fn get_player_color(&self) -> Color {
        self.color
    }

    pub fn get_player_captured(&self) -> usize {
        self.captured
    }

    pub fn add_capture(&mut self) {
        self.captured += CAPTURE_RANGE;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Players {
    player1: Player,
    player2: Player,
    current_player: Player
}

impl Players {

    pub fn new(player1: Player, player2: Player) -> Players {
        Players {player1, player2, current_player: player1}
    }

    pub fn next_player(&mut self) -> () {
        match self.current_player.get_player_color() == self.player1.get_player_color() {
            true => self.current_player = self.player2.clone(),
            _ => self.current_player = self.player1.clone()
        }
    }

    pub fn is_finished(&self) -> (bool, Option<Color>) {
        match (self.player1.get_player_captured(), self.player2.get_player_captured()) {
            (CAPTURED_NB.., _) => (true, Some(self.player1.color)),
            (_, CAPTURED_NB..) => (true, Some(self.player2.color)),
            _ => (false, None)
        }
    }

    pub fn _get_player(&self, color: Color) -> Player {
        match color {
            Color::Black => self.player1,
            _ => self.player2
        }
    }

    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    pub fn add_capture(&mut self, color: Color) {
        match color {
            Color::Black => self.player1.add_capture(),
            _ => self.player2.add_capture()
        }
    }
}