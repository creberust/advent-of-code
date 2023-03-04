use std::{fmt::Display, path::Path};

/// The Advent of Code year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Year(pub u16);

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The day of the puzzle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Day(pub u8);

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The part of the puzzle to solve
pub enum Part {
    /// Only the first part
    One,
    /// Only the second part
    Two,
    /// Both of the parts
    Both,
}

/// Abstraction for an AoC event (year)
pub trait Event {
    /// The year of the event
    fn year(&self) -> Year;

    /// Solve the given day for an AoC event
    ///
    /// # Parameters
    /// * `day` - The day of the event to solve
    /// * `part` - The part(s) of the puzzle to solve
    fn solve(&self, day: Day, input: &Path, part: Part);
}

impl Display for dyn Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Advent of Code {}", self.year())
    }
}

/// Abstraction for an AoC puzzle
pub trait Puzzle {
    /// The name of the puzzle
    fn name(&self) -> String;

    /// The day of the puzzle
    fn day(&self) -> Day;

    /// Solve the `part` of the puzzle
    ///
    /// # Parameters
    /// * `part` - The part(s) of the puzzle to solve
    fn solve(&self, input: &Path, part: Part);
}

impl Display for dyn Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}: {}", self.day(), self.name())
    }
}
