//! Implementation for a Part of a Puzzle in Advent of Code.

use std::fmt::Display;

/// The part of the puzzle to solve.
#[derive(Debug)]
pub enum Part {
    /// Only the first part.
    One,
    /// Only the second part.
    Two,
    /// Both of the parts.
    Both,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "one"),
            Part::Two => write!(f, "two"),
            Part::Both => write!(f, "both"),
        }
    }
}
