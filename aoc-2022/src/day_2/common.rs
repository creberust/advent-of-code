pub const LOSE: u32 = 0;
pub const DRAW: u32 = 3;
pub const WIN: u32 = 6;

pub const ROCK: u32 = 1;
pub const PAPER: u32 = 2;
pub const SCISSORS: u32 = 3;

/// Opponent
/// A => Rock
/// B => Paper
/// C => Scissors
pub enum Opponent {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Opponent {
    fn from(c: char) -> Self {
        match c {
            'A' => Opponent::Rock,
            'B' => Opponent::Paper,
            'C' => Opponent::Scissors,
            _ => panic!("Opponent input is invalid"),
        }
    }
}

/// Player
/// Y => Paper
/// X => Rock
/// Z => Scissors

pub enum Player {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Player {
    fn from(c: char) -> Self {
        match c {
            'X' => Player::Rock,
            'Y' => Player::Paper,
            'Z' => Player::Scissors,
            _ => panic!("Opponent input is invalid"),
        }
    }
}

/// Player
/// Y => DRAW
/// X => LOSE
/// Z => WIN

pub enum RoundStatus {
    LOSE,
    DRAW,
    WIN,
}

impl From<char> for RoundStatus {
    fn from(c: char) -> Self {
        match c {
            'X' => RoundStatus::LOSE,
            'Y' => RoundStatus::DRAW,
            'Z' => RoundStatus::WIN,
            _ => panic!("Opponent input is invalid"),
        }
    }
}
