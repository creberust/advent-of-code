//! Implementation for a Puzzle in Advent of Code.

use std::{fmt::Display, hash::Hash};

use crate::*;

/// The puzzle of the given day for a given event.
pub struct Puzzle {
    /// The [Day] of the Puzzle.
    day: Day,
    /// The name of the Puzzle.
    name: String,

    /// Solution for the first part.
    part_1: Solver,
    /// Solution for the second part.
    part_2: Solver,
}

impl Puzzle {
    /// Create a new puzzle
    pub fn new(day: Day, name: String, part_1: Solver, part_2: Solver) -> Self {
        Self {
            day,
            name,
            part_1,
            part_2,
        }
    }

    /// The name of the puzzle
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// The day of the puzzle
    pub fn day(&self) -> Day {
        self.day
    }

    /// Solve the first part of the puzzle with the given `input` and return the result.
    pub fn solve_one(&self, input: &Input) -> i64 {
        self.part_1.solve(input)
    }

    /// Solve the second part of the puzzle with the given `input` and return the result.
    pub fn solve_two(&self, input: &Input) -> i64 {
        self.part_2.solve(input)
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}: {}", self.day(), self.name())
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
    }
}
impl Eq for Puzzle {}

impl Hash for Puzzle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.day.hash(state)
    }
}
