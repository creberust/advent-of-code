use std::fmt::Display;

/// The Advent of Code year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Year(pub u16);

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
