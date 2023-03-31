use std::{fmt::Display, hash::Hash};

use crate::*;

/// The puzzle of the given day for a given event.
#[derive(Debug, Eq)]
pub struct Puzzle<S: Fn(&Input)> {
    day: Day,
    name: String,
    part_1: S,
    part_2: S,
}

impl<S: Fn(&Input)> Puzzle<S> {
    /// Create a new puzzle
    pub fn new(day: Day, name: String, part_1: S, part_2: S) -> Self {
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

    /// Solve the `part` of the puzzle
    ///
    /// # Parameters
    /// * `input` - The input given to the puzzle
    /// * `part` - The part(s) of the puzzle to solve
    pub fn solve(&self, input: &Input, part: Part) {
        match part {
            Part::One => {
                println!("    |---Part One");
                (self.part_1)(input);
            }
            Part::Two => {
                println!("    \\---Part Two");
                (self.part_2)(input);
            }
            Part::Both => {
                self.solve(input, Part::One);
                self.solve(input, Part::Two);
            }
        }
    }
}

impl<S: Fn(&Input)> Display for Puzzle<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}: {}", self.day(), self.name())
    }
}

impl<S: Fn(&Input)> PartialEq for Puzzle<S> {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
    }
}

impl<S: Fn(&Input)> Hash for Puzzle<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.day.hash(state)
    }
}
