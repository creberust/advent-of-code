//! Implementation for an Event in Advent of Code.

use std::{collections::HashSet, fmt::Display};

use crate::{Day, Puzzle, Year};

/// An Event containing all the [Puzzle]s for a [Year].
pub struct Event {
    /// The [Year] of the Event.
    year: Year,

    /// The puzzles for this event.
    puzzles: HashSet<Puzzle>,
}

/// Abstraction for an AoC event (year)
impl Event {
    /// Create a new Event for the `year` with its set of `puzzles`.
    pub fn new(year: Year, puzzles: impl Iterator<Item = Puzzle>) -> Self {
        Self {
            year,
            puzzles: puzzles.collect(),
        }
    }

    /// The [Year] of the event.
    pub fn year(&self) -> Year {
        self.year
    }

    /// Get the puzzle for the specified `day`.
    pub fn puzzle(&self, day: Day) -> Option<&Puzzle> {
        self.puzzles.iter().find(|&puzzle| puzzle.day() == day)
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Advent of Code {}", self.year())
    }
}
