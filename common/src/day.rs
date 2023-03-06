use std::fmt::Display;

/// The day of the puzzle
///
/// The day can only be in the range [[Day::MIN], [Day::MAX]].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Day(pub u8);

impl Day {
    /// The minimum day correspond to the 1st of December
    pub const MIN: u8 = 1;
    /// The maximum day correspond to the 25th of December
    pub const MAX: u8 = 25;
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        if value < Self::MIN || value > Self::MAX {
            panic!("day is not within the range [1, 25]: {}", value);
        }

        Day(value)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
