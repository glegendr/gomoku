use crate::color::Color;
use crate::config::CONFIG;
use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum PlayerType {
    Bot,
    Human
}

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub struct Player {
    color: Color,
    player_type: PlayerType,
    captured: usize,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -- {}", self.color, self.captured)
    }
}

impl Player {
    pub fn new(color: Color, player_type: PlayerType) -> Player {
        Player {color, player_type, captured: 0}
    }

    pub fn reset(&mut self) {
        self.captured = 0;
    }

    fn change_player_type(&mut self) {
        match self.player_type {
            PlayerType::Bot => self.player_type = PlayerType::Human,
            _ => self.player_type = PlayerType::Bot,
        }
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

    pub fn add_capture(&mut self, capture_range: usize) {
        self.captured += capture_range;
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub struct Players {
    player1: Player,
    player2: Player,
    current_player: Player,
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let current_player = self.get_current_player();
        let mut is_player_1 = "".to_owned();
        let mut is_player_2 = "".to_owned();
        match current_player.get_player_color() {
            Color::Black => is_player_1 = "<".to_owned(),
            _ => is_player_2 = "<".to_owned()
        }
        write!(f, "Player 1: {} {}\nPlayer 2: {} {}", self.get_player(Color::Black), is_player_1, self.get_player(Color::White), is_player_2)
    }
}

impl Players {
    pub fn new(player1: Player, player2: Player) -> Players {
        Players {
            player1,
            player2,
            current_player: player1,
        }
    }

    pub fn reset(&mut self) {
        self.player1.reset();
        self.player2.reset();
        self.current_player = self.player1;
    }

    pub fn next_player(&mut self) {
        match self.current_player.get_player_color() == self.player1.get_player_color() {
            true => self.current_player = self.player2.clone(),
            _ => self.current_player = self.player1.clone()
        }
    }

    pub fn is_finished(&self) -> (bool, Option<Color>) {
        if self.player1.get_player_captured() >= CONFIG.capture_nb {
            return (true, Some(self.player1.color));
        } else if self.player2.get_player_captured() >= CONFIG.capture_nb {
            return (true, Some(self.player2.color));
        } else {
            return (false, None)
        }
    }

    pub fn get_player(&self, color: Color) -> Player {
        match color {
            Color::Black => self.player1,
            _ => self.player2
        }
    }

    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    pub fn change_player_type(&mut self, color: Color) {
        match color {
            Color::Black => self.player1.change_player_type(),
            _ => self.player2.change_player_type()
        }
        if self.current_player.get_player_color() == color {
            self.current_player.change_player_type();
        }
    }

    pub fn add_capture(&mut self, color: Color) {
        match color {
            Color::Black => self.player1.add_capture(CONFIG.capture_range),
            _ => self.player2.add_capture(CONFIG.capture_range)
        }
    }
}
