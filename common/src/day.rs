use std::fmt::Display;

/// The day of the puzzle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Day(pub u8);

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
