use std::{fmt::Display, path::Path};

use crate::*;

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

    /// Solve all the implemented puzzles
    fn solve_all(&self);
}

impl Display for dyn Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Advent of Code {}", self.year())
    }
}
